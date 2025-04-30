pub use crate::Error;
use std::{
    fs::{self},
    io::Cursor,
    path::Path,
};

/// Install chrome for testing
pub async fn install_chrome(path: &Path) -> Result<(), Error> {
    let spectre_dir = path.join(".spectre/browsers");
    fs::create_dir_all(&spectre_dir)?;

    println!("Downloading chrome...");

    #[cfg(target_os = "linux")]
    let url = "https://storage.googleapis.com/chrome-for-testing-public/136.0.7103.49/linux64/chrome-linux64.zip";

    // TODO add macos_x86 support
    #[cfg(target_os = "macos")]
    let url = "https://storage.googleapis.com/chrome-for-testing-public/136.0.7103.49/mac-arm64/chrome-mac-arm64.zip";

    #[cfg(target_os = "windows")]
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
            continue;
        }

        // Create the parent directories if they don't exist
        if let Some(parent) = outpath.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let mut out_file = std::fs::File::create(&outpath)?;
        std::io::copy(&mut file, &mut out_file)?;

        #[cfg(unix)]
        {
            let mut perms = out_file.metadata()?.permissions();
            perms.set_mode(0o777);
            out_file.set_permissions(perms);
        }
    }

    // #[cfg(target_os = "macos")]
    // let bin =
    //     "chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing";
    // #[cfg(target_os = "linux")]
    // let bin = "chrome-linux64/chrome";

    // #[cfg(unix)]
    // {
    //     use std::os::unix::fs::PermissionsExt;
    //     let file_path = spectre_dir.join(bin);
    //     let mut perms = std::fs::metadata(&file_path)?.permissions();
    //     perms.set_mode(0o755);
    //     fs::set_permissions(file_path, perms)?;
    // }

    Ok(())
}
