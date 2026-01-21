use std::fs;
use std::path::Path;
use std::process::Command;
use std::{thread, time};
use rand::Rng;
mod config;
use config::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    loop{
        let mut entries: Vec<_> = fs::read_dir(wallpapers_dir)?.filter_map(|e| e.ok()).collect();
        
        for entry in &entries{
        
                
            let wallpaper = entry.path();

            if let Some(path) = wallpaper.to_str(){
                Command::new("feh").args(&["--bg-fill",path]).output().expect("err");
                println!("installed wallpaper {}",path);
            }
        thread::sleep(time::Duration::from_secs(interval));
        }
    }
}
