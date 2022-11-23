use std::borrow::Borrow;
use crate::{Mode, Result};
use std::ffi::{c_void, OsStr};
use std::io;
use std::iter;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use windows::core::{PCWSTR};
use windows::w;
use windows::Win32::UI::WindowsAndMessaging::{SPI_SETDESKWALLPAPER, SPI_GETDESKWALLPAPER, SPIF_UPDATEINIFILE, SPIF_SENDCHANGE};
use windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoW;
use windows::Win32::System::Registry::{HKEY_CURRENT_USER, REG_SZ, REGSTR_PATH_DESKTOP, HKEY, RegSetValueW, RegCreateKeyW, REGSTR_PATH_LOOKSCHEMES, RegSetValueExW};

#[cfg(feature = "from_url")]
use crate::download_image;

/// Returns the current wallpaper.
pub fn get() -> Result<String> {
    unsafe {
        let buffer: [u16; 260] = mem::zeroed();
        let successful = SystemParametersInfoW(
            SPI_GETDESKWALLPAPER,
            buffer.len() as u32,
            Option::from(buffer.as_ptr() as *mut c_void),
            SPIF_UPDATEINIFILE,
        ) == true;

        if successful {
            let path = String::from_utf16(&buffer)?
                // removes trailing zeroes from buffer
                .trim_end_matches('\x00')
                .into();
            Ok(path)
        } else {
            Err(io::Error::last_os_error().into())
        }
    }
}

/// Sets the wallpaper from a file.
pub fn set_from_path(path: &str) -> Result<()> {
    unsafe {
        let path = OsStr::new(path)
            .encode_wide()
            // append null byte
            .chain(iter::once(0))
            .collect::<Vec<u16>>();
        let successful = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Option::from(path.as_ptr() as *mut c_void),
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        ) == true;

        if successful {
            Ok(())
        } else {
            Err(io::Error::last_os_error().into())
        }
    }
}

/// Sets the wallpaper from a URL.
#[cfg(feature = "from_url")]
pub fn set_from_url(url: &str) -> Result<()> {
    let path = download_image(url)?;
    set_from_path(&path)
}

/// Sets the wallpaper style.
pub fn set_mode(mode: Mode) -> Result<()> {
    let tile_val = match mode {
        Mode::Tile => "1",
        _ => "0",
    }.to_string().as_bytes();

    let style_val = match mode {
        Mode::Center | Mode::Tile => "0",
        Mode::Fit => "6",
        Mode::Span => "22",
        Mode::Stretch => "2",
        Mode::Crop => "10",
    }.to_string().as_bytes();

    unsafe {
        let mut h_key: HKEY = Default::default();
        RegCreateKeyW(HKEY_CURRENT_USER, REGSTR_PATH_DESKTOP, &mut h_key);
        RegSetValueExW(h_key, w!("TileWallpaper"), 0, REG_SZ, Option::from(tile_val));
        RegSetValueExW(h_key, w!("WallpaperStyle"), 0, REG_SZ, Option::from(style_val));

        // updates wallpaper
        set_from_path(&get()?)
    }
}
