use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(
        required = false,
        long, short,
        help = "The prompt to display",
        default_value_t = String::from("Enter your choice:"),
    )]
    prompt: String,

    #[arg(
        required = true,
        long, short,
        help = "Options",
        value_delimiter = ' ',
        num_args = 1..,
    )]
    choices: Option<Vec<String>>,

    #[arg(required = false, long, short, help = "Allows multiple selections")]
    multi: bool,
}

pub fn run(args: &Args) {
    if args.multi {
        let select =
            inquire::MultiSelect::new(&args.prompt, args.choices.clone().unwrap()).prompt();
        if select.is_ok() {
            print!("{}", select.unwrap().join(","));
        }
    } else {
        let select = inquire::Select::new(&args.prompt, args.choices.clone().unwrap()).prompt();
        if select.is_ok() {
            print!("{}", select.unwrap());
        }
    }
}
