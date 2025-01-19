use clap::Parser;
use std::{
    env, fs,
    fs::{read_to_string, write},
    io::{Error, ErrorKind},
    os::unix::fs::symlink,
    path::{Path, PathBuf},
    thread, time,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Identifierfile directory (contains current wallpaper location)
    #[arg(short = 'i', long = "wallpaper-id-dir")]
    id_dir: Option<String>,

    /// Directory with images
    #[arg(short = 'd', long = "img-directory")]
    image_directory: String,

    /// Interval between changing wallpapers (in seconds)
    #[arg(short, long, default_value_t = 120)]
    time: u16,
}

// Check if the passed id_dir is a valid directory.
// If no directory is passed, check for XDG_DATA_HOME and HOME.
// If the directory for wallpaper_controller does not exist, create it.
fn check_id_dir(args: &Args) -> Result<PathBuf, Error> {
    fn is_valid_dir(path: &str) -> Option<PathBuf> {
        let path_buf = PathBuf::from(path);
        if path_buf.is_dir() {
            Some(path_buf)
        } else {
            None
        }
    }

    if let Some(dir) = &args.id_dir {
        if let Some(valid_dir) = is_valid_dir(dir) {
            return Ok(valid_dir);
        }
    }

    if let Ok(xdg_data_home) = env::var("XDG_DATA_HOME") {
        let path = PathBuf::from(xdg_data_home).join("wallpaper_controller");
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        if path.is_dir() {
            return Ok(path);
        }
    }

    if let Ok(home) = env::var("HOME") {
        let path = PathBuf::from(home).join(".wallpaper_controller");
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        if path.is_dir() {
            return Ok(path);
        }
    }

    Err(Error::new(
        ErrorKind::NotFound,
        "No XDG directory found ($HOME or $XDG_DATA_HOME) or it is not a directory",
    ))
}

// Write the next image to the current_wallpaper file and create a symlink
fn write_next_image(image_dir: &Path, current_wallpaper_file: &Path) -> Result<String, Error> {
    fn write_and_link(image_id_file: &Path, current_wallpaper: &str) {
        // Write the content to the file
        write(image_id_file, current_wallpaper).expect("Failed to write to current_wallpaper file");

        // Create a symbolic link to the file
        let symlink_path = image_id_file.with_extension("symlink");
        if symlink_path.exists() {
            fs::remove_file(&symlink_path).expect("Failed to remove symlink file");
        }
        symlink(current_wallpaper, symlink_path).expect("Failed to create symlink");
    }

    let dir = fs::read_dir(image_dir).expect("Failed to read image directory");

    let entries: Vec<PathBuf> = dir
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && matches!(
                    path.extension().and_then(|ext| ext.to_str()),
                    Some("jpg" | "png" | "jpeg")
                )
        })
        .collect();

    let current_wallpaper = read_to_string(current_wallpaper_file);
    println!("{:?}", current_wallpaper);

    let i = entries.iter().position(|entry| {
        if let Ok(current_wallpaper) = &current_wallpaper {
            if let Some(entry) = entry.to_str() {
                return entry == current_wallpaper;
            }
        }
        false
    });

    if let Some(i) = i {
        let next = if i + 1 < entries.len() {
            &entries[i + 1]
        } else {
            &entries[0]
        };

        let next = next
            .to_str()
            .expect("next image path of image_dir is not valid");

        write_and_link(current_wallpaper_file, next);

        return Ok(next.to_owned());
    }

    if let Some(first) = entries.first() {
        let first = first
            .to_str()
            .expect("first image path of image_dir is not valid");

        write_and_link(current_wallpaper_file, first);

        return Ok(first.to_owned());
    }

    Err(Error::new(ErrorKind::NotFound, "No next image file found"))
}

fn main() {
    let args = Args::parse();

    loop {
        let id_dir = match check_id_dir(&args) {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        };

        match write_next_image(
            Path::new(&args.image_directory),
            &id_dir.join("current_wallpaper"),
        ) {
            Ok(next_img_path) => {
                let _ = std::process::Command::new("swww")
                    .arg("img")
                    .arg(next_img_path)
                    .spawn();
            }
            Err(e) => eprintln!("Error: {}", e),
        };

        let now = time::Instant::now();
        let interval_in_ms = time::Duration::from_secs(args.time.into());
        thread::sleep(interval_in_ms);

        assert!(now.elapsed() >= interval_in_ms);
    }
}
