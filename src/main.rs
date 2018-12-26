use select::document::Document;
use select::predicate::Class;

fn get_thing(uri: &str) -> Result<String, reqwest::Error> {
    reqwest::get(uri)?.text()
}

// TODO: implement dotenv
// TODO: implement envy
// TODO: implement clap

pub fn main() -> Result<(), Box<std::error::Error>> {
    // TODO: Move this to clap/dotenv something
    let list = vec!["1920x1080", "2560x1440", "3840x2160", "1366x768"];

    let document = Document::from(
        // TODO: Move this to some input via clap, etc
        get_thing("http://wallpaperswide.com/ama_dablam_mountain_nepal-wallpapers.html")?.as_str(),
    );

    for node in document.find(Class("wallpaper-resolutions")) {
        node.descendants()
            .filter(|f| f.inner_html().contains('x') && list.contains(&f.text().as_str()))
            .for_each(|d| {
                // TODO: add base url
                // TODO: download each to folder
                // TODO: dotenv / clap folder for downloads
                println!("{}: {}", d.attr("href").unwrap(), d.text());
            });
    }
    Ok(())
}
