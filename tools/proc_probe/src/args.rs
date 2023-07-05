use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Arguments {
    /// Output file destination
    #[clap(short, long)]
    pub output: Option<String>,

    /// Program to run
    pub program: Vec<String>,

    #[clap(short, long, value_enum, default_value_t = Method::Pss)]
    pub method: Method,

    /// Use gzip to compress the json file
    #[clap(long)]
    pub compress: bool,

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
