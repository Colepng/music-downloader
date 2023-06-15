use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // chose to start with a capacity of 100 because the chances someone will add over 100 links is
    // a unlicky and this way it won't have to shift increase it's capacity every time it gets
    // pushed to
    let mut links: Vec<String> = Vec::with_capacity(100);
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        let mut buf: String = String::new();
        let _ = stdin.read_line(&mut buf);
        if buf.trim() != "".to_string() {
            links.push(buf.trim().to_string());
        } else {
            break;
        }
    }
    let path = match home::home_dir() {
        Some(mut path) => {
            path.push("Music");
            println!("Saving media in {}", path.display());
            Some(path.clone())
        }
        None => {
            println!("could not find home dir, defauling to current_dir");
            None
        }
    };

    for link in links {
        let mut spotdl = Command::new("spotdl");
        if let Some(ref path) = path {
            spotdl.current_dir(path);
        }
        spotdl
            .arg(format!("{link}"))
            .arg("--config")
            .status()
            .expect("faild to run spotdl");
    }

    print!("Would you like run python script Y/n: ");
    let _ = stdout.flush();
    let mut answer: String = String::new();
    let _ = stdin.read_line(&mut answer);
    answer.make_ascii_lowercase();
    if answer.trim() != "n" {
        let mut python = Command::new("python");
        if let Some(ref path) = path {
            python.current_dir(path);
        }
        python
            .arg("music.py")
            .status()
            .expect("failed to run script");
    }
}
