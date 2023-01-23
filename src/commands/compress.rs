use clap::Parser;

#[derive(Parser)]
pub struct Compress {
    #[arg(short, long="output")]
    output: Option<std::path::PathBuf>,
}