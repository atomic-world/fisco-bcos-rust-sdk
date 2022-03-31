mod cli;

use rustyline::{error::ReadlineError, Editor};

use crate::cli::Cli;

fn print_help() {
    println!("=============================================================================================");
    println!("Welcome to FISCO BCOS console(0.4.0).");
    println!("Type 'help' for help.");
    println!("Type 'CTRL-C' or 'CTRL-D' to quit console.");
    println!("Visit https://github.com/kkawakam/rustyline#actions to get more actions.\n");
    println!(
        r#"________ ______  ______   ______   ______       _______   ______   ______   ______
|        |      \/      \ /      \ /      \     |       \ /      \ /      \ /      \
| $$$$$$$$\$$$$$|  $$$$$$|  $$$$$$|  $$$$$$\    | $$$$$$$|  $$$$$$|  $$$$$$|  $$$$$$\
| $$__     | $$ | $$___\$| $$   \$| $$  | $$    | $$__/ $| $$   \$| $$  | $| $$___\$$
| $$  \    | $$  \$$    \| $$     | $$  | $$    | $$    $| $$     | $$  | $$\$$    \
| $$$$$    | $$  _\$$$$$$| $$   __| $$  | $$    | $$$$$$$| $$   __| $$  | $$_\$$$$$$\
| $$      _| $$_|  \__| $| $$__/  | $$__/ $$    | $$__/ $| $$__/  | $$__/ $|  \__| $$
| $$     |   $$ \\$$    $$\$$    $$\$$    $$    | $$    $$\$$    $$\$$    $$\$$    $$
 \$$      \$$$$$$ \$$$$$$  \$$$$$$  \$$$$$$      \$$$$$$$  \$$$$$$  \$$$$$$  \$$$$$$"#
    );
    println!("\n=============================================================================================\n");
}

#[tokio::main]
async fn main() {
    let mut cli = Cli::new();
    let mut rl = Editor::<()>::new();
    if let Some(path) = home::home_dir() {
        let _ = rl.load_history(path.join(".fisco_bcos_history").as_path());
    }
    print_help();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let command = line.as_str();
                rl.add_history_entry(command);
                cli.run_command(command).await;
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }

    if let Some(path) = home::home_dir() {
        rl.save_history(path.join(".fisco_bcos_history").as_path())
            .unwrap();
    }
}
