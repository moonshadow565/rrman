#![allow(dead_code)]
#![feature(map_first_last)]
mod rman;
use ureq;

fn main() -> Result<(), String> {
    //let cdn = "http://0.0.0.0:8000";
    let cdn = "http://lol.secure.dyn.riotcdn.net/channels/public/bundles";
    let dir = "/home/jesus/tmp/test";
    let url = "/home/jesus/tmp/test.manifest";
    let mut agent = ureq::AgentBuilder::new().build();
    let man = rman::Manifest::download(&mut agent, url)?;
    for file in &man.files {
        if !file.name.ends_with(".dll") && !file.name.ends_with(".exe") {
            continue;
        }
        let download = file.download_checked_in_dir(dir);
        if let Err(err) = download.download_in_dir(dir, &mut agent, cdn) {
            println!("FAIL: {} because: {}", file.name, err);
        } else {
            if download.bundles.len() == 0 {
                println!("SKIPED: {}", file.name);
            } else {
                println!("DONE: {}", file.name);
            }
        }
    }
    Ok(())
}

