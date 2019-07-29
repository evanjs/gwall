#![feature(todo_macro)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate url_serde;

use rayon::prelude::*;

pub mod config;

use config::Config;
use failure;
use tendril::stream::TendrilSink;
use std::clone::Clone;
use std::borrow::ToOwned;

#[macro_use]
extern crate log;

pub struct WallClient {
    client: reqwest::Client,
    config: config::Config,
    css_selector: String,
}

impl WallClient {
    pub fn new(config: Config, css_selector: &str) -> Self {
        WallClient {
            client: reqwest::Client::new(),
            config,
            css_selector: String::from(css_selector),
        }
    }

    pub fn get_wallpaper_name(&self, url: &url::Url) -> Result<String, failure::Error> {
        trace!("URL: {:#?}", url);
        let path = url.path_segments().expect("Failed to get path");
        let real_path = path
            .last()
            .expect("Failed to get next path segment")
            .to_string()
            .replace(".html", "")
            .replace("-wallpapers", "");
        info!("Real file path: {:#?}", &real_path);
        Ok(real_path)
    }

    pub fn get_full_wallpaper_url(&self, s: &str) -> reqwest::Url {
        self.config
            .site
            .join(s)
            .expect("Failed to parse uri for request.")
    }

    pub fn parse_url_and_get_request(&self, s: &str) -> String {
        let url = self.get_full_wallpaper_url(&s);
        let res = self
            .client
            .get(url)
            .send()
            .expect(format!("Failed to parse wallpaper url: {:?}", &s).as_str())
            .text();
        let body = format!(
            "{}",
            res.expect("Failed to process request body").to_string()
        );
        return body;
    }

    pub fn get_request(&self, url: &url::Url) -> String {
        let res = self
            .client
            .get(url.to_owned())
            .send()
            .expect("Failed to process url")
            .text();
        let body = format!(
            "{}",
            res.expect("Failed to process request body").to_string()
        );
        return body;
    }

    pub fn eval_css_selector(&self, text: &str) -> Vec<String> {
        let doc = kuchiki::parse_html().one(text);
        let css_match = doc.select(&self.css_selector).unwrap();
        let as_nodes = css_match.into_iter();
        let matches = as_nodes.filter(|x| self.config.sizes.contains(&x.text_contents()));
        matches
            .map(|x| x.attributes.borrow().get("href").unwrap().to_string())
            .collect()
    }

    pub fn create_download_folder(&self, name: std::path::PathBuf) -> std::path::PathBuf {
        match std::fs::read_dir(&name) {
            Ok(_o) => {
                panic!("Directory {:?} already exists! Aborting", &name);
            }
            Err(_e) => {
                debug!("Directory {:?} does not exist! Creating directory.", &name);
                std::fs::create_dir(&name)
                    .expect(format!("Failed to create directory for {:?}", &name).as_str());
                info!("Created directory {:?}", &name);
                name
            }
        }
    }
    pub fn download_images(
        &self,
        download_folder: std::path::PathBuf,
        downloads: &Vec<String>,
    ) -> Result<(), failure::Error> {
        downloads.par_iter().for_each(|w| {
            let download_url = self
                .config
                .site
                .join(w)
                .expect("Failed to join site url with image url");
            trace!("Download url: {}", &download_url);
            trace!("Download folder: {}", download_folder.display());
            self.download_image(download_url, download_folder.clone());
        });

        Ok(())
    }

    pub fn download_paper(&self, wallpaper_url: &str) -> Result<(), failure::Error> {
        let url = url::Url::parse(&wallpaper_url)?;
        let document = self.get_request(&url);
        let downloads = self.eval_css_selector(document.as_str());
        let name = self
            .get_wallpaper_name(&url)
            .expect("Failed to get wallpaper name");
        let joined_name = self.config.wallpaper_directory.join(name);
        let folder = self.create_download_folder(joined_name);

        self.download_images(folder, &downloads)
    }

    /// Downloads images to the given folder
    pub fn download_image(&self, url: reqwest::Url, path: std::path::PathBuf) {
        trace!("Downloading image from {}...", &url);
        let image_name = self
            .get_wallpaper_name(&url)
            .expect("Failed to get image name");
        let full_path = path.join(image_name);
        let mut resp = self
            .client
            .get(url.clone())
            .send()
            .expect(format!("Failed to get image for url {:?}", &url).as_str());
        let mut out = std::fs::File::create(&full_path)
            .expect(format!("Failed to create file {:?}", &full_path).as_str());
        std::io::copy(&mut resp, &mut out)
            .expect(format!("Failed to copy file to {:?}", &out).as_str());
        println!("Downloaded image from {}", &url);
    }
}
