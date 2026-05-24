use super::{ModrinthIndex, get_app_dir, read_modrinth_index};
use std::error::Error;

pub fn download_modrinth_modpack(index: ModrinthIndex) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .user_agent("JorgesLauncherMC (https://github.com/PrimatasDev/JorgesLauncherMC)")
        .build()?;

    Ok(())
}
