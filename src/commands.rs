use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(about = "Run depploy")]
pub enum Command {
    #[structopt(about = "Create a new template")]
    create{
        #[structopt(parse(from_os_str), default_value = ".")]
        dir: PathBuf,
        #[structopt(short = "v")]
        debug: bool,
    },
    #[structopt(about = "New project from template")]
    init{

    }
}