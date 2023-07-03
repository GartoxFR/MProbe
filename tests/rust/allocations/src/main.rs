use std::{env, process};
use std::hint::black_box;

fn main() {
    env::args().len();

    let Some((x, y)) = parse_args() else {
        eprintln!("Arguments invalides : allocations x y");
        process::exit(1);
    };

    if y > x {
        eprintln!("Arguments invalides : y > x");
        process::exit(2);
    }

    let mut buffer: Vec<u8> = Vec::default();

    // On alloue x MB
    buffer.reserve(x * 1024 * 1024); 

    // On écrit y MB
    buffer.extend((0..(y * 1024 * 1024)).map(|_| 42u8)); 

    black_box(buffer);

}

fn parse_args() -> Option<(usize, usize)> {
    let mut args = env::args().skip(1);

    let x = args.next()?.parse().ok()?;
    let y = args.next()?.parse().ok()?;

    Some((x, y))
}
