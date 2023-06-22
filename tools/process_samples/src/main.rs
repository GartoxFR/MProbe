use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use common::{Round, SampleValue, TimeMicro, Sample};

fn main() {
    let json_path = env::args().nth(1).unwrap();
    let json_file = BufReader::new(File::open(json_path).unwrap());
    let rounds: Vec<Round> = serde_json::from_reader(json_file).unwrap();
    println!("{}", rounds.len());

    let res: Vec<(TimeMicro, SampleValue, usize, TimeMicro)> = rounds
        .iter()
        .map(|round| (round.start_time, round.samples.iter().map(|(_, Sample {value, ..},)| value).copied().sum(), round.samples.len(), round.end_time - round.start_time))
        .collect();

    let mut out = BufWriter::new(File::create("res_seconds.csv").unwrap());
    writeln!(out, "Time,Memory,Process count,Duration").unwrap();
    for (start_time, sample, process_count, duration) in res.iter() {
        writeln!(out, "{},{},{},{}", start_time, sample.pss, process_count, duration).unwrap();
    }

    let res_byte = res
        .iter()
        .scan((0, 0), |(acc, prev), (_, m, _, _)| {
            *acc += m.pss.saturating_sub(*prev);
            *prev = m.pss;

            Some((*acc, m))
        })
        .dedup();

    let mut out = BufWriter::new(File::create("res_bytes.csv").unwrap());
    for (i, m) in res_byte {
        writeln!(out, "{},{}", i, m.pss).unwrap();
    }
}
