use std::{path::PathBuf};

use clap::{Parser, ValueEnum};
#[derive(Parser)]
struct CliArgs {
    working_dir: PathBuf,
    #[arg(long, short, value_name = "FILE")]
    config: Option<PathBuf>,
    #[arg(short, long)]
    log_level: LogLevel
}

#[derive(Debug, Clone)]
enum LogLevel {
    Trace,
    Info,
    Debug
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