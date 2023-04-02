#[macro_use]
extern crate log;
mod command_handlers;
mod commands;
mod config;

use anyhow::{anyhow, Result};
use clap::Parser;
use command_handlers::handle_command;
use commands::{Commands, SdkCli};
use env_logger::Env;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::path::Path;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(
        Env::default()
            .default_filter_or("debug,rustyline=warn,hyper=warn,reqwest=warn,rustls=warn,h2=warn"),
    )
    .init();

    let cli = SdkCli::parse();
    if cli.data_dir.is_some() && !Path::new(cli.data_dir.as_ref().unwrap().as_str()).exists() {
        println!("Error: data directory doesn't exist");
        return;
    }
    let rl = &mut Editor::<()>::new().unwrap();
    if rl.load_history("history.txt").is_err() {
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
                let data_dir = cli.data_dir.clone().unwrap_or(".".to_string());
                let res = handle_command(rl, &data_dir, cli_res.unwrap()).await;
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
    rl.save_history("history.txt")
        .map_err(|e| anyhow!(e))
        .unwrap()
}

fn show_results(res: Result<String>) {
    match res {
        Ok(inner) => {
            println!("{}", inner);
        }
        Err(err) => error!("Error: {}", err),
    }
}
