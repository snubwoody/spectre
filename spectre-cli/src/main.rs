use clap::{Parser, Subcommand, ValueEnum};
use spectre::{Error, download::install_chrome};

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

// TODO improve messages and add a quiet flag
#[tokio::main]
async fn main() -> Result<(), Error> {
    let home_dir;
    match home::home_dir() {
        Some(dir) => home_dir = dir,
        None => return Err(Error::FailedToGetHomeDir),
    }

    let args = Args::parse();
    match args.command {
        Command::Download { browser } => match browser {
            Browser::Chrome => {
                install_chrome(&home_dir).await?;
            }
        },
    }

    Ok(())
}
