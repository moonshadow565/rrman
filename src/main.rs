#![allow(dead_code)]
#![feature(map_first_last)]
mod rman;
use ureq;
use rayon::prelude::*;

fn main() -> Result<(), String> {
    let dir = "/home/jesus/tmp/lol";
    let url = "/home/jesus/tmp/lol/manifest.manifest";
    let mut agent = ureq::AgentBuilder::new().build();
    let man = rman::Manifest::download(&mut agent, url)?;
    man.files.par_iter().for_each(|file| {
        if !file.langs.contains("none") {
            return;
        }
        if file.verify(dir) {
            println!("{} is correct!", &file.name);
        } else {
            println!("{} is bad!", &file.name);
        }
    });
    Ok(())
}

