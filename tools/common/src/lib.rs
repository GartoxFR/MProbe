use std::collections::HashMap;
use std::iter::Sum;
use std::ops::{Add, AddAssign};

use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

pub type Pid = u32;
pub type TimeMicro = u64;
 


#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderInfo {
    pub start_date: DateTime<Local>,
    pub end_date: DateTime<Local>,
    pub probe_commit_sha: String,
    pub probe_build_date: String,
    pub round_count: usize,
    pub command: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveFile {
    pub header: HeaderInfo,
    pub rounds: Vec<Round>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sample {
    pub time: TimeMicro,
    pub value: SampleValue
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SampleValue {
    pub pss: usize,
    pub pss_anon: usize,
    pub pss_file: usize,
    pub pss_shmem: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Round {
    pub start_time: TimeMicro,
    pub end_time: TimeMicro,
    pub samples: HashMap<Pid, Sample>
}

impl Add for SampleValue {
    type Output = SampleValue;

    fn add(self, rhs: Self) -> Self::Output {
        SampleValue {
            pss: self.pss + rhs.pss,
            pss_anon: self.pss_anon + rhs.pss_anon,
            pss_file: self.pss_file + rhs.pss_file,
            pss_shmem: self.pss_shmem + rhs.pss_shmem,
        }
    }
}

impl AddAssign for SampleValue {
    fn add_assign(&mut self, rhs: Self) {
        self.pss += rhs.pss;
        self.pss_anon += rhs.pss_anon;
        self.pss_file += rhs.pss_file;
        self.pss_shmem += rhs.pss_shmem;
    }
}

impl Sum for SampleValue {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = SampleValue::default();
        for s in iter {
            sum += s;
        }
        sum
    }
}
