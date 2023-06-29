use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufWriter, Read};
use std::process::Command;
use std::time::Instant;

use std::fmt::Write;

use chrono::Local;
use clap::Parser;
use common::{Pid, Round, Sample, SampleValue, TimeMicro, HeaderInfo, SaveFile};

use self::args::Arguments;

mod args;

struct Error;

fn measure(pid: u32, buffer: &mut String, start_time: Instant) -> Result<Sample, Error> {
    buffer.clear();
    write!(buffer, "/proc/{}/smaps_rollup", pid).expect("Should be able to write fmt to a string");
    let mut smaps_rollup = File::open(&buffer).map_err(|_| Error)?;

    buffer.clear();
    smaps_rollup.read_to_string(buffer).map_err(|_| Error)?;

    let mut lines = buffer.lines();
    let pss = parse_line(2, &mut lines)?;
    // let pss_anon = parse_line(1, &mut lines)?;
    // let pss_file = parse_line(0, &mut lines)?;
    // let pss_shmem = parse_line(0, &mut lines)?;

    let pss_anon = 0;
    let pss_file = 0;
    let pss_shmem = 0;

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
) -> Result<usize, Error> {
    lines
        .nth(line)
        .ok_or(Error)?
        .split_whitespace()
        .nth(1)
        .ok_or(Error)?
        .parse()
        .map_err(|_| Error)
}

fn main() {
    let args = Arguments::parse();
    let mut buffer = String::with_capacity(1024);
    let mut rounds = Vec::with_capacity(1024);
    let mut processes = Vec::with_capacity(100);
    let mut to_explore = Vec::with_capacity(100);

    let start_date = Local::now();

    if let Some(command) = args.program.first() {
        let mut handle = Command::new(command)
            .args(&args.program[1..])
            .spawn()
            .unwrap();

        let pid = handle.id() as Pid;
        let mut last_round_end = 0;
        let start_time = Instant::now();
        while handle.try_wait().unwrap().is_none() {
            // Round start
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
                start_time: last_round_end,
                end_time: round_end,
                samples,
            });
            last_round_end = round_end;
            // Round end
        }
    }

    let end_date = Local::now();

    let header = HeaderInfo {
        start_date,
        end_date,
        probe_commit_sha: env!("VERGEN_GIT_SHA").to_owned(),
        probe_build_date: env!("VERGEN_BUILD_DATE").to_owned(),
        round_count: rounds.len(),
        command: args.program.join(" "),
    };

    let save_file_content = SaveFile {
        header,
        rounds
    };

    serialize_result(&args.output.unwrap_or("detail.json".into()), &save_file_content);
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

fn list_children(
    pid: Pid,
    to_explore: &mut Vec<Pid>,
    buffer: &mut String,
) -> Result<(), io::Error> {
    buffer.clear();
    write!(buffer, "/proc/{pid}/task").expect("Should be able to write fmt to a string");
    for dir_entry in fs::read_dir(&*buffer)? {
        buffer.clear();
        write!(
            buffer,
            "/proc/{pid}/task/{}/children",
            dir_entry?
                .file_name()
                .as_os_str()
                .to_str()
                .expect("Dir name should be valid utf 8")
        )
        .expect("Should be able to write fmt to a string");
        let mut file = File::open(&mut *buffer)?;
        buffer.clear();
        file.read_to_string(buffer).unwrap();
        to_explore.extend(buffer.split(' ').filter_map(|str| str.parse::<u32>().ok()));
    }

    Ok(())
}

fn serialize_result(filename: &str, save_file_content: &SaveFile) {
    let mut out = BufWriter::new(File::create(filename).unwrap());
    serde_json::to_writer_pretty(&mut out, save_file_content).unwrap();
}
