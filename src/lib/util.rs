use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Commands {
    /// Add a task
    Add {
        /// Name for task
        alias: String,

        /// Command to run, surround in quotes
        cmd: String,

        /// Command to run, surround in quotes
        #[structopt(short)]
        #[structopt(parse(from_os_str))]
        path: Option<PathBuf>,
    },

    /// Remove a task
    Remove {
        alias: String,
    },

    /// List tasks
    List {}
}

#[derive(StructOpt, Debug)]
#[structopt(name = "Firing Phasers", about = "Like Warp Drive but for tasks")]
pub struct FirePhasers {
    #[structopt(subcommand)]
    cmd: Commands,
}
