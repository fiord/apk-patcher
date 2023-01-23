use clap::Parser;

#[derive(Parser)]
pub struct Patches {
    #[arg(short, long="output")]
    output: Option<std::path::PathBuf>,
}