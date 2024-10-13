// TODO: support white space separated values
// TODO: consider removing csv column `duration` as it can be derived from `datetime-start` and `datetime-stop`
// TODO: change config file format to 'toml'
// TODO: add options 'get --first' and 'get --last' for filtering
// TODO: add reflexive row formatter that loads correct format
// TODO: change from DateTime<Local> to DateTime<FixedOffset> to support time zones
// TODO: `log-timer get total` should have a flag where you can get HH:MM instead of just minutes
// TODO: add warning when log is empty

#![allow(unused)]
use crate::cli::*;
use chrono::{
    DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, RoundingError,
    SecondsFormat, Utc,
};
use clap::Parser;
use colored::*;
use csv::Writer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::{fs, io};

mod cli;

/// folder name of config and data dirs
const DIR_NAME: &str = "log-timer";

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

impl RowFormatter {
    fn make_row(&self, activity: &Activity, time: DateTime<Local>) -> Vec<String> {
        let time_passed = time.signed_duration_since(activity.time_started);
        let hours_passed = time_passed.num_hours();
        let minutes_passed = time_passed.num_minutes() % 60;
        let duration = format!("{hours_passed:02}:{minutes_passed:02}");
        let label = activity.label.clone().unwrap_or("-".into());

        match self {
            Self::V2_1 => {
                let start = activity
                    .time_started
                    .to_rfc3339_opts(SecondsFormat::Secs, true);
                let finish = time.to_rfc3339_opts(SecondsFormat::Secs, true);
                vec![start, finish, duration, label]
            }
            Self::V2_0 => {
                let start = activity.time_started.format("%Y-%m-%d-%H-%M").to_string();
                let finish = time.format("%Y-%m-%d-%H-%M").to_string();
                vec![start, finish, duration, label]
            }
            Self::V1_0 => {
                let date = activity.time_started.format("%Y-%m-%d").to_string();
                let start = activity.time_started.format("%H:%M").to_string();
                let finish = time.format("%H:%M").to_string();
                vec![date, start, finish, duration, label]
            }
        }
    }

    // fn get_row() -> Row {
    //     unimplemented!()
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    log_file_path: PathBuf,
    row_formatter: RowFormatter,
}

enum ConfigError {
    ConfigNotFound,
    LogFileNotFound { path_tried: PathBuf, config: Config },
}

impl Config {
    fn load_checked(file: &Path) -> Result<Self, ConfigError> {
        let data = if let Ok(v) = fs::read_to_string(file) {
            v
        } else {
            return Err(ConfigError::ConfigNotFound);
        };

        let config: Self = serde_json::from_str(&data).unwrap();

        if config.log_file_path.exists() {
            Ok(config)
        } else {
            Err(ConfigError::LogFileNotFound {
                path_tried: config.log_file_path.clone(),
                config,
            })
        }
    }

    fn save(&self, path: &Path) {
        let json = serde_json::to_string_pretty(self).unwrap();
        let mut file = std::fs::File::create(path).unwrap();
        std::io::Write::write_all(&mut file, json.as_bytes()).unwrap();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_file_path: dirs::data_dir().unwrap().join(DIR_NAME).join("log.csv"),
            row_formatter: RowFormatter::V2_1,
        }
    }
}

fn append_csv(filename: &PathBuf, row: &[String]) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;

    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(row)?;
    wtr.flush()?;

    Ok(())
}

fn parse_time(time_str: &str) -> Result<DateTime<Local>, chrono::format::ParseError> {
    let time = NaiveTime::parse_from_str(time_str, "%H:%M")?;
    let datetime = Local::now()
        .date_naive()
        .and_time(time)
        .and_local_timezone(Local::now().timezone())
        .unwrap();
    Ok(datetime)
}

