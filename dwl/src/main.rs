#![allow(clippy::multiple_crate_versions)]

use indicatif::{ProgressBar, ProgressStyle};
use inquire::{Confirm, Text};
use std::{
    fs::{remove_file, File},
    path::Path,
    process::Command,
    time::Duration,
};

fn ask(p: &str) -> String {
    loop {
        let data: String = Text::new(p).prompt().unwrap();
        if data.is_empty() {
            continue;
        }
        return data.trim().to_string();
    }
}

fn capture(p: &mut Vec<String>) {
    loop {
        p.push(ask("Please enter the Spotify url : "));
        match Confirm::new("Download ? : ").with_default(false).prompt() {
            Ok(true) => {
                break;
            }
            Ok(false) | Err(_) => {
                continue;
            }
        }
    }
}

fn download(song: &str) -> i32 {
    if Path::new("/tmp/dwl-stdout").is_file() {
        assert!(remove_file("/tmp/dwl-stdout").is_ok());
    }
    if Path::new("/tmp/dwl-stderr").is_file() {
        assert!(remove_file("/tmp/dwl-stderr").is_ok());
    }

    let e: File = File::create("/tmp/dwl-stderr").expect("msg");
    let o: File = File::create("/tmp/dwl-stdout").expect("msg");
    let _ = Command::new("spotdl")
        .arg(song)
        .stdout(o)
        .stderr(e)
        .spawn()
        .expect("spotdl not founded")
        .wait()
        .expect("msg")
        .success();
    0
}

fn main() {
    let mut musics: Vec<String> = Vec::new();
    capture(&mut musics);
    let pb: ProgressBar = ProgressBar::new(musics.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.white} [ {percent}% ] [{bar:80.white}] {msg}")
            .expect("aaaa")
            .progress_chars("=-"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    for music in &musics {
        pb.set_message(music.to_string());
        assert_eq!(download(music.as_str()), 0);
        pb.inc(1);
    }
    pb.finish_with_message(format!("{} queries downloaded successfully", musics.len()));
    if Path::new("/tmp/dwl-stdout").is_file() {
        assert!(File::create("/tmp/dwl-stdout").is_ok());
    }
    if Path::new("/tmp/dwl-stderr").is_file() {
        assert!(File::create("/tmp/dwl-stderr").is_ok());
    }
}
