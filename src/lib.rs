use std::{
    error::Error,
    fmt::Write,
    io::{BufRead, BufReader, Read},
    process::{Child, Stdio},
    time::{Duration, Instant},
};

use anyhow::anyhow;
use indicatif::{HumanDuration, ProgressBar, ProgressState, ProgressStyle};

mod storage;

pub use crate::storage::{RecordedRun, StoredDurations};

trait Runner {
    fn run(&self) -> RecordedRun;
}

pub fn spawn_child(
    command: &str,
    args: impl IntoIterator<Item = String>,
) -> Result<Child, Box<dyn Error>> {
    Ok(std::process::Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow!("cannot run command {command} {}", e))?)
}

fn update_progress(
    pb: &ProgressBar,
    reader: impl Read + Send,
    start_time: Instant,
    with_line: impl Fn(&str) + Sync + Send,
    with_elapsed: impl Fn(Duration) + Sync + Send,
) {
    rayon::join(
        || {
            for line in BufReader::new(reader).lines() {
                let line = line.unwrap();
                let elapsed = std::time::Instant::now() - start_time;

                with_line(&line);
                with_elapsed(elapsed);
            }
            pb.finish_and_clear();
        },
        || loop {
            let elapsed = std::time::Instant::now() - start_time;

            if pb.is_finished() {
                break;
            }
            with_elapsed(elapsed);

            std::thread::sleep(Duration::from_millis(200));
        },
    );
}

pub fn run_with_progress(
    command: &str,
    args: impl IntoIterator<Item = String>,
    expected_duration: Duration,
) -> Result<RecordedRun, Box<dyn Error>> {
    let pb = ProgressBar::new(expected_duration.as_millis() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.bold} {eta} [{wide_bar:.green/white.bold}] ~({duration})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        }),
    );

    let start_time = std::time::Instant::now();

    let mut child = spawn_child(command, args)?;

    let reader = child.stdout.take().unwrap();
    update_progress(
        &pb,
        reader,
        start_time,
        |line| pb.println(line),
        |elapsed| pb.set_position(elapsed.as_millis() as u64),
    );
    let duration = std::time::Instant::now() - start_time;

    println!("done after {}", HumanDuration(duration));

    Ok(RecordedRun::here(command.into(), duration)?)
}

pub fn run_with_spinner(
    command: &str,
    args: impl IntoIterator<Item = String>,
) -> Result<RecordedRun, Box<dyn Error>> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.bold} running {wide_msg} {elapsed_precise}")
            .unwrap(),
    );

    let start_time = std::time::Instant::now();

    let mut child = spawn_child(command, args)?;

    let reader = child.stdout.take().unwrap();
    update_progress(
        &pb,
        reader,
        start_time,
        |line| pb.println(line),
        |elapsed| pb.set_message(HumanDuration(elapsed).to_string()),
    );

    let duration = std::time::Instant::now() - start_time;

    println!("done after {:.2} seconds", duration.as_secs_f64());

    Ok(RecordedRun::here(command.into(), duration)?)
}
