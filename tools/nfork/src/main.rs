use std::env;
use std::process::Command;

fn main() {
    let mut args = env::args().skip(1);
    let n = args.next().unwrap().parse::<usize>().unwrap();
    let command = args.next().unwrap();
    let args: Vec<_> = args.collect();
    let handles: Vec<_> = (0..n)
        .map(|n| {
            println!("Spawning {n}..");
            Command::new(&command).args(args.iter()).spawn().unwrap()
        })
        .collect(); // We must collect here because of iterator lazyness
                    // We want to fork as soon as possible 

    for mut handle in handles {
        handle.wait().unwrap();
    }
}
