#![allow(dead_code)]
#![feature(map_first_last)]
mod rman;
use ureq;

fn main() -> Result<(), String> {
    let dir = "/home/jesus/tmp/lol";
    let url = "/home/jesus/tmp/lol/manifest.manifest";
    let mut agent = ureq::AgentBuilder::new().build();
    let man = rman::Manifest::download(&mut agent, url)?;
    for file in &man.files {
        if !file.langs.contains("none") {
            continue;
        }
        if file.verify(dir) {
            println!("{} is correct!", &file.name);
        } else {
            println!("{} is bad!", &file.name);
        }
    }
    Ok(())
}

