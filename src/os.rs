use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::{env, fs};

/// Clipboard commands tried in order, the first one that succeeds wins.
const CLIPBOARDS: [&[&str]; 4] = [
    &["wl-copy"],
    &["xclip", "-selection", "clipboard"],
    &["pbcopy"],
    &["clip"],
];

/// Termux paths, also visible from proot-distro.
const TERMUX_HOME: &str = "/data/data/com.termux/files/home";
const TERMUX_CLIPBOARD: &[&str] = &["/data/data/com.termux/files/usr/bin/termux-clipboard-set"];

fn termux() -> bool {
    Path::new(TERMUX_HOME).is_dir() && Path::new(TERMUX_CLIPBOARD[0]).is_file()
}

fn spawn(argv: &[&str]) -> io::Result<Child> {
    Command::new(argv[0])
        .args(&argv[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}

/// Pipes `text` into `argv`, true when it exits cleanly.
fn pipe(argv: &[&str], text: &str) -> bool {
    spawn(argv).is_ok_and(|mut child| {
        let written = child
            .stdin
            .take()
            .is_some_and(|mut stdin| stdin.write_all(text.as_bytes()).is_ok());
        child.wait().is_ok_and(|status| written && status.success())
    })
}

pub fn copy(text: &str) -> io::Result<()> {
    CLIPBOARDS
        .into_iter()
        .chain(termux().then_some(TERMUX_CLIPBOARD))
        .any(|argv| pipe(argv, text))
        .then_some(())
        .ok_or_else(|| io::Error::other("no working clipboard command"))
}

fn home() -> String {
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_default()
}

/// XDG download dir, `$HOME/Downloads` (or `%USERPROFILE%`) when `xdg-user-dir` is missing.
fn downloads() -> PathBuf {
    Command::new("xdg-user-dir")
        .arg("DOWNLOAD")
        .output()
        .ok()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_owned())
        .filter(|dir| !dir.is_empty())
        .map_or_else(|| PathBuf::from(home()).join("Downloads"), PathBuf::from)
}

pub fn save(name: &str, bytes: &[u8]) -> io::Result<PathBuf> {
    let dir = downloads();
    fs::create_dir_all(&dir)?;
    let path = dir.join(name);
    fs::write(&path, bytes)?;
    Ok(path)
}
