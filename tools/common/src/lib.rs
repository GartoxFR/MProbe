use std::collections::HashMap;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

pub type Pid = u32;
pub type TimeMicro = u64;

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderInfo {
    pub start_date: DateTime<Local>,
    pub end_date: DateTime<Local>,
    pub probe_commit_sha: String,
    pub probe_build_date: String,
    pub round_count: usize,
    pub command: String,
    pub method: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveFile {
    pub header: HeaderInfo,
    pub rounds: Vec<Round>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sample {
    pub time: TimeMicro,
    pub value: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Round {
    pub start_time: TimeMicro,
    pub end_time: TimeMicro,
    pub samples: HashMap<Pid, Sample>,
}
