use std::path::{PathBuf, Path};

use std::str::FromStr;
use url::Url;
use directories::UserDirs;

fn default_site() -> Url {
    Url::from_str("http://wallpaperswide.com/").expect("Failed to get URL from string")
}

fn default_sizes() -> Vec<String> {
    vec![
        String::from("1920x1080"),
        String::from("2560x1440"),
        String::from("3840x2160"),
        String::from("1366x768"),
    ]
}

fn default_wallpaper_directory() -> PathBuf {
    if let Some(user_dirs) = UserDirs::new() {
        Path::join(user_dirs.picture_dir().expect("aaa"), "Wallpapers")
    } else {
        todo!("Check for user configured path here");
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(with = "url_serde", default = "default_site")]
    pub site: url::Url,
    #[serde(default = "default_sizes")]
    pub sizes: Vec<String>,
    #[serde(default = "default_wallpaper_directory")]
    pub wallpaper_directory: PathBuf,
}
