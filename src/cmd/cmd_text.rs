use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(required = true, long, short, help = "The prompt to display")]
    prompt: String,
}

pub fn run(args: &Args) {
    let answer = inquire::Text::new(&args.prompt).prompt();
    match answer {
        Ok(text) => print!("{}", text),
        Err(_) => {}
    }
}
