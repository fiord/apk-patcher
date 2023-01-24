mod commands;
use clap::{Parser};

#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        commands::Decompress(decompress) => {
            decompress.execute();
        },
        commands::Patches(patches) => {
            println!("patches");
        },
        commands::Compress(compress) => {
            compress.execute();
        },
    }

}
