use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter, Read};
use std::process::Command;
use std::time::Instant;

use std::fmt::Write;

use common::{Pid, Round, Sample, SampleValue, TimeMicro};

type Result<T> = io::Result<T>;

pub fn measure(pid: u32, buffer: &mut String, start_time: Instant) -> Result<Sample> {
    buffer.clear();
    write!(buffer, "/proc/{}/smaps_rollup", pid).expect("Should be able to write fmt to a string");
    let mut smaps_rollup = File::open(&buffer)?;

    buffer.clear();
    smaps_rollup.read_to_string(buffer)?;

    let mut lines = buffer.lines();
    let pss = parse_line(2, &mut lines)?;
    let pss_anon = parse_line(1, &mut lines)?;
    let pss_file = parse_line(0, &mut lines)?;
    let pss_shmem = parse_line(0, &mut lines)?;

    Ok(Sample {
        time: start_time.elapsed().as_micros() as u64,
        value: SampleValue {
            pss,
            pss_anon,
            pss_file,
            pss_shmem,
        },
    })
}

fn parse_line<'a, 'b>(
    line: usize,
    lines: &mut (impl Iterator<Item = &'b str> + 'a),
) -> Result<usize> {
    lines
        .nth(line)
        .ok_or(io::Error::new(io::ErrorKind::UnexpectedEof, "Line"))?
        .split_whitespace()
        .nth(1)
        .ok_or(io::Error::new(io::ErrorKind::UnexpectedEof, "Tab"))?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, ""))
}

fn main() {
    let mut buffer = String::with_capacity(1024);
    let mut rounds = Vec::with_capacity(1024);
    let mut processes = Vec::with_capacity(100);
    let mut to_explore = Vec::with_capacity(100);

    let mut args = env::args().skip(1);
    let command = args.next().unwrap();

    let mut handle = Command::new(command).args(args).spawn().unwrap();
    let start_time = Instant::now();

    let pid = handle.id() as Pid;

    while handle.try_wait().unwrap().is_none() {
        // Round start
        let round_start = start_time.elapsed().as_micros() as TimeMicro;

        // We look for new children
        explore_children(pid, &mut processes, &mut to_explore, &mut buffer);

        // We sample all of them
        let mut samples = HashMap::with_capacity(processes.len());
        for process in processes.drain(..) {
            if let Ok(sample) = measure(process, &mut buffer, start_time) {
                samples.insert(process, sample);
            }
        }

        let round_end = start_time.elapsed().as_micros() as TimeMicro;
        rounds.push(Round {
            start_time: round_start,
            end_time: round_end,
            samples,
        })
        // Round end
    }

    serialize_result(rounds);
}

fn explore_children(
    parent: Pid,
    processes: &mut Vec<Pid>,
    to_explore: &mut Vec<Pid>,
    buffer: &mut String,
) {
    to_explore.push(parent);
    while let Some(pid) = to_explore.pop() {
        // Ignore error because we can't do anything
        let _ = list_children(pid, to_explore, buffer);

        processes.push(pid);
    }
}

fn list_children(pid: Pid, to_explore: &mut Vec<Pid>, buffer: &mut String) -> Result<()> {
    buffer.clear();
    write!(buffer, "/proc/{pid}/task").expect("Should be able to write fmt to a string");
    for dir_entry in fs::read_dir(&*buffer)? {
        buffer.clear();
        let mut task_dir = dir_entry.expect("Cannot read dir entry").path();
        task_dir.push("children");
        File::open(task_dir)?.read_to_string(buffer).unwrap();
        to_explore.extend(buffer.split(' ').filter_map(|str| str.parse::<u32>().ok()));
    }

    Ok(())
}

fn serialize_result(rounds: Vec<Round>) {
    let mut out = BufWriter::new(File::create("detail.json").unwrap());
    serde_json::to_writer_pretty(&mut out, &rounds).unwrap();
    // println!("{res:?}")
}
