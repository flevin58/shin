// To see different ways to define args with clap:
// For derive  visit: https://github.com/clap-rs/clap/tree/master/examples/tutorial_derive
// For builder visit: https://github.com/clap-rs/clap/tree/master/examples/tutorial_builder
mod cmd;

fn main() {
    cmd::parse_and_run();
}
