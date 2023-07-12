use std::fs::File;
use std::io::BufReader;

use clap::Parser;
use common::SaveFile;

use crate::args::{Arguments, GraphType};
use crate::plots::plot_memory;

mod args;
mod plots;
pub mod utils;

fn main() {
    let args = Arguments::parse();

    let json_file = BufReader::new(File::open(args.input).unwrap());
    let save_file: SaveFile = serde_json::from_reader(json_file).unwrap();

    if save_file.data.is_empty() {
        return;
    }

    let mut missed_sample_count = 0;
    let res: Vec<_> = save_file
        .data
        .iter()
        .enumerate()
        .filter_map(|(index, memory)| match *memory {
            0 => {
                missed_sample_count += 1;
                None
            }
            memory => Some((index * save_file.header.sample_period as usize, memory)),
        })
        .collect();

    match missed_sample_count {
        0 => eprintln!("No sample missed."),
        missed_sample_count => eprintln!(
            "{} sample missed ({:.2} %)",
            missed_sample_count,
            missed_sample_count as f64 / save_file.data.len() as f64 * 100f64
        ),
    }

    match args.r#type {
        GraphType::Memory => {
            plot_memory(
                args.output.as_ref().map(|s| &s[..]),
                &res,
                save_file.header.title.unwrap_or(save_file.header.command).as_str(),
                !args.queit,
            )
            .unwrap();
        }
    }
}
