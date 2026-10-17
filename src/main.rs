mod profile;
mod render;

use std::{env, error::Error, fs, io, path::PathBuf, process::ExitCode};

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let Some(input) = args.next() else {
        return Err(usage_error().into());
    };

    if input == "--help" || input == "-h" {
        print_usage();
        return Ok(());
    }

    let input = PathBuf::from(input);
    let output = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("output/profile.png"));
    if args.next().is_some() {
        return Err(usage_error().into());
    }

    let profile = profile::load(&input)?;
    let html = render::render_html(&profile)?;
    let png = render::render_png(&html).await?;

    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, png)?;
    println!("Generated {}", output.display());
    Ok(())
}

fn usage_error() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        "usage: profileforge <input.yaml> [output.png] (use --help for details)",
    )
}

fn print_usage() {
    println!("Usage: profileforge <input.yaml> [output.png]");
    println!("Defaults output to output/profile.png.");
    println!("Set CHROMIUM_PATH to use a Chromium executable outside PATH.");
}
