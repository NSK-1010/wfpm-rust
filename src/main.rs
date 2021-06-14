extern crate flate2;
extern crate tar;
extern crate anyhow;
extern crate tokyo;
extern crate reqwest;
extern crate helix;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
#[cfg(unix)]
use std::os::unix;
#[cfg(windows)]
use std::os::windows;
use std::path::Path;
use anyhow::Result;
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;

let get_url;
let save_dir;
let save_file

#[tokio::main]
async fn fileget(get_url: &str, save_dir: &str) -> Result<()> {
    let save_file = url.split("/").last().unwrap();
    let response: reqwest::Response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    let mut out = File::create(save_dir + "/" + save_file)?;
    io::copy(&mut bytes.as_ref(), &mut out)?;

    Ok(())
}

let targzpath;

fn targz_unpack(targzpath: &str) -> Result<(), std::io::Error> {
    let tar_gz = File::open(targzpathpath)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}

//hello
#[cfg(target_os = "windows")]
fn hello() {
    println!("Windows");
}
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly", target_os = "openbsd", target_os = "netbsd"))]
fn hello(){
    println!("Linux or BSD");
}
#[cfg(target_os = "macos")]
fn hello() {
    println!("Mac OS");
}

//module
#[cfg(target_os = "windows")]
mod windows;
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly", target_os = "openbsd", target_os = "netbsd"))]
mod linux_bsd;
#[cfg(target_os = "macos")]
mod macos;

fn main() {
    hello();
}