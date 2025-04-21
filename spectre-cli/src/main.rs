use clap::{Parser, Subcommand, ValueEnum};
pub use spectre_core::Error;
use std::io::Cursor;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Download a browser
    Download { browser: Browser },
}

#[derive(Debug, Clone, ValueEnum)]
enum Browser {
    Chrome,
}

async fn download_chrome() -> Result<(), Error> {
    println!("Downloading chrome...");
    let url = "https://storage.googleapis.com/chrome-for-testing-public/135.0.7049.95/win64/chrome-win64.zip";
    let response = reqwest::get(url).await?;

    let bytes = response.bytes().await?;
    let cursor = Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor)?;

    println!("Unzipping...");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        // FIXME
        let outpath = file.enclosed_name().unwrap();

        if file.is_dir() {
            std::fs::create_dir_all(outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut out_file = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut out_file)?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    match args.command {
        Command::Download { browser } => match browser {
            Browser::Chrome => {
                download_chrome().await?;
            }
        },
    }

    Ok(())
}
