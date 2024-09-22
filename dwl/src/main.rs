#![allow(clippy::multiple_crate_versions)]

use indicatif::{ProgressBar, ProgressStyle};
use inquire::{Confirm, Text};
use std::{
    fs::File,
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
    let e: File = File::create("/tmp/dwl-stderr").expect("msg");
    let o: File = File::create("/tmp/dwl-stdout").expect("msg");
    if let Ok(mut child) = Command::new("spotdl").arg("--config").arg(song).stdout(o).stderr(e).spawn() {
        if let Ok(status) = child.wait() {
           return  if status.success() {
               0
            }else{
                1
            };
        }
        return 1;
    }
    1
}

fn main() {
    let mut musics: Vec<String> = Vec::new();
    capture(&mut musics);
    let pb: ProgressBar = ProgressBar::new(musics.len() as u64);
    if let Ok(template) = ProgressStyle::with_template("{spinner:.white} [ {percent}% ] [{bar:80.white}] {msg}") {
        pb.set_style(template.progress_chars("==-"));
    }
    pb.enable_steady_tick(Duration::from_millis(100));
    let mut success : usize = 0;
    let mut failures: usize = 0;
    for music in &musics {
        pb.set_message(format!("Start to download {music}"));
        if download(music.as_str()).eq(&0) {
            pb.set_message(format!("{music} has been successfully downloaded"));
            success +=1;
            pb.inc(1);
        }else {
            pb.set_message(format!("{music} failed to download the query"));
            failures +=1;
            pb.inc(1);
            continue;
        }
    }
    pb.finish_with_message(format!("Success : {success}/{} Failure : {failures}/{}", musics.len(),musics.len()));
}
