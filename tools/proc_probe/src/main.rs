use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{Read, Seek};
use std::process::Command;

#[derive(Debug)]
#[allow(dead_code)]
struct ProcFsHandles {
    status: File,
}

#[derive(Debug)]
#[allow(dead_code)]
struct ProcessInfo {
    probe_count: u32,
    alive: bool,
    procfs_handles: Option<ProcFsHandles>,
}

impl Default for ProcessInfo {
    fn default() -> Self {
        Self {
            probe_count: Default::default(),
            alive: true,
            procfs_handles: Default::default(),
        }
    }
}
fn main() {
    let mut process_infos = HashMap::new();
    let mut buffer = String::new();

    let mut args = env::args().skip(1);
    let command = args.next().unwrap();

    let mut handle = Command::new(command).args(args).spawn().unwrap();

    let status_file = File::open(format!("/proc/{}/status", handle.id())).unwrap();
    println!("First status read : {}", fs::read_to_string(format!("/proc/{}/status", handle.id())).unwrap());
    process_infos.insert(
        handle.id(),
        ProcessInfo {
            procfs_handles: Some(ProcFsHandles {
                status: status_file,
            }),
            ..Default::default()
        },
    );

    let mut new_children = Vec::new();

    // We look for new children

    while handle.try_wait().unwrap().is_none() {
        for (pid, _) in process_infos.iter().filter(|(_, info)| info.alive) {
            explore_children(*pid, &process_infos, &mut new_children);
        }
        process_infos.extend(new_children.drain(..));

        for (_, mut process_info) in process_infos.iter_mut().filter(|(_, info)| info.alive) {
            buffer.clear();
            process_info.procfs_handles.as_mut().unwrap().status.seek(std::io::SeekFrom::Start(0)).unwrap();
            match process_info
                .procfs_handles
                .as_mut()
                .unwrap()
                .status
                .read_to_string(&mut buffer) {
                    Ok(_) => {
                        process_info.probe_count += 1;
                    }
                    Err(_) => {
                        process_info.alive = false;
                        process_info.procfs_handles.take();
                    }
                }

                
        }
    }

    println!("{:#?}", process_infos);
}

fn explore_children(
    parent: u32,
    process_infos: &HashMap<u32, ProcessInfo>,
    new_children: &mut Vec<(u32, ProcessInfo)>,
) {
    if let Some(children) = list_children(parent) {
        for child in children {
            if !process_infos.contains_key(&child) {
                let status_file = File::open(format!("/proc/{child}/status")).unwrap();
                new_children.push((
                    child,
                    ProcessInfo {
                        procfs_handles: Some(ProcFsHandles {
                            status: status_file,
                        }),
                        ..Default::default()
                    },
                ));
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
