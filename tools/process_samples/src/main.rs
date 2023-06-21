use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufWriter};
use itertools::Itertools;

use common::{ProcessInfo, SampleValue};

fn main() {
    let json_path = env::args().nth(1).unwrap();
    let json_file = BufReader::new(File::open(json_path).unwrap());
    let process_infos: HashMap<usize, ProcessInfo> = serde_json::from_reader(json_file).unwrap();
    println!("{}", process_infos.len());
    const TIME_STEP_US: u128 = 1_000;
    let mut iters: Vec<_> = process_infos
        .values()
        .map(|pinfo| (SampleValue::default(), pinfo.measurements.iter().peekable()))
        .collect();

    let res: Vec<_> = (1..)
        .map_while(|next_time_step| {
            let mut sum = SampleValue::default();
            let mut stop = true;
            for (prev, iter) in iters.iter_mut().filter(|(_, iter)| iter.len() > 0) {
                stop = false;
                match iter
                    .peeking_take_while(|measure| measure.time_us < next_time_step * TIME_STEP_US)
                    .map(|measure| measure.value)
                    .max()
                {
                    Some(max) => {
                        *prev = max;
                        sum += max
                    }
                    None => {
                        sum += *prev;
                    }
                }
            }
            if stop {
                None
            } else {
                Some(sum)
            }
        })
        .collect();

    let mut out = BufWriter::new(File::create("res_seconds.csv").unwrap());
    for (i, m) in res.iter().enumerate() {
        writeln!(out, "{},{}", i as u128 * TIME_STEP_US, m.pss).unwrap();
    }

    let res_byte = res
        .iter()
        .scan((0, 0), |(acc, prev), m| {
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
