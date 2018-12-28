use select::document::Document;
use select::predicate::Class;

#[macro_use]
extern crate clap;

use clap::{App};

use dotenv;
use envy;
use rayon::prelude::*;

#[macro_use] extern crate log;
use reqwest::Url;
use wape::config::Config;

fn get_thing(uri: &str) -> Result<String, reqwest::Error> {
    reqwest::get(uri)?.text()
}

// TODO: implement dotenv
// TODO: implement envy
// TODO: implement clap

pub fn main() -> Result<(), Box<std::error::Error>> {
    drop(dotenv::dotenv());
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let paper = value_t!(matches.value_of("paper"), String).unwrap_or_default();
    let mut config: Config;
    match envy::from_env::<Config>() {
        Ok(env_config) => {
            config = env_config;
        }
        Err(error) => panic!("{:#?}", error),
    }

    let url = Url::parse(&paper)?;
    debug!("{:?}", &url);
    let pre = url.host().expect("fuck").to_string();
    debug!("{:?}", &pre);
    let base_url = Url::parse(config.site.as_str())?;
    debug!("{:?}", &base_url);
    let thing = get_thing(url.as_str())?;
    let document = Document::from(thing.as_str());

    // TODO: Move this to some input via clap, etc

    let mut to_donwload = Vec::new();

    for node in document.find(Class("wallpaper-resolutions")) {
        node.descendants()
            .filter(|f| f.inner_html().contains('x') && config.sizes.contains(&f.text()))
            .for_each(|d| {
                // TODO: download each to folder
                // TODO: dotenv / clap folder for downloads
                debug!("{:?}", &d);
                let page = d.attr("href").unwrap();
                debug!("{:?}", &page);
                let full = reqwest::Url::join(&base_url, page).expect("fuck");
                trace!("{:?}", &full);
                info!("{}: {}", full, &d.text());
                to_donwload.push(full.to_string())
            });
    }

    to_donwload.par_iter().for_each(|w|{
        reqwest::get(w).and_then(|r|{
           Ok(println!("Downloaded: {:?}", &r.url().as_str().to_string()))
        }).expect("wioegn");
    });

    Ok(())
}
