use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(required = true, long, short, help = "The prompt to display")]
    prompt: String,

    #[arg(
        required = false,
        long,
        short,
        help = "The default value to return",
        default_value_t = false
    )]
    yes: bool,
}

pub fn run(args: &Args) {
    let answer = inquire::Confirm::new(&args.prompt)
        .with_default(args.yes)
        .prompt();
    match answer {
        Ok(true) => print!("yes"),
        Ok(false) => print!("no"),
        Err(_) => {}
    }
}
