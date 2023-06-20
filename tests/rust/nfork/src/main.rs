use std::env;
use std::process::Command;

fn main() {
    let mut args = env::args().skip(1);
    let n = args.next().unwrap().parse::<usize>().unwrap();
    let command = args.next().unwrap();
    let args: Vec<_> = args.collect();
    let handles = (0..n).map(|_| {
        Command::new(&command).args(args.iter()).spawn().unwrap()
    });
    
    for mut handle in handles {
        handle.wait().unwrap();
    }
}
