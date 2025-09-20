use std::{
    error::Error,
    fs::{remove_dir_all, File},
    path::PathBuf,
};

use clap::Parser;
use tar::Archive;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to compress or decompress
    path: PathBuf,
    #[arg(long, short)]
    /// Decompress data (default is compression)
    decompress: bool,
    #[arg(long, short)]
    /// Keep (don't delete) input directory/file during (de)compression
    keep: bool,
    #[arg(long, short)]
    /// Force (de)compression if the output already exists
    force: bool,
    #[arg(long, short)]
    /// Follow hard links; archive and dump the files they refer to
    symbolic_follow: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut output = args.path.clone();
    output.set_extension("");
    if output.exists() && !args.force {
        return Err("Output already exists. Use -f to force".into());
    }

    if args.decompress {
        let output_dir = output.file_stem().unwrap();
        let output_file = File::open(&args.path)?;

        let dec = snap::read::FrameDecoder::new(output_file);
        Archive::new(dec).unpack(output_dir)?;
    } else {
        let output_file = File::create(output)?;

        let enc = snap::write::FrameEncoder::new(output_file);
        let mut tar = tar::Builder::new(enc);
        tar.follow_symlinks(args.symbolic_follow);
        tar.append_dir_all("", &args.path)?;
    }

    match args.keep {
        true => Ok(()),
        false => Ok(remove_dir_all(args.path)?),
    }
}
