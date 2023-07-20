use std::borrow::Cow;
use std::fs::File;
use std::io::BufReader;
use std::thread;

use clap::Parser;
use common::{HeaderInfo, SaveFile};
use plotpy::Plot;

use crate::args::Arguments;

use self::plots::add_curve_to_plot;

mod args;
mod plots;
pub mod utils;

fn process_input_file(input: &str) -> (&str, HeaderInfo, Vec<(usize, usize)>) {
    let json_file = BufReader::new(File::open(&input).unwrap());
    let save_file: SaveFile = serde_json::from_reader(json_file).unwrap();

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

    if missed_sample_count > 0 {
        eprintln!(
            "{}: {} sample missed ({:.2} %)",
            input,
            missed_sample_count,
            missed_sample_count as f64 / save_file.data.len() as f64 * 100f64
        );
    }

    (input, save_file.header, res)
}

fn main() {
    let Arguments {
        inputs,
        output,
        queit,
        single_window,
    } = Arguments::parse();

    let processed = thread::scope(|s| {
        // Spawning all threads
        let join_handles: Vec<_> = inputs
            .iter()
            .map(|input| s.spawn(|| process_input_file(input)))
            .collect();

        // Waiting for all thread to terminate and collect their result
        join_handles
            .into_iter()
            .filter_map(|handle| match handle.join() {
                Ok(result) => Some(result),
                Err(err) => {
                    eprintln!("A processing thread panicked {:?}", err);
                    None
                }
            })
            .collect::<Vec<_>>()
    });

    match single_window {
        true => {
            let grid_cols = (processed.len() as f64).sqrt().ceil() as usize;
            let grid_rows = (processed.len() as f64 / grid_cols as f64).ceil() as usize;
            let mut plot = Plot::new();

            for (i, (_, header, res)) in processed.into_iter().enumerate() {
                if res.is_empty() {
                    continue;
                }

                plot.set_subplot(grid_rows, grid_cols, i + 1);
                add_curve_to_plot(
                    &res,
                    header.title.unwrap_or(header.command).as_str(),
                    &mut plot,
                    false,
                );
            }
            let plot_file_name = output.as_ref().map(|s| &s[..]).unwrap_or("all.svg");

            match queit {
                true => plot.save(plot_file_name).unwrap(),
                false => plot.save_and_show(plot_file_name).unwrap(),
            };
        }
        false => {
            let output = output.filter(|_| {
                if processed.len() > 1 {
                    eprintln!(
                        "Warning: arguments -o ignore because multiple file will be generated"
                    );
                    false
                } else {
                    true
                }
            });
            thread::scope(|s| {
                for (input, header, res) in processed.into_iter() {
                    if res.is_empty() {
                        continue;
                    }
                    let plot_file_name = match output {
                        Some(ref name) => Cow::from(name),
                        None => Cow::from(format!(
                            "{}.svg",
                            input
                                .split('.')
                                .next()
                                .expect("Split should not return empty iterator")
                        )),
                    };
                    let mut plot = Plot::new();
                    add_curve_to_plot(
                        &res,
                        header.title.unwrap_or(header.command).as_str(),
                        &mut plot,
                        true,
                    );
                    s.spawn(move || match queit {
                        true => plot.save(plot_file_name.as_ref()).unwrap(),
                        false => plot.save_and_show(plot_file_name.as_ref()).unwrap(),
                    });
                }
            });
        }
    }
}
