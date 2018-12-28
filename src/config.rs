fn default_sizes() -> Vec<String> {
    vec![
        String::from("1920x1080"),
        String::from("2560x1440"),
        String::from("3840x2160"),
        String::from("1366x768"),
    ]
}

fn default_site() -> String {
    String::from("http://wallpaperswide.com/")
}
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_site")]
    pub site: String,
    #[serde(default = "default_sizes")]
    pub sizes: Vec<String>,
}