fn main() -> Result<(), Box<dyn Error>> {
    let warning = "warning".yellow();

    let cli = Cli::parse();

    let conf_dir = dirs::config_dir().unwrap().join(DIR_NAME);
    if !conf_dir.exists() {
        fs::create_dir_all(&conf_dir).unwrap();
    }
    let data_dir = dirs::data_dir().unwrap().join(DIR_NAME);
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).unwrap();
    }
    let config_file_name = Path::new("config.json");
    let config_file_path = conf_dir.join(config_file_name);

    let config = match Config::load_checked(&config_file_path) {
        Ok(v) => v,
        Err(ConfigError::ConfigNotFound) => Config::default(),
        Err(ConfigError::LogFileNotFound { path_tried, config }) => {
            let mut file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&path_tried)?;
            let mut wtr = csv::Writer::from_writer(file);
            wtr.write_record(config.row_formatter.get_column_names());
            wtr.flush()?;
            println!("Created new log file at {path_tried:?}.");
            config
        }
    };

    if let Some(Commands::Config(args)) = cli.command {
        match args.command {
            ConfigCommands::Set {
                log_file_path,
                row_formatter,
            } => {
                let mut config = config.clone();

                if let Some(log_file_path) = log_file_path {
                    if let Ok(v) = log_file_path.canonicalize() {
                        match v.extension() {
                            Some(ext) if ext.eq_ignore_ascii_case("csv") => {
                                config.log_file_path = v.clone();
                                println!("Log file now at {v:?}.");
                            }
                            _ => {
                                eprintln!("{warning}: The file provided is not the expected 'csv' format: {log_file_path:?}.");
                                exit(1);
                            }
                        }
                    } else {
                        eprintln!(
                            "{warning}: The file provided does not exist: {log_file_path:?}."
                        );
                        exit(1)
                    };
                }
                if let Some(row_formatter) = row_formatter {
                    config.row_formatter = row_formatter;
                    println!("Row formatter is now {row_formatter}.");
                }

                config.save(&config_file_path);

                // if let Ok(v) = log_file_path.canonicalize() {
                //     match v.extension() {
                //         Some(ext) if ext.eq_ignore_ascii_case("csv") => Config {
                //             log_file_path: v,
                //             row_formatter,
                //         }
                //         .save(&config_file_path),
                //         _ => {
                //             eprintln!("{warning}: The file provided is not the expected 'csv' format: {log_file_path:?}.");
                //             exit(1);
                //         }
                //     }
                // } else {
                //     eprintln!("{warning}: The file provided does not exist: {log_file_path:?}.");
                //     exit(1)
                // };
            }
            ConfigCommands::SetDefault => {
                Config::default().save(&config_file_path);
                println!("Configuration options reset.");
            }
            ConfigCommands::Get => println!("{}", serde_json::to_string_pretty(&config)?),
            ConfigCommands::GetDefault => {
                println!("{}", serde_json::to_string_pretty(&Config::default())?)
            }
            ConfigCommands::Path => println!("{}", config_file_path.to_str().unwrap()),
        }
        exit(0);
    }

    let tmp_file_name = Path::new("tmp.json");
    let tmp_file_path = data_dir.join(tmp_file_name);
    let activity = tmp_file_path
        .exists()
        .then(|| Activity::load(&tmp_file_path).unwrap());
    let warn_if_time_is_long = |time_passed: &chrono::TimeDelta| {
        if time_passed.num_hours() >= 10 {
            eprintln!(
                "{warning}: Time recorded was {} hours. Are you sure this is correct?",
                time_passed.num_hours()
            );
        }
    };

    if let Some(cmd) = cli.command {
        match (activity, cmd) {
            (None, Commands::Start { label, time: None }) => {
                Activity {
                    time_started: Local::now(),
                    label: label.clone(),
                }
                .save(&tmp_file_path);
                match label {
                    Some(l) => println!("Activity started '{l}'."),
                    None => println!("Activity started."),
                }
            }
            (
                None,
                Commands::Start {
                    label,
                    time: Some(v),
                },
            ) => {
                let time = match parse_time(&v) {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("{warning}: Could not parse time input '{v}'. Reason: {e}.");
                        exit(-1);
                    }
                };
                Activity {
                    time_started: time,
                    label: label.clone(),
                }
                .save(&tmp_file_path);
                match label {
                    Some(l) => println!("Activity started '{l}' at time {}.", time.format("%H:%M")),
                    None => println!("Activity started at time {}.", time.format("%H:%M")),
                }
            }
            (Some(activity), Commands::Stop { time: None }) => {
                let now = Local::now();

                let time_passed = now.signed_duration_since(activity.time_started);
                warn_if_time_is_long(&time_passed);

                let row = config.row_formatter.make_row(&activity, now);
                append_csv(&config.log_file_path, &row).unwrap();
                fs::remove_file(tmp_file_path).unwrap();

                match activity.label {
                    Some(v) => println!("Stopped activity '{v}'. Logged '{row:?}'."),
                    None => println!("Stopped activity. Logged '{row:?}'."),
                }
            }
            (Some(activity), Commands::Stop { time: Some(v) }) => {
                let time = match parse_time(&v) {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("{warning}: Could not parse time input '{v}'. Reason: {e}.");
                        exit(-1);
                    }
                };

                let row = config.row_formatter.make_row(&activity, time);

                let time_passed = time.signed_duration_since(activity.time_started);
                if time_passed.num_seconds() < 0 {
                    eprintln!("{warning}: Time recorded was negative. Skipping log '{row:?}'.");
                    exit(-1);
                }

                warn_if_time_is_long(&time_passed);

                append_csv(&config.log_file_path, &row).unwrap();
                fs::remove_file(tmp_file_path).unwrap();
                match activity.label {
                    Some(v) => println!("Stopped activity '{v}'. Logged '{row:?}'."),
                    None => println!("Stopped activity. Logged '{row:?}'."),
                }
            }
            (Some(activity), Commands::Abort) => {
                fs::remove_file(tmp_file_path).unwrap();
                match activity.label {
                    Some(v) => println!("Aborted activity '{v}'."),
                    None => println!("Aborted activity."),
                }
            }
            (None, Commands::Stop { .. }) => {
                eprintln!("{warning}: There is no activity being timed, so nothing to stop.")
            }
            (None, Commands::Abort) => {
                eprintln!("{warning}: There is no activity being timed, so nothing to abort.")
            }
            (Some(..), Commands::Start { .. }) => {
                eprintln!(
                    "{warning}: There is already an activity being timed. Won't start another one."
                )
            }
            (.., Commands::Get(GetArgs { command })) => {
                let mut reader_csv = csv::ReaderBuilder::new()
                    .flexible(false)
                    .has_headers(true)
                    .comment(Some(b'#'))
                    .from_path(&config.log_file_path)?;

                let mut writer_csv = csv::Writer::from_writer(io::stdout());

                match command {
                    GetCommands::Logs => {
                        writer_csv.write_record(reader_csv.headers()?);
                        for result in reader_csv.records() {
                            writer_csv.write_record(&result?);
                        }
                        writer_csv.flush()?;
                    }
                    GetCommands::Today => {
                        match config.row_formatter {
                            RowFormatter::V2_1 | RowFormatter::V2_0 => {}
                            _ => unimplemented!(),
                        };

                        writer_csv.write_record(reader_csv.headers()?);
                        for result in reader_csv.records() {
                            let result = result?;
                            let (start, stop) = (
                                result[0].parse::<DateTime<FixedOffset>>()?,
                                result[1].parse::<DateTime<FixedOffset>>()?,
                            );

                            if [start.date_naive(), stop.date_naive()]
                                .contains(&Local::now().date_naive())
                            {
                                writer_csv.write_byte_record(result.as_byte_record());
                            }
                        }
                    }
                    GetCommands::Total => {
                        match config.row_formatter {
                            RowFormatter::V2_1 => {}
                            _ => unimplemented!(),
                        };

                        let mut minutes = HashMap::new();
                        for result in reader_csv.deserialize() {
                            let (start, stop, _, label): (
                                DateTime<Local>,
                                DateTime<Local>,
                                String,
                                String,
                            ) = result?;

                            let duration = stop.signed_duration_since(start);
                            *minutes.entry(label.clone()).or_insert(0) += duration.num_minutes();
                        }

                        writer_csv.write_record(["label", "minutes"]);
                        for (key, value) in minutes {
                            writer_csv.write_record([key, value.to_string()]);
                        }
                        writer_csv.flush();
                    }
                    _ => unimplemented!(),
                }
            }
            (.., Commands::Config { .. }) => unreachable!(),
        }
    } else if tmp_file_path.exists() {
        let activity = Activity::load(&tmp_file_path).unwrap();
        match activity.label {
            Some(v) => {
                let days_passed = Local::now()
                    .signed_duration_since(activity.time_started)
                    .num_days();

                match days_passed {
                    0 => println!(
                        "Currently timing activity '{v}', started at {}.",
                        activity.time_started.format("%H:%M")
                    ),
                    1 => println!(
                        "Currently timing activity '{v}', started yesterday at {}.",
                        activity.time_started.format("%H:%M")
                    ),
                    _ => {
                        println!(
                            "Currently timing activity '{v}', started {days_passed} days ago at {}.",
                            activity.time_started.format("%H:%M")
                        )
                    }
                }
            }
            None => println!(
                "Currently timing activity started at {}.",
                activity.time_started.format("%H:%M")
            ),
        }
    } else {
        println!("No activity is being timed.");
    }
    Ok(())
}
