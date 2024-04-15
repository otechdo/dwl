use inquire::{prompt_confirmation, MultiSelect, Select, Text};
use notifme::Notification;
use std::fs;
use std::path::Path;
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
                return p.to_vec();
            }
            Ok(false) | Err(_) => {
                continue;
            }
        }
    }
}
fn check(codec: &str) {
    let p = format!("{}/Records/{codec}", env!("HOME"));
    if !Path::new(p.as_str()).exists() {
        fs::create_dir(p.as_str()).expect("Failed to create codec directory");
    }
}

fn youtube(songs: &mut Vec<String>, codecs: &mut [&str], bitrate: &str) {
    for codec in codecs {
        check(codec);
        let l = songs.len();
        if l.gt(&1) {
            assert!(Notification::new()
                .app("spot_dwl")
                .summary("Spotify Downloader")
                .body(format!("Started to download {l} queries from youtube").as_str())
                .icon("spot_dwl")
                .send());
        } else {
            assert!(Notification::new()
                .app("spot_dwl")
                .summary("Spotify Downloader")
                .body(format!("Started to download {l} query from youtube").as_str())
                .icon("spot_dwl")
                .send());
        }
        assert!(Command::new("yt-dlp")
            .arg("--ignore-config")
            .arg("--extractor-args")
            .arg("youtubetab:skip=authcheck")
            .arg("--extract-audio")
            .arg("--no-config-locations")
            .arg("--break-on-existing")
            .arg("--write-thumbnail")
            .arg("--cookies")
            .arg(format!("{}/Records/cookies.txt", env!("HOME")).as_str())
            .arg("--no-sponsorblock")
            .arg("--audio-quality")
            .arg(bitrate)
            .arg("--audio-format")
            .arg(&codec)
            .arg("--output")
            .arg(format!("{}/Records/{codec}/{}", env!("HOME"),"%(playlist)s/%(title)s.%(ext)s").as_str())
            .arg("--keep-video") 
            .args(&mut *songs)
            .spawn()
            .expect("yt-dlp not founded")
            .wait()
            .expect("msg")
            .success());
    }
}

fn spotify(songs: &mut [String], codecs: &mut [&str], bitrate: &str) {
    for codec in codecs {
        check(codec);
        assert!(Command::new("spotdl")
            .arg("--audio")
            .arg("youtube-music")
            .arg("youtube")
            .arg("soundcloud")
            .arg("--headless")
            .arg("--bitrate")
            .arg(bitrate)
            .arg("--format")
            .arg(codec.to_string().as_str())
            .arg("--preload")
            .arg("--output")
            .arg(
                format!(
                    "{}/Records/{}/{}",
                    env!("HOME"),
                    codec,
                    "{artist}/{album}/{title}.{output-ext}"
                )
                .as_str()
            )
            .arg("--cookie-file")
            .arg(format!("{}/Records/cookies.txt", env!("HOME")).as_str())
            .arg("download")
            .args(&mut *songs)
            .spawn()
            .expect("spotdl not founded")
            .wait()
            .expect("msg")
            .success());
    }
}
fn main() -> ExitCode {
    if !Path::new(format!("{}/Records", env!("HOME")).as_str()).exists() {
        fs::create_dir(format!("{}/Records", env!("HOME")).as_str())
            .expect("Failed to create Records directory")
    }
    let mut provider = String::new();
    let mut bitrate = String::new();
    let mut codecs: Vec<&str> = Vec::new();
    let mut musics: Vec<String> = Vec::new();
    loop {
        let p = Select::new("Select a provider : ", vec!["youtube", "spotify"])
            .prompt()
            .unwrap();
        if !p.is_empty() {
            provider.clear();
            provider.push_str(p);
            break;
        } else {
            continue;
        }
    }
    loop {
        let c: Vec<&str> = MultiSelect::new(
            "Select codecs : ",
            vec![
                "mp3", "flac", "ogg", "opus", "m4a", "wav", "alac", "aac", "vorbis",
            ],
        )
        .prompt()
        .unwrap();

        if !c.is_empty() {
            codecs.clear();
            codecs = c;
            break;
        } else {
            continue;
        }
    }
    loop {
        let b: &str = Select::new(
            "Select a bitrate : ",
            vec![
                "auto", "disable", "8k", "16k", "24k", "32k", "40k", "48k", "64k", "80k", "96k",
                "112k", "128k", "160k", "192k", "224k", "256k", "320k",
            ],
        )
        .prompt()
        .unwrap();
        if !b.is_empty() {
            bitrate.clear();
            bitrate.push_str(b);
            break;
        } else {
            continue;
        }
    }

    if provider.eq("spotify") {
        spotify(&mut capture(&mut musics, "spotify"), &mut codecs, &bitrate);
    } else {
        youtube(&mut capture(&mut musics, "youtube"), &mut codecs, &bitrate);
    }

    exit(0);
}
