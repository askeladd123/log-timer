use std::{fmt::Display, path::PathBuf};

use clap::{command, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(
    about = "A tool that helps you track time when you work (or play).",
    long_about = "This tool helps you keep track of time. Example usage: \n- 'log-timer start washing-dishes'\n- 'log-timer stop' when you're done.\nThe program will add an entry with the time you washed dishes to a log file. See 'log-timer configure --help' for initial setup of the log file.",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Begin timing an activity now.")]
    Start {
        #[arg(help = "Word or sentance describing the activity started.")]
        label: Option<String>,

        #[arg(short, long, value_name = "H24:M", help = "Alternative start time.")]
        time: Option<String>,
    },

    #[command(about = "Stop timing an activity, and write it to a log file. ")]
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

        #[arg(short, long, default_value_t=RowFormatter::V2_0)]
        row_formatter: RowFormatter,
    },

    #[command(
        about = "A quick way to see how the program is configured. This is from a file stored somewhere on your machine."
    )]
    GetConfig,
}

#[derive(ValueEnum, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum RowFormatter {
    #[value(help = "Row format: 'date, time-start, time-stop, label'. ")]
    V1_0,

    #[value(help = "Row format: 'datetime-start, datetime-stop, label'. ")]
    V2_0,

    #[value(
        help = "Row format: 'datetime-start, datetime-stop, label' where datetime is in 'RFC 3339'. "
    )]
    V2_1,
}

impl Display for RowFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::V2_1 => "version 2.1",
                Self::V2_0 => "version 2.0",
                Self::V1_0 => "version 1.0",
            }
        )
    }
}

impl From<String> for RowFormatter {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "v2-1" => Self::V2_1,
            "v2-0" => Self::V2_0,
            "v1-0" => Self::V1_0,
            _ => panic!("Could not convert '{value}' to RowFormatter. "),
        }
    }
}
