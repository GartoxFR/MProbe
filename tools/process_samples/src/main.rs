use std::fs::File;
use std::io::BufReader;

use clap::Parser;
use common::{Sample, SampleValue, TimeMicro, SaveFile};

use crate::args::{Arguments, GraphType};
use crate::plots::{plot_memory, plot_process_duration};

mod plots;
mod args;

fn main() {
    let args = Arguments::parse();
    
    let json_file = BufReader::new(File::open(args.input).unwrap());
    let save_file: SaveFile = serde_json::from_reader(json_file).unwrap();

    if save_file.rounds.is_empty() {
        return;
    }

    let res: Vec<(TimeMicro, SampleValue, usize, TimeMicro)> = save_file.rounds
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
        GraphType::Memory => 
            plot_memory(
                args.output.as_ref().map(|s| &s[..]),
                res.iter().map(|(x, y, _, _)| (*x as usize, y.pss)),
                !args.queit
            )
            .unwrap(),
        GraphType::Duration => 
            plot_process_duration(
                args.output.as_ref().map(|s| &s[..]),
                res.iter().map(|(_, _, x, y)| (*x, *y as usize)),
                !args.queit
            )
            .unwrap()
    }

}
