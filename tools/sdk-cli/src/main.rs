#[macro_use]
extern crate log;
mod command_handlers;
mod commands;
mod config;
mod persist;

use anyhow::{anyhow, Result};
use breez_sdk_core::BreezServices;
use clap::Parser;
use command_handlers::handle_command;
use commands::{Commands, SdkCli};
use persist::CliPersistence;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::path::Path;

#[tokio::main]
async fn main() {
    let cli = SdkCli::parse();
    let data_dir = cli.data_dir.clone().unwrap_or(".".to_string());
    let data_dir_path = Path::new(&data_dir);
    if !data_dir_path.exists() {
        println!("Error: data directory doesn't exist");
        return;
    }

    BreezServices::set_log_directory("/Users/roeierez/test/sdk-logs".to_string())
        .expect("Failed to init logging");

    let persistence = CliPersistence { data_dir };
    let history_file = &persistence.history_file();

    let rl = &mut Editor::<()>::new().unwrap();
    if rl.load_history(history_file).is_err() {
        info!("No previous history.");
    }

    loop {
        let readline = rl.readline("sdk> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let mut vec: Vec<&str> = line.as_str().split_whitespace().collect();
                vec.insert(0, " ");
                let cli_res = Commands::try_parse_from(vec);
                if cli_res.is_err() {
                    println!("{}", cli_res.unwrap_err());
                    continue;
                }
                let res = handle_command(rl, &persistence, cli_res.unwrap()).await;
                show_results(res);
                continue;
            }
            Err(ReadlineError::Interrupted) => {
                info!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                info!("CTRL-D");
                break;
            }
            Err(err) => {
                error!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(history_file)
        .map_err(|e| anyhow!(e))
        .unwrap()
}

fn show_results(res: Result<String>) {
    match res {
        Ok(inner) => println!("{inner}"),
        Err(err) => eprintln!("Error: {err}"),
    }
}
