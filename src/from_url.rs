use crate::Result;
use std::fs::File;

pub fn download_image(url: &str) -> Result<String> {
    let cache_dir = dirs::cache_dir().ok_or("no cache dir")?;
    let file_path = cache_dir.join("wallpaper");

    let file = File::create(&file_path)?;
    attohttpc::get(url).send()?.write_to(file)?;

    Ok(file_path.to_str().to_owned().ok_or("no file path")?.into())
}
