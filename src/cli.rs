use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLi {
    // #[arg(short, long)]
    pub path: String,
}
