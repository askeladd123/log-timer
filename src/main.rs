// TODO: support white space separated values

#![allow(unused)]
use chrono::{DateTime, Local};
use clap::builder::styling::AnsiColor;
use clap::{Arg, ArgAction, Command, Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::{default, fs};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        label: Option<String>,
        time: Option<DateTime<Local>>,
    },
    Stop {
        time: Option<DateTime<Local>>,
    },
    Abort,
    Configure {
        #[arg(short, long)]
        log_file_path: PathBuf,

        #[arg(short, long, default_value_t=RowFormatter::New)]
        row_formatter: RowFormatter,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Activity {
    time_started: DateTime<Local>,
    label: Option<String>,
}

impl Activity {
    fn load(file: &Path) -> Result<Self, Box<dyn Error>> {
        if Path::new(file).exists() {
            let data = fs::read_to_string(file).unwrap();

            let my_struct = serde_json::from_str(&data).unwrap();
            Ok(my_struct)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            )))
        }
    }

    fn save(&self, path: &Path) {
        let json = serde_json::to_string_pretty(self).unwrap();
        let mut file = std::fs::File::create(path).unwrap();
        std::io::Write::write_all(&mut file, json.as_bytes()).unwrap();
    }
}

impl Display for Activity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
enum RowFormatter {
    Old,
    New,
}

impl Display for RowFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Self::Old => "date,time-start,time-stop,label",
                // Self::New => "datetime-start,datetime-stop,label",
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
            _ => panic!("Could not convert {value} to RowFormatter. "),
        }
    }
}

impl RowFormatter {
    fn format(&self, activity: &Activity) -> impl Display {
        let now = Local::now();
        let time_passed = now.signed_duration_since(activity.time_started);
        let hours_passed = time_passed.num_hours();
        let minutes_passed = time_passed.num_minutes() % 60;
        let label = activity.label.clone().unwrap_or("-".into());

        match self {
            Self::New => {
                let start = activity.time_started.format("%Y-%m-%d-%H-%M");
                let finish = now.format("%Y-%m-%d-%H-%M");
                format!("{start},{finish},{hours_passed:02}:{minutes_passed:02},{label}")
            }
            Self::Old => {
                let date = activity.time_started.format("%Y-%m-%d");
                let start = activity.time_started.format("%H:%M");
                let finish = now.format("%H:%M");
                format!("{date},{start},{finish},{hours_passed:02}:{minutes_passed:02},{label}")
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    log_file_path: PathBuf,
    row_formatter: RowFormatter,
}

impl Config {
    fn load_checked(file: &Path) -> Result<Self, Box<dyn Error>> {
        let data = fs::read_to_string(file)?;

        let my_struct: Self = serde_json::from_str(&data)?;

        if !my_struct.log_file_path.exists() {
            return Err("Configured log file does not exist.".into());
        }

        Ok(my_struct)
    }

    fn save(&self, path: &Path) {
        let json = serde_json::to_string_pretty(self).unwrap();
        let mut file = std::fs::File::create(path).unwrap();
        std::io::Write::write_all(&mut file, json.as_bytes()).unwrap();
    }
}

fn append_to_file(filename: &PathBuf, content: &str) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)?;

    writeln!(file, "{}", content)?;
    Ok(())
}

fn main() {
    let warning = "warning".yellow();

    let cli = Cli::parse();

    let data_dir = dirs::data_dir().unwrap().join("log-timer");
    if !data_dir.exists() {
        fs::create_dir(&data_dir).unwrap();
    }
    let config_file_name = Path::new("config.json");
    let config_file_path = data_dir.join(config_file_name);

    if let Some(Commands::Configure {
        ref log_file_path,
        row_formatter,
    }) = cli.command
    {
        if let Ok(v) = log_file_path.canonicalize() {
            match v.extension() {
                Some(ext) if ext.eq_ignore_ascii_case("csv") => Config {
                    log_file_path: v,
                    row_formatter,
                }
                .save(&config_file_path),
                _ => {
                    eprintln!("{warning}: The file provided is not the expected `csv` format: {log_file_path:?}.");
                }
            }
        } else {
            eprintln!("{warning}: The file provided does not exist: {log_file_path:?}.");
        };
        exit(0);
    }

    let config = if let Ok(v) = Config::load_checked(&config_file_path) {
        v
    } else {
        eprintln!("{warning}: Program not configured yet. Some fields are required, use the `--help` flag for more info.");
        exit(0);
    };

    let tmp_file_name = Path::new("tmp.json");
    let tmp_file_path = data_dir.join(tmp_file_name);
    let activity = tmp_file_path
        .exists()
        .then(|| Activity::load(&tmp_file_path).unwrap());

    if let Some(cmd) = cli.command {
        match (activity, cmd) {
            (None, Commands::Start { label, time: None }) => {
                Activity {
                    time_started: Local::now(),
                    label: label.clone(),
                }
                .save(&tmp_file_path);
                match label {
                    Some(v) => println!("Activity started {v}."),
                    None => println!("Activity started."),
                }
            }
            (
                None,
                Commands::Start {
                    label,
                    time: Some(v),
                },
            ) => todo!(),
            (Some(activity), Commands::Stop { time: None }) => {
                let now = Local::now();
                let row = config.row_formatter.format(&activity);
                append_to_file(&config.log_file_path, &row.to_string());
                fs::remove_file(tmp_file_path).unwrap();
                match activity.label {
                    Some(v) => println!("Stopped activity {v}."),
                    None => println!("Stopped activity."),
                }
            }
            (Some(activity), Commands::Stop { time: Some(v) }) => todo!(),
            (Some(activity), Commands::Abort) => {
                fs::remove_file(tmp_file_path).unwrap();
                match activity.label {
                    Some(v) => println!("Aborted activity {v}."),
                    None => println!("Aborted activity."),
                }
            }
            (None, Commands::Stop { .. }) => {
                eprintln!("{warning}: There is no activity being timed, so nothing to stop.")
            }
            (None, Commands::Abort) => {
                eprintln!("{warning}: There is no activity being timed, so nothing to abort.")
            }
            (Some(activity), Commands::Start { label, time }) => {
                eprintln!(
                    "{warning}: There is already an activity being timed. Won't start another one."
                )
            }
            (.., Commands::Configure { .. }) => unreachable!(),
        }
    } else if tmp_file_path.exists() {
        let activity = Activity::load(&tmp_file_path).unwrap();
        println!("Currentliy timing activity {activity:?}.");
    } else {
        println!("No activity is beeing timed.");
    }
}
