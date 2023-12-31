use clap::{Parser, ValueEnum};
use common::TimeMicro;

#[derive(Parser)]
#[command(author = "CHORYNSKI Ewan", about = "Measure a given program memory usage by probing its procfs")]
pub struct Arguments {
    /// Output file destination
    #[clap(short, long)]
    pub output: Option<String>,

    /// Program to run
    pub program: Vec<String>,

    #[clap(short, long, value_enum, default_value_t = Method::Rss)]
    pub method: Method,

    /// Sample period in micro second
    #[clap(short, long, default_value_t = 1000)]
    pub sample_period: TimeMicro,

    #[clap(long)]
    pub title: Option<String>,
}

#[derive(ValueEnum, Clone, Copy)]
pub enum Method {
    Pss,
    Rss,
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match *self {
            Method::Pss => "PSS".to_owned(),
            Method::Rss => "RSS".to_owned(),
        }
    }
}
