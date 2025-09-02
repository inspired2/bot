use std::{path::PathBuf};
use clap::{Parser, ValueEnum};


#[derive(Parser)]
pub struct CliArgs {
    working_dir: Option<PathBuf>,
    #[arg(long, short, value_name = "FILE")]
    config_path: PathBuf,
    #[arg(short, long, default_value = "info")]
    log_level: Option<LogLevel>
}

impl CliArgs {
    pub fn set_cwd(&mut self, path: PathBuf) -> () {
        self.working_dir = Some(path);
    }
    pub fn get_cwd(&self) -> PathBuf {
        assert!(self.working_dir.is_some());
        self.working_dir.as_ref().unwrap().clone()
    }
    pub fn get_config_path(&self) -> PathBuf {
        self.config_path.clone()
    }
}

#[derive(Debug, Clone)]
enum LogLevel {
    Trace,
    Info,
    Debug,
}

impl ValueEnum for LogLevel {
    fn value_variants<'a>() -> &'a [Self] {
        &[LogLevel::Debug, LogLevel::Trace, LogLevel::Info]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            LogLevel::Trace => Some("trace".into()),
            LogLevel::Info => Some("info".into()),
            LogLevel::Debug => Some("debug".into())}
    }
}