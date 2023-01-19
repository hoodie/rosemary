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

use crate::storage::CommandKey;
pub use crate::storage::{RecordedRun, StoredDurations};

trait Runner {
    fn run(&self) -> RecordedRun;
}

#[derive(Clone, Debug)]
pub struct CommandWithArgs {
    pub cmd: String,
    pub args: Vec<String>,
}

impl CommandWithArgs {
    pub fn new<S: Into<String>>(cmd: &str, args: impl IntoIterator<Item = S>) -> Self {
        CommandWithArgs {
            cmd: cmd.into(),
            args: args.into_iter().map(Into::into).collect(),
        }
    }

    pub fn from_env() -> Option<Self> {
        let mut full_cmd = std::env::args().skip(1);
        full_cmd.next().map(|cmd| CommandWithArgs {
            cmd,
            args: full_cmd.map(Into::into).collect(),
        })
    }
}

pub fn spawn_child(call: CommandWithArgs) -> Result<(Child, CommandKey), Box<dyn Error>> {
    let child = std::process::Command::new(&call.cmd)
        .args(&call.args)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow!("cannot run command {} {}", call.cmd, e))?;

    let key: CommandKey = call.into();

    Ok((child, key))
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
    call: CommandWithArgs,
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

    let (mut child, command_key) = spawn_child(call)?;

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

    Ok(RecordedRun::here(command_key, duration)?)
}

pub fn run_with_spinner(call: CommandWithArgs) -> Result<RecordedRun, Box<dyn Error>> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.bold} running {wide_msg} {elapsed_precise}")
            .unwrap(),
    );

    let start_time = std::time::Instant::now();

    let (mut child, command_key) = spawn_child(call)?;

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

    Ok(RecordedRun::here(command_key, duration)?)
}
