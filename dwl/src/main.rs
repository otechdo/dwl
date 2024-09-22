#![allow(clippy::multiple_crate_versions)]

use indicatif::{ProgressBar, ProgressStyle};
use inquire::{Confirm, Text};
use std::cell::Cell;
use std::thread::sleep;
use std::{fs::File, process::Command, time::Duration};

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
    if let Ok(mut child) = Command::new("spotdl")
        .arg("--config")
        .arg(song)
        .stdout(o)
        .stderr(e)
        .spawn()
    {
        if let Ok(status) = child.wait() {
            return if status.success() { 0 } else { 1 };
        }
        return 1;
    }
    1
}

fn pause(pb: &ProgressBar) {
    for x in 1..=60 {
        let y = 60 - x;
        pb.set_message(y.to_string());
        sleep(Duration::from_secs(1));
    }
}
fn main() {
    let mut musics: Vec<String> = Vec::new();
    capture(&mut musics);
    let pb: ProgressBar = ProgressBar::new(musics.len() as u64);
    if let Ok(template) =
        ProgressStyle::with_template("{spinner:.white} [ {percent}% ] [{bar:80.white}] {msg}")
    {
        pb.set_style(template.progress_chars("==-"));
    }
    pb.enable_steady_tick(Duration::from_millis(100));
    let success: Cell<usize> = Cell::new(0);
    let failure: Cell<usize> = Cell::new(0);
    let size = musics.len();
    for (id, query) in musics.into_iter().enumerate() {
        pb.set_message(query.to_string());
        if download(query.as_str()).eq(&0) {
            pb.set_message(format!(
                "Query #{id} {query} has been downloaded successfully"
            ));
            success.set(success.get() + 1);
            pb.inc(1);
            if pb.is_finished() {
                break;
            }
            pause(&pb);
        } else {
            pb.set_message(format!("Query #{id} {query} has been failed"));
            failure.set(failure.get() + 1);
            if pb.is_finished() {
                break;
            }
            for i in 1..=3 {
                if pb.is_finished() {
                    break;
                }
                pause(&pb);
                pb.set_message(format!("Retry to download {query} retry id #{i}"));
                if download(query.as_str()).eq(&0) {
                    failure.set(failure.get() - 1);
                    success.set(success.get()  + 1);
                    pb.set_message(format!(
                        "Successfully downloaded the {query} query at the #{i} retry",
                    ));
                    break;
                }
            }
            pb.inc(1);
        }
    }
    pb.finish_with_message(format!(
        "Success : {}/{size} Failure : {}/{size}",
        success.get(),
        failure.get()
    ));
}
