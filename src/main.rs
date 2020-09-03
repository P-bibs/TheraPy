use structopt::StructOpt;
extern crate clap_verbosity_flag;
use colored::*;
use std::process::Command;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    // The pattern to look for
    // pattern: String,
}

fn main() {
    let args = Cli::from_args();
    let mut child = Command::new("python")
        .arg(&args.path)
        .spawn()
        .expect("Couldn't spawn process");

    let ecode = child.wait().expect("failed to wait on child");

    if !ecode.success() {
        println!("\n{}", "Error -41: Sit by a lake.".cyan().bold())
    }
}
