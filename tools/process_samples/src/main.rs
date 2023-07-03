use std::fs::File;
use std::io::BufReader;

use clap::Parser;
use common::{Sample, SampleValue, SaveFile, TimeMicro};

use crate::args::{Arguments, GraphType};
use crate::plots::{plot_memory, plot_process_duration};

mod args;
mod plots;
pub mod utils;

fn main() {
    let args = Arguments::parse();

    let json_file = BufReader::new(File::open(args.input).unwrap());
    let save_file: SaveFile = serde_json::from_reader(json_file).unwrap();

    if save_file.rounds.is_empty() {
        return;
    }

    let res: Vec<(TimeMicro, SampleValue, usize, TimeMicro)> = save_file
        .rounds
        .iter()
        .map(|round| {
            (
                round.start_time,
                round
                    .samples
                    .iter()
                    .map(|(_, Sample { value, .. })| value)
                    .copied()
                    .sum(),
                round.samples.len(),
                round.end_time - round.start_time,
            )
        })
        .collect();

    match args.r#type {
        GraphType::Memory => {
            let vec: Vec<_> = res
                .iter()
                .map(|(x, y, _, _)| (*x as usize, y.pss))
                .collect();
            plot_memory(args.output.as_ref().map(|s| &s[..]), &vec, &save_file.header.command, !args.queit).unwrap();
        }
        GraphType::Duration => {
            let vec: Vec<_> = res.iter().map(|(_, _, x, y)| (*x, *y as usize)).collect();
            plot_process_duration(args.output.as_ref().map(|s| &s[..]), &vec, &save_file.header.command, !args.queit).unwrap();
        }
    }
}
