use clap::{crate_version, App, Arg};
use ropey::Rope;
use std::fs::{metadata, OpenOptions};
use std::io::BufReader;

pub fn argparser() -> Option<String> {
    let matches = App::new("ReVim")
        .version(crate_version!())
        .author("Cowboy8625 <cowboy8625@protonmail.com>")
        .about("Cross Platform Small Simple Terminal Text Editor Written In Rust")
        .arg(
            Arg::with_name("in_file")
                .index(1)
                .help("Input file to open"),
        )
        .after_help(
            "Longer explanation to appear after the options when \
                                         displaying the help information from --help or -h",
        )
        .get_matches();

    matches.value_of("in_file").map(|v| v.to_string())
}

pub fn from_path(path: Option<String>) -> (Rope, Option<String>) {
    let text = path
        .as_ref()
        .and_then(|path| {
            if metadata(&path).is_ok() {
                let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(path)
                    .expect("Problem opening the file");

                Rope::from_reader(BufReader::new(file)).ok()
            } else {
                None
            }
        })
        .unwrap_or_else(Rope::new);

    (text, path)
}
