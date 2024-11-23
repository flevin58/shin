mod cmd_confirm;
mod cmd_password;
mod cmd_select;
mod cmd_text;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct CmdArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Select(cmd_select::Args),
    Password(cmd_password::Args),
    Confirm(cmd_confirm::Args),
    Text(cmd_text::Args),
}

pub fn parse_and_run() {
    let args = CmdArgs::parse();

    match &args.command {
        Some(Commands::Select(args)) => cmd_select::run(&args),
        Some(Commands::Password(args)) => cmd_password::run(&args),
        Some(Commands::Confirm(args)) => cmd_confirm::run(&args),
        Some(Commands::Text(args)) => cmd_text::run(&args),
        None => {
            println!("No subcommand given");
        }
    }
}
