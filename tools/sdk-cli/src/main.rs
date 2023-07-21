#[macro_use]
extern crate log;
mod command_handlers;
mod commands;
mod config;
mod persist;

use anyhow::{anyhow, Result};
use chrono::Local;
use clap::Parser;
use command_handlers::handle_command;
use commands::{Commands, SdkCli};
use persist::CliPersistence;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[tokio::main]
async fn main() {
    init_logging();

    let cli = SdkCli::parse();
    let data_dir = cli.data_dir.clone().unwrap_or(".".to_string());
    let data_dir_path = Path::new(&data_dir);
    if !data_dir_path.exists() {
        println!("Error: data directory doesn't exist");
        return;
    }

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
        Ok(inner) => {
            println!("{}", inner);
        }
        Err(err) => error!("Error: {}", err),
    }
}

fn init_logging() {
    let target = Box::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open("cli.log")
            .expect("Can't create log file"),
    );
    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target))
        .parse_filters(
            r#"
            info,
            gl_client=warn,
            h2=warn,
            hyper=warn,
            lightning_signer=warn,
            reqwest=warn,
            rustls=warn,
            rustyline=warn,
            vls_protocol_signer=warn
        "#,
        )
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}
