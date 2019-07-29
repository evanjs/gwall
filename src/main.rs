#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use clap::App;
use dotenv;
use envy;

use gwall::config::Config;
use gwall::WallClient;

pub fn main() -> Result<(), failure::Error> {
    drop(dotenv::dotenv());
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let paper = value_t!(matches.value_of("paper"), String)?;

    let config: Config;
    match envy::from_env::<Config>() {
        Ok(env_config) => {
            config = env_config;
        }
        Err(error) => panic!("{:#?}", error),
    }

    let dir = value_t!(matches.value_of("wallpaper_directory"), std::path::PathBuf)
        .unwrap_or_else(|_| config.wallpaper_directory.to_owned());
    let css_selector = ".wallpaper-resolutions > a[target=\"_self\"]";

    let client = WallClient::new(config, css_selector);

    // TODO: Move this to some input via clap, etc

    client.download_paper(&paper);

    Ok(())
}
