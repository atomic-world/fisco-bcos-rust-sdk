mod cli;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::cli::Cli;

#[tokio::main]
async fn main() {
    let mut cli = Cli::new();
    let mut rl = Editor::<()>::new();
    if let Some(path) = home::home_dir() {
        let _ = rl.load_history(path.join(".fisco_bcos_history").as_path());
    }
    println!("Welcome to Command line tool for FISCO BCOS (V0.2.0). Type help to get help");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let command = line.as_str();
                rl.add_history_entry(command);
                cli.run_command(command).await;
            },
            Err(ReadlineError::Interrupted) => {
                break
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }

    if let Some(path) = home::home_dir() {
        rl.save_history(path.join(".fisco_bcos_history").as_path()).unwrap();
    }
}