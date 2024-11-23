use std::error::Error;
use std::rc::Rc;

use clap::Parser;
use inquire::validator::Validation;
use inquire::PasswordDisplayMode;

#[derive(Parser)]
pub struct Args {
    #[arg(
        required = false,
        long, short,
        help = "The prompt to display",
        default_value_t = String::from("Enter password:"),
    )]
    prompt: String,
    #[arg(
        required = false,
        long,
        short,
        help = "The minimum nuber of characters",
        default_value_t = 8
    )]
    min: usize,
    #[arg(
        required = false,
        long,
        short,
        help = "Must have at least one symbol",
        default_value_t = false
    )]
    symbols: bool,
    #[arg(
        required = false,
        long,
        short,
        help = "Must have at least one digit",
        default_value_t = false
    )]
    digits: bool,
}

pub fn run(args: &Args) {
    let min = Rc::new(args.min);
    let symbols = Rc::new(args.symbols);
    let digits = Rc::new(args.digits);
    let pw_validator = {
        let min = min.clone();
        move |p: &str| -> Result<Validation, Box<dyn Error + Send + Sync>> {
            // Check that length is at least 8 charaters
            if p.len() < *min {
                return Ok(Validation::Invalid(
                    format!(
                        "Password too short. Need to be at least {} characters.",
                        *min
                    )
                    .into(),
                ));
            }
            // Check that it contains at least one digit 0..9
            if *digits && !p.as_bytes().iter().any(|ch| ch.is_ascii_digit()) {
                return Ok(Validation::Invalid(
                    "Password should contain at least one digit.".into(),
                ));
            }
            // Check that it contains at least one symbol character
            if *symbols && !p.as_bytes().iter().any(|ch| ch.is_ascii_punctuation()) {
                return Ok(Validation::Invalid(
                    "Password should contain at least one special character.".into(),
                ));
            }
            return Ok(Validation::Valid);
        }
    };

    let prompt_message = args.prompt.clone();
    let passw = inquire::Password::new(&prompt_message)
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_validator(pw_validator)
        .prompt();
    if passw.is_ok() {
        print!("{}", passw.unwrap());
    }
}
