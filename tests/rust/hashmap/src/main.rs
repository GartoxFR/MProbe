use std::collections::HashMap;
use std::hint::black_box;
use std::{env, process};

fn main() {
    let Some(x) = parse_args() else {
        eprintln!("Arguments invalides : hashmap x");
        process::exit(1);
    };

    let mut map = HashMap::new();
    for i in 0..x {
        map.insert(i, i);
    }
    black_box(map);
}

fn parse_args() -> Option<u32> {
    let mut args = env::args().skip(1);

    args.next()?.parse().ok()
}
