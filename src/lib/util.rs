//use serde::Deserialize;
//use std::process::Command;

use std::error::Error;
//use std::fs::File;
//use std::io::BufReader;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

const CONFIG_PATH: &str = "../../.fprc.json";

struct Task {
    alias: String,
    path: PathBuf,
    command: String,
}

#[derive(StructOpt, Debug)]
pub enum CommandList {
    /// Add a task
    Add {
        /// Name for task
        alias: String,

        /// Command to run, surround in quotes
        command: String,

        /// Command to run, surround in quotes
        #[structopt(short)]
        path: Option<String>,
    },

    /// Remove a task
    Remove {
        alias: String,
    },

    /// List tasks
    List {

    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "Firing Phasers", about = "Like Warp Drive but for tasks")]
pub struct FirePhasers {
    #[structopt(subcommand)]
     pub cmd: CommandList,
}

pub fn add_task(alias: &String, command: &String, path: String) {
    println!("{}\n{}\n{}", alias, command, path);
}

//pub fn parse_opts(opts: &CommandList) -> Result<Task, Box<dyn Error>> {
//
//    println!("args: {:?}", opts);
//
// //   let task: Task {
// //   }
//
//
//    return Ok(task);
//}
