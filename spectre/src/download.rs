pub use crate::Error;
use std::{fs, io::Cursor, path::PathBuf};

/// Install chrome for testing
pub async fn install_chrome(path: &PathBuf) -> Result<(), Error> {
    let spectre_dir = path.join(".spectre/browsers");
    fs::create_dir_all(&spectre_dir)?;

    println!("Downloading chrome...");

    #[cfg(target_os="linux")]
    let url = "https://storage.googleapis.com/chrome-for-testing-public/136.0.7103.49/linux64/chrome-linux64.zip";
    
    // TODO add macos_x86 support
    #[cfg(target_os="macos")]
    let url = "https://storage.googleapis.com/chrome-for-testing-public/136.0.7103.49/mac-arm64/chrome-mac-arm64.zip";

    #[cfg(target_os="windows")]
    let url = "https://storage.googleapis.com/chrome-for-testing-public/135.0.7049.95/win64/chrome-win64.zip";

    let response = reqwest::get(url).await?;

    let bytes = response.bytes().await?;
    let cursor = Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor)?;

    println!("Unzipping...");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        // FIXME
        let relpath = file.enclosed_name().unwrap();
        let outpath = spectre_dir.join(relpath);

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