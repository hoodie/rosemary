#![allow(unused_imports)]
use std::{
    collections::HashMap,
    error::Error,
    fmt::Write,
    fs::read_to_string,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use itertools::intersperse;
use serde::{Deserialize, Serialize};

use rosemary::{RecordedRun, StoredDurations};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stored_durations = StoredDurations::load().unwrap_or_default();

    let run = dbg!(RecordedRun {
        command: "yarn test".into(),
        pwd: PathBuf::from("/home/hendrik/code/web-app"),
        duration: Duration::from_millis(12_346),
        timestamp: chrono::Utc::now()
    });

    stored_durations.add(run);

    stored_durations.store()?;

    // dbg!(stored_durations);
    Ok(())
}
