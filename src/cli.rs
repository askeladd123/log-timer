use std::{fmt::Display, path::PathBuf};

use clap::{command, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(
    about = "A tool that helps you track time when you work (or play).",
    long_about = "This tool helps you keep track of time. Example usage: \n- `log-timer start washing-dishes`\n- `log-timer stop` when you're done.\nThe program will add an entry with the time you washed dishes to a log file. See `log-timer configure --help` for initial setup of the log file."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Begin timing an activity now.")]
    Start {
        #[arg(help = "Label describing the activity started.")]
        label: Option<String>,

        #[arg(short, long, value_name = "H24:M", help = "Alternative start time.")]
        time: Option<String>,
    },

    #[command(about = "Stop timing an activty, and write it to a log file. ")]
    Stop {
        #[arg(short, long, value_name = "H24:M", help = "Alternative stop time.")]
        time: Option<String>,
    },

    #[command(about = "Stop timing an activity, and forget about it.")]
    Abort,

    #[command(about = "Use this command to for example decide where to log activities.")]
    Configure {
        #[arg(short, long)]
        log_file_path: PathBuf,

        #[arg(short, long, default_value_t=RowFormatter::New)]
        row_formatter: RowFormatter,
    },

    #[command(
        about = "A quick way to see how the program is configured. This is from a file stored somewhere on your machine."
    )]
    GetConfig,
}

#[derive(ValueEnum, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum RowFormatter {
    #[value(help = "Row format: `date, time-start, time-stop, label`.")]
    Old,

    #[value(help = "Row format: `datetime-start, datetime-stop, label`.")]
    New,
}

impl Display for RowFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Old => "old",
                Self::New => "new",
            }
        )
    }
}

impl From<String> for RowFormatter {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "old" => Self::Old,
            "new" => Self::New,
            _ => panic!("Could not convert `{value}` to RowFormatter. "),
        }
    }
}
