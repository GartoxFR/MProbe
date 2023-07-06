use std::fs::{self, File};
use std::io::{self, BufWriter, Read};
use std::process::Command;
use std::time::Instant;

use std::fmt::Write;

use chrono::Local;
use clap::Parser;
use common::{HeaderInfo, Pid, SaveFile};

use self::args::Arguments;

mod args;

struct Error;

fn measure_pss(pid: u32, buffer: &mut String) -> Result<usize, Error> {
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

    // let pss_anon = 0;
    // let pss_file = 0;
    // let pss_shmem = 0;

    Ok(pss)
}

fn measure_rss(pid: u32, buffer: &mut String) -> Result<usize, Error> {
    buffer.clear();
    write!(buffer, "/proc/{}/statm", pid).expect("Should be able to write fmt to a string");
    let mut statm = File::open(&buffer).map_err(|_| Error)?;

    buffer.clear();
    statm.read_to_string(buffer).map_err(|_| Error)?;

    let page_count: usize = buffer
        .split_ascii_whitespace()
        .nth(1)
        .ok_or(Error)?
        .parse()
        .map_err(|_| Error)?;

    let rss = page_count * 4;

    Ok(rss)
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

    let measure = match args.method {
        args::Method::Pss => measure_pss,
        args::Method::Rss => measure_rss,
    };

    let mut buffer = String::with_capacity(1024);
    let mut rounds = Vec::with_capacity(1024);
    let mut processes = Vec::with_capacity(100);
    let mut to_explore = Vec::with_capacity(100);

    let start_date = Local::now();

    let sample_period = args.sample_period;
    let mut max_memory_sampled = 0;
    let mut current_sample_index = 0;

    let mut round_count = 0;
    if let Some(command) = args.program.first() {
        let mut handle = Command::new(command)
            .args(&args.program[1..])
            .spawn()
            .unwrap();

        let pid = handle.id() as Pid;
        let start_time = Instant::now();

        while handle.try_wait().unwrap().is_none() {
            round_count += 1;
            // First we check in wich sample we are
            let new_sample_index =
                (start_time.elapsed().as_micros() as usize) / sample_period as usize;

            if new_sample_index != current_sample_index {
                if rounds.len() < current_sample_index {
                    rounds.resize(current_sample_index, 0);
                }
                rounds.push(max_memory_sampled);

                current_sample_index = new_sample_index;
                max_memory_sampled = 0;
            }
            // Round start
            // We look for new children
            explore_children(pid, &mut processes, &mut to_explore, &mut buffer);

            // We sample all of them
            let sum = processes
                .drain(..)
                .filter_map(|pid| measure(pid, &mut buffer).ok())
                .sum();

            max_memory_sampled = usize::max(max_memory_sampled, sum);

            // Round end
        }
        if max_memory_sampled > 0 {
            rounds.push(max_memory_sampled);
        }
    }

    let end_date = Local::now();

    let header = HeaderInfo {
        start_date,
        end_date,
        probe_commit_sha: env!("VERGEN_GIT_SHA").to_owned(),
        probe_build_date: env!("VERGEN_BUILD_DATE").to_owned(),
        round_count,
        command: args.program.join(" "),
        method: args.method.to_string(),
        sample_period,
    };

    let save_file_content = SaveFile {
        header,
        data: rounds,
    };

    let default_name = "samples.json";

    serialize_result(
        args.output.as_ref().map_or(default_name, |s| s.as_str()),
        &save_file_content,
    );
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

    serde_json::to_writer(&mut out, save_file_content).unwrap();
}
