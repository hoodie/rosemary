#![allow(unused)]
use std::error::Error;
use std::io::Write;

use env_logger::fmt::Color;
use indicatif::HumanDuration;
use itertools::intersperse;

use rosemary::StoredDurations;

fn main() -> Result<(), Box<dyn Error>> {
    let env = env_logger::Env::default();
    //.filter_or("MY_LOG_LEVEL", "trace");

    // env_logger::init_from_env(env);
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            let mut style = buf.style();
            style.set_color(Color::Green).set_bold(true);
            writeln!(
                buf,
                "[{:^9}] {}",
                style.value(record.level()),
                record.args()
            )
        })
        .init();

    let mut full_cmd = std::env::args().skip(1);
    if let Some(cmd) = full_cmd.next() {
        let input_args = full_cmd.collect::<Vec<_>>();

        log::trace!("{cmd:#?} | {input_args:#?}");

        let mut stored_durations = StoredDurations::load().unwrap_or_default();

        log::trace!("running {cmd:?}");

        log::warn!("skipping args");

        let latest_run = if let Some(prior_duration) = stored_durations.read_previous(&cmd) {
            log::trace!("previous run took ~{}", HumanDuration(prior_duration));
            rosemary::run_with_progress(&cmd, input_args, prior_duration)?
        } else {
            log::trace!("no previous runs, running without progress bar");
            rosemary::run_with_spinner(&cmd, input_args)?
        };

        stored_durations.add(latest_run);
        stored_durations.store()?;
    } else {
        log::warn!("no command given");
    }
    Ok(())
}
