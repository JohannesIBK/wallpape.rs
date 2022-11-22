
# wallpape.rs [![crate](https://img.shields.io/crates/v/wallpape-rs.svg)](https://crates.io/crates/wallpape-rs) [![docs](https://docs.rs/wallpape-rs/badge.svg)](https://docs.rs/wallpape-rs)

This Rust library gets and sets the desktop wallpaper/background.

The supported desktops are:

- Windows
- macOS
- GNOME
- KDE
- Cinnamon
- Unity
- Budgie
- XFCE
- LXDE
- MATE
- Deepin
- Most Wayland compositors (set only, requires swaybg)
- i3 (set only, requires feh)

## Examples

```rust
use wallpape_rs as wallpaper;

fn main() {
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper::get());
    // Sets the wallpaper for the current desktop from a file path.
    wallpaper::set_from_path("/usr/share/backgrounds/gnome/adwaita-day.png").unwrap();
    // Sets the wallpaper style.
    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper::get());
}
```

If you want to set an image as background via an URL, make sure you activated the `from_url` feature of the wallpaper crate on Cargo.toml:

```toml
[dependencies]
wallpape-rs = { version = "1.0.0", features = ["from_url"] }
```

Then, on your main.rs:

```rust
use wallpape_rs as wallpaper;

fn main() {
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper::get());
    // Sets the wallpaper for the current desktop from a URL.
    wallpaper::set_from_url("https://source.unsplash.com/random").unwrap();
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper::get());
}
```
