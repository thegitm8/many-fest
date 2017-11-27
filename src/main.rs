#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::env;
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Msconfig {
    start: String,
    stop: String,
    prepare: String,
    release: String,
    deploy: String,
}

fn read_config<P: AsRef<Path>>(path: P) -> Result<Msconfig, Box<Error>> {

    let file = File::open(path)?;
    let config = serde_json::from_reader(file)?;

    Ok(config)
}

fn main() {

    let conf = read_config("Manyfest").unwrap();
    let escaped_cmd = &conf.start;
    let cmd_with_args: Vec<&str> = escaped_cmd.split_whitespace().collect();
    let cmd = cmd_with_args[0];
    let args = &cmd_with_args[1..cmd_with_args.len()];
    let envs: HashMap<String, String> = env::vars().collect();

    Command::new(cmd)
        .args(args)
        .envs(&envs)
        .status()
        .expect("failed to execute child");
}