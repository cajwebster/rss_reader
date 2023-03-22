use std::io::BufRead;

fn main() {
    let project_dirs = directories::ProjectDirs::from("", "cajwebster", "RSS Reader")
        .expect("Could not find project directories");
    let config_dir = project_dirs.config_dir();

    std::fs::create_dir_all(config_dir).expect("Could not create config dir");

    let feeds_path = config_dir.join("feeds");
    let feeds = parse_feeds_file(
        std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&feeds_path)
            .unwrap_or_else(|e| panic!("Could not open feeds file {feeds_path:?}: {e}")),
    );

    println!("{feeds:?}");
}

fn parse_feeds_file(file: impl std::io::Read) -> Vec<(String, String)> {
    let file = std::io::BufReader::new(file);
    let mut feeds = vec![];

    for line in file.lines() {
        let line = line.expect("Failed to read line from feeds file");
        let (name, url) = line.split_once(':').expect("Invalid format for feeds file");
        feeds.push((name.to_string(), url.to_string()));
    }

    feeds
}
