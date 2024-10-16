use std::{fmt::Display, ops::Index, str::FromStr};

use chrono::{DateTime, FixedOffset, Local, SecondsFormat::Secs};
use clap::ValueEnum;
use csv::StringRecord;
use serde::{Deserialize, Serialize};

use crate::cli::RowFormatter;
use crate::Activity;

impl RowFormatter {
    pub fn make_row(&self, activity: &Activity, time: DateTime<Local>) -> Vec<String> {
        let time_passed = time.signed_duration_since(activity.time_started);
        let hours_passed = time_passed.num_hours();
        let minutes_passed = time_passed.num_minutes() % 60;
        let duration = format!("{hours_passed:02}:{minutes_passed:02}");
        let label = activity.label.clone().unwrap_or("-".into());

        match self {
            Self::V2_1 => {
                let start = activity.time_started.to_rfc3339_opts(Secs, true);
                let finish = time.to_rfc3339_opts(Secs, true);
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

    pub fn get_column_names(&self) -> Vec<&str> {
        match self {
            RowFormatter::V1_0 => vec!["date", "time-start", "time-stop", "duration", "label"],
            RowFormatter::V2_0 => vec!["datetime-start, datetime-stop", "duration", "label"],
            RowFormatter::V2_1 => vec!["datetime-start", "datetime-stop", "duration", "label"],
        }
    }

    pub fn read_row(&self, row: &StringRecord) -> RowOutput {
        match self {
            RowFormatter::V1_0 => todo!(),
            RowFormatter::V2_0 => todo!(),
            RowFormatter::V2_1 => RowOutput {
                start: row[0].parse().unwrap(),
                stop: row[1].parse().unwrap(),
                label: row[3].to_string(),
            },
        }
    }
}

pub struct RowOutput {
    pub start: DateTime<FixedOffset>,
    pub stop: DateTime<FixedOffset>,
    pub label: String,
}
