use std::ops::{Add, AddAssign};

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Sample {
    pub time_us: u128,
    pub value: SampleValue
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SampleValue {
    pub pss: usize,
    pub pss_anon: usize,
    pub pss_file: usize,
    pub pss_shmem: usize,
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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub measurements: Vec<Sample>,
}
