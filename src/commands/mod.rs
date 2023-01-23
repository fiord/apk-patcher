mod decompress;
mod patches;
mod compress;

#[derive(clap::Subcommand)]
pub enum Commands {
    Decompress(decompress::Decompress),
    Patches(patches::Patches),
    Compress(compress::Compress),
}

pub use Commands::Decompress;
pub use Commands::Patches;
pub use Commands::Compress;