use anyhow::Result;
use flexi_logger::{Duplicate, Logger};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;

mod settings;
mod stack;

use crate::settings::Settings;
use crate::stack::Stack;

#[derive(Debug, StructOpt)]
struct Cli {
    /// The location of the settings files
    #[structopt(
        short = "s",
        long = "settings-dir",
        default_value = "settings",
        parse(from_os_str)
    )]
    settings_dir: PathBuf,
}

fn main() -> Result<()> {
    // Parse command line arguments
    let args = Cli::from_args();
    println!("Args: {:?}", args);

    // Fetch current stack
    let stack = match Stack::from_env() {
        Ok(s) => Some(s),
        Err(e) => {
            println!("Couldn't fetch stack from env, defaulting to None: {:?}", e);
            None
        }
    };
    println!("Stack: {:?}", stack);

    // Load settings from file
    let settings = Settings::new(&args.settings_dir, stack.clone())?;
    println!("Settings: {:?}", settings);

    // Get location of logs, make sure we substitute any env vars in the path
    let log_dir = envsubst::substitute(
        settings.logging.log_dir.to_str().unwrap(),
        &env::vars().collect::<HashMap<String, String>>(),
    )?;
    println!("Log dir: {}", log_dir);

    // Open log file
    Logger::with_str(&settings.logging.verbosity)
        .log_to_file()
        .directory(&log_dir)
        .duplicate_to_stderr(Duplicate::All)
        .start()?;

    // Start the log
    info!("Application launched with settings: {:?}", settings);
    info!("Using stack: {:?}", stack);

	// Test the logger
	debug!("This is a DEBUG message");
	info!("This is an INFO message");
	warn!("This is a WARNING message");
	error!("This is an ERROR message");

    Ok(())
}
