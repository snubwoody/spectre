use spectre::Result;
use spectre::download::install_chrome;
use std::{fs, path::PathBuf};

#[cfg(test)]
fn gen_random_path() -> PathBuf {
    let num: u32 = rand::random();
    let path = PathBuf::from("./.tmp");
    path.join(format!("test-{num}"))
}

#[tokio::test]
async fn install_path() -> Result<()> {
    let path = gen_random_path();
    fs::create_dir_all(&path)?;

    install_chrome(&path).await?;

    #[cfg(target_os = "windows")]
    fs::read_dir(path.join(".spectre/browsers/chrome-win64"))?;

    #[cfg(target_os = "linux")]
    fs::read_dir(path.join(".spectre/browsers/chrome-linux64"))?;

    #[cfg(target_os = "macos")]
    fs::read_dir(path.join(".spectre/browsers/chrome-mac-arm64"))?;

    fs::remove_dir_all(path)?;
    Ok(())
}
