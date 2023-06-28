use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Arguments {
    /// Output file destination
    #[clap(short, long)]
    pub output: Option<String>,

    /// Program to run
    pub program: Vec<String>,

}
