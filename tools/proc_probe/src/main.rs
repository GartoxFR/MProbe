use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Seek};
use std::process::Command;
use std::time::Instant;

use common::{ProcessInfo, Sample, SampleValue};

type Result<T> = io::Result<T>;

#[derive(Debug)]
#[allow(dead_code)]
struct ProcFsProbe {
    pid: u32,
    read_buffer: String,
}

impl ProcFsProbe {
    pub fn try_new(pid: u32) -> Result<Self> {
        Ok(Self {
            pid,
            read_buffer: String::default(),
        })
    }

    pub fn measure(&mut self, start_time: Instant) -> Result<Sample> {
        let mut smaps_rollup = File::open(format!("/proc/{}/smaps_rollup", self.pid))?;
        smaps_rollup.seek(io::SeekFrom::Start(0))?;
        self.read_buffer.clear();
        smaps_rollup.read_to_string(&mut self.read_buffer)?;

        let mut lines = self.read_buffer.lines();
        let pss = Self::parse_line(2, &mut lines)?;
        let pss_anon = Self::parse_line(1, &mut lines)?;
        let pss_file = Self::parse_line(0, &mut lines)?;
        let pss_shmem = Self::parse_line(0, &mut lines)?;

        Ok(Sample {
            time_us: start_time.elapsed().as_micros(),
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
}

fn main() {
    let mut process_infos = HashMap::new();

    let mut args = env::args().skip(1);
    let command = args.next().unwrap();

    let mut handle = Command::new(command).args(args).spawn().unwrap();
    let start_time = Instant::now();

    process_infos.insert(
        handle.id(),
        (
            ProcessInfo::default(),
            Some(ProcFsProbe::try_new(handle.id()).expect("Could not create first probe")),
        ),
    );

    let mut new_children = Vec::new();

    // We look for new children

    while handle.try_wait().unwrap().is_none() {
        for (pid, _) in process_infos
            .iter()
            .filter(|(_, (_, probe))| probe.is_some())
        {
            explore_children(*pid, &process_infos, &mut new_children);
        }
        process_infos.extend(new_children.drain(..));

        for (_, (process_info, probe)) in process_infos
            .iter_mut()
            .filter(|(_, (_, probe))| probe.is_some())
        {
            match probe.as_mut().unwrap().measure(start_time) {
                Ok(measurement) => {
                    process_info.measurements.push(measurement);
                }
                Err(_e) => {
                    probe.take();
                }
            }
        }
    }

    compute_result(process_infos.into_iter().map(|(pid, (pinfo, _))| (pid, pinfo)).collect());

    // for (pid, process_info) in process_infos {
    //     println!(
    //         "{} => {:?} ({} mesures)",
    //         pid,
    //         process_info.measurements,
    //         process_info.measurements.len()
    //     );
    // }
}

fn explore_children(
    parent: u32,
    process_infos: &HashMap<u32, (ProcessInfo, Option<ProcFsProbe>)>,
    new_children: &mut Vec<(u32, (ProcessInfo, Option<ProcFsProbe>))>,
) {
    if let Some(children) = list_children(parent) {
        for child in children {
            if !process_infos.contains_key(&child) {
                match ProcFsProbe::try_new(child) {
                    Ok(probe) => {
                        new_children.push((child, (ProcessInfo::default(), Some(probe))));
                    }
                    Err(_) => {
                        eprintln!("Could not open procfs for newly discovered child");
                    }
                }
                explore_children(child, process_infos, new_children)
            }
        }
    }
}

fn list_children(pid: u32) -> Option<Vec<u32>> {
    let path = format!("/proc/{pid}/task");
    let mut children = Vec::new();
    let mut buffer = String::new();
    for dir_entry in fs::read_dir(path).ok()? {
        buffer.clear();
        let mut task_dir = dir_entry.expect("Cannot read dir entry").path();
        task_dir.push("children");
        File::open(task_dir)
            .ok()?
            .read_to_string(&mut buffer)
            .unwrap();
        children.extend(buffer.split(' ').filter_map(|str| str.parse::<u32>().ok()));
    }

    Some(children)
}

fn compute_result(process_infos: HashMap<u32, ProcessInfo>) {
    // const TIME_STEP_US: u128 = 10;
    // let mut iters: Vec<_> = process_infos
    //     .values()
    //     .map(|pinfo| (SampleValue::default(), pinfo.measurements.iter().peekable()))
    //     .collect();
    //
    // let res: Vec<_> = (1..)
    //     .map_while(|next_time_step| {
    //         let mut sum = SampleValue::default();
    //         let mut stop = true;
    //         for (prev, iter) in iters.iter_mut().filter(|(_, iter)| iter.len() > 0) {
    //             stop = false;
    //             match iter
    //                 .peeking_take_while(|measure| measure.time_us < next_time_step * TIME_STEP_US)
    //                 .map(|measure| measure.value)
    //                 .max()
    //             {
    //                 Some(max) => {
    //                     *prev = max;
    //                     sum += max
    //                 }
    //                 None => {
    //                     sum += *prev;
    //                 }
    //             }
    //         }
    //         if stop {
    //             None
    //         } else {
    //             Some(sum)
    //         }
    //     })
    //     .collect();
    //
    // let mut out = BufWriter::new(File::create("res_seconds.csv").unwrap());
    // for (i, m) in res.iter().enumerate() {
    //     writeln!(out, "{},{}", i as u128 * TIME_STEP_US, m.pss).unwrap();
    // }
    //
    // let res_byte = res
    //     .iter()
    //     .scan((0, 0), |(acc, prev), m| {
    //         *acc += m.pss.saturating_sub(*prev);
    //         *prev = m.pss;
    //
    //         Some((*acc, m))
    //     })
    //     .dedup();
    //
    // let mut out = BufWriter::new(File::create("res_bytes.csv").unwrap());
    // for (i, m) in res_byte {
    //     writeln!(out, "{},{}", i, m.pss).unwrap();
    // }

    let mut out = BufWriter::new(File::create("detail.json").unwrap());
    serde_json::to_writer_pretty(&mut out, &process_infos).unwrap();

    // println!("{res:?}")
}
