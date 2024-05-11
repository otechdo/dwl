#![allow(clippy::multiple_crate_versions)]
use inquire::{prompt_confirmation, Select, Text};
use std::process::{exit, Command, ExitCode};

fn ask(p: &str) -> String {
    loop {
        let data: String = Text::new(p).prompt().unwrap();
        if data.is_empty() {
            continue;
        }
        return data;
    }
}

fn capture(p: &mut Vec<String>, provider: &str) -> Vec<String> {
    loop {
        p.push(ask(format!("Please enter the {provider} url : ").as_str()));
        match prompt_confirmation("Download ? : ") {
            Ok(true) => {
                return p.clone();
            }
            Ok(false) | Err(_) => {
                continue;
            }
        }
    }
}

fn youtube(songs: &mut Vec<String>) -> i32 {
    for song in songs {
        assert!(Command::new("yt-dlp")
            .arg(song.as_str())
            .spawn()
            .expect("yt-dlp not founded")
            .wait()
            .expect("msg")
            .success());
    }
    0
}

fn spotify(songs: &mut [String]) -> i32 {
    for song in songs {
        assert!(Command::new("spotdl")
            .arg(song.as_str())
            .spawn()
            .expect("spotdl not founded")
            .wait()
            .expect("msg")
            .success());
    }
    0
}

fn main() -> ExitCode {
    let mut provider = String::new();
    let mut musics: Vec<String> = Vec::new();
    loop {
        let p = Select::new("Select a provider : ", vec!["youtube", "spotify"])
            .prompt()
            .unwrap();
        if !p.is_empty() {
            provider.clear();
            provider.push_str(p);
            break;
        }
    }
    if provider.eq("spotify") {
        exit(spotify(&mut capture(&mut musics, "spotify")));
    } else {
        exit(youtube(&mut capture(&mut musics, "youtube")));
    }
}
