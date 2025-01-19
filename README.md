# Wallpaper Controller

This is a simple application that changes the desktop wallpaper at regular intervals. It uses [swww](https://github.com/LGFae/swww) as wallpaper engine. It also keeps track
of the current wallpaper location using XDG directory standards.


## Usage

### Command-line Arguments

- `-i, --wallpaper-id-dir <ID_DIR>`: Directory containing the current wallpaper location. If not provided, it will use `$XDG_DATA_HOME/wallpaper_controller` or `$HOME/.wallpaper_controller`.
- `-d, --img-directory <IMG_DIRECTORY>`: Directory containing the images to be used as wallpapers. This argument is required.
- `-t, --time <TIME>`: Interval between changing wallpapers in seconds. Default is 120 seconds.

### Example

```sh
./wallpaper_controller -d /path/to/images -t 300
```

## Dependencies

- `swww`: Command-line tool to set wallpapers.

## Build

1. Ensure you have Rust installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone this repository.
3. Navigate to the project directory and run:

```sh
cargo build --release
```
