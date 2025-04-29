pub use crate::Error;
use std::{fs, io::Cursor, path::PathBuf};

/// Install chrome for testing
pub async fn install_chrome(path: &PathBuf) -> Result<(), Error> {
    let spectre_dir = path.join(".spectre/browsers");
    fs::create_dir_all(&spectre_dir)?;

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