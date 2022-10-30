use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, read_to_string},
    path::PathBuf,
    time::Duration,
};

type Command = String;

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum StorageError {
        #[error("config file can not be created")]
        Inaccessible,

        #[error("config file is not in a valid format")]
        Invalid(#[from] serde_json::Error),

        #[error("data store disconnected")]
        Disconnect(#[from] std::io::Error),
    }

    pub type Result<T> = std::result::Result<T, StorageError>;
}

use self::error::{Result, StorageError};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StoredDurations {
    paths: HashMap<PathBuf, RunByCommand>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RunByCommand {
    runs: HashMap<Command, RecordedRun>,
}

impl From<RecordedRun> for RunByCommand {
    fn from(recording: RecordedRun) -> Self {
        let mut runs = HashMap::new();
        runs.insert(recording.command.clone(), recording);
        RunByCommand { runs }
    }
}

impl StoredDurations {
    fn storage_path() -> Result<PathBuf> {
        dirs2::config_dir()
            .filter(|config_path| config_path.exists())
            .ok_or(StorageError::Inaccessible)
            .map(|mut path| {
                path.set_file_name("rosemary.json");
                path
            })
    }

    pub fn load() -> Result<StoredDurations> {
        let config_path = Self::storage_path()?;
        log::trace!("loading prior runs from {}", config_path.display());
        let content = read_to_string(&config_path)?;
        Ok(serde_json::from_str::<StoredDurations>(&content)?)
    }

    pub fn read_previous(&self, command: &str) -> Option<Duration> {
        if let Ok(pwd) = std::env::current_dir() {
            self.paths
                .get(&pwd)
                .and_then(|RunByCommand { runs }| runs.get(command))
                .map(|run| run.duration)
        } else {
            None
        }
    }

    pub fn store(&self) -> Result<()> {
        let stored_runs = serde_json::to_string_pretty(&self)?;
        fs::write(Self::storage_path()?, stored_runs)?;
        Ok(())
    }

    pub fn add(&mut self, recording: RecordedRun) {
        self.paths
            .entry(recording.pwd.clone())
            .and_modify(|by_command| {
                by_command
                    .runs
                    .entry(recording.command.clone())
                    .and_modify(|run| run.duration = recording.duration)
                    .or_insert_with(|| recording.clone());
            })
            .or_insert_with(|| recording.into());
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct RecordedRun {
    pub command: Command,
    pub pwd: PathBuf,
    pub duration: Duration,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl RecordedRun {
    pub fn here(command: Command, duration: Duration) -> Result<Self> {
        Ok(RecordedRun {
            command,
            pwd: std::env::current_dir()?, // TODO: how about env.cwd or env.pwd?
            duration,
            timestamp: chrono::Utc::now(),
        })
    }
}
