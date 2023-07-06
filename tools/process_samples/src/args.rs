use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Arguments {
    /// JSON data input file (proc_probe output)
    pub input: String,

    
    /// Output file destination
    #[clap(short, long)]
    pub output: Option<String>,
    #[clap(short, long, value_enum, default_value_t = GraphType::Memory)]
    /// Which graph you want
    pub r#type: GraphType,

    /// Don't show the graph and just save it
    #[clap(short, long)]
    pub queit: bool

}

#[derive(ValueEnum, Clone, Copy)]
pub enum GraphType {
    Memory,
}
