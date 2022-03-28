mod commands;

use std::path::{Path, PathBuf};
use std::str::FromStr;

use commands::Command;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Depploy",
    about = "Create docker image of cargo project with Depploy."
)]
#[derive(Debug)]
pub struct Depploy {
    #[structopt(subcommand)]
    pub cmd: Command,
}

fn main() {
    let depploy_dir =  PathBuf::from_str("/etc/depploy").unwrap();
    let cli = Depploy::from_args();

    match &cli.cmd {
        Command::init {  } => {            
        }
        Command::create {dir , debug} => {}
    }

}