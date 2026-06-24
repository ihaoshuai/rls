use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Arg {
    pub path: Option<String>,

    #[arg(short, long)]
    pub all: bool,

    #[arg(short, long)]
    pub du: bool,
}
