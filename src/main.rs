use std::{error::Error, fs::File, path::PathBuf};

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

    if args.decompress {
        let mut output = args.path.clone();
        output.set_extension("");
        if output.exists() && !args.force {
            return Err("Output already exists. Use -f to force".into());
        }
        let output_dir = output.file_stem().unwrap();
        let output_file = File::open(&args.path)?;

        let dec = snap::read::FrameDecoder::new(output_file);
        let mut archive = Archive::new(dec);
        archive.unpack(output_dir)?;
        std::fs::remove_file(args.path)?;
    } else {
        let mut output = args.path.clone();
        output.set_extension("tszip");
        if output.exists() && !args.force {
            return Err("Output already exists. Use -f to force".into());
        }
        let output_file = File::create(output).unwrap();

        let enc = snap::write::FrameEncoder::new(output_file);
        let mut tar = tar::Builder::new(enc);
        tar.follow_symlinks(args.symbolic_follow);
        tar.append_dir_all("", &args.path)?;

        if !args.keep {
            std::fs::remove_dir_all(args.path)?;
        }
    }

    Ok(())
}
