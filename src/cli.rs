use std::{fmt::Display, path::PathBuf};

use clap::{command, Args, Parser, Subcommand, ValueEnum};
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

        #[arg(short, long, default_value_t=RowFormatter::V2_1)]
        row_formatter: RowFormatter,
    },

    #[command(
        about = "Has different subcommands for getting information about logs and configuration."
    )]
    Get(GetArgs),
}

#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub struct GetArgs {
    #[command(subcommand)]
    pub command: GetCommands,
}

#[derive(Debug, Subcommand, Copy, Clone)]
pub enum GetCommands {
    #[command(
        about = "Get a sanitized version of the log file. Essentially without comments. Format: csv"
    )]
    Logs,

    #[command(
        about = "Like the command 'logs' but if one day has multiple activities, summarize them. Format: csv"
    )]
    Days,

    #[command(about = "Get stats for today session. Format: ?")]
    Today,

    #[command(about = "Get stats from all sessions. Format: ?")]
    Total,

    #[command(about = "Get contents of config file. Format: json")]
    Config,

    #[command(about = "Get path of config file. Format: string")]
    ConfigPath,
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

// enum Row {
//     V2_1 {
//         start: DateTime<FixedOffset>,
//         stop: DateTime<FixedOffset>,
//         duration: Timelike,
//         label: String,
//     },
//     V2_0 {
//         start: DateTime<FixedOffset>,
//         stop: DateTime<FixedOffset>,
//         duration: Timelike,
//         label: String,
//     },
//     V1_0 {
//         date: NaiveDate,
//         start: Timelike,
//         stop: Timelike,
//         duration: Timelike,
//         label: String,
//     },
// }
