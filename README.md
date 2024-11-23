# SHIN - SHell INput
This program allows to have some cool user input for shell scripts.
It is based on the inquire crate and implements some of its features.
It is *not* meant to be run in the console: it just outputs user selection to stdout.

## Usage
response = $(shin command flags)

commands have same name and meaning of the inquire crate:

- confirm: prompts the user for a confirmation and returns either "yes" or "no"
- password: prompts the user for a password and asks for confirmation. Returns the password
- select: prompts the user to choose among a series of choices and resturns the selection.
- text: prompts the user to input a string and returns it.

## confirm

Usage: shin confirm [OPTIONS] --prompt <PROMPT>

Options:
  -p, --prompt <PROMPT>  The prompt to display
  -y, --yes              The default value to return
  -h, --help             Print help

## password

Usage: shin password [OPTIONS]

Options:
  -p, --prompt <PROMPT>  The prompt to display [default: "Enter password:"]
  -m, --min <MIN>        The minimum nuber of characters [default: 8]
  -s, --symbols          Must have at least one symbol
  -d, --digits           Must have at least one digit
  -h, --help             Print help

## select

Usage: shin select [OPTIONS] --choices <CHOICES>...

Options:
  -p, --prompt <PROMPT>       The prompt to display [default: "Enter your choice:"]
  -c, --choices <CHOICES>...  Options
  -m, --multi                 Allows multiple selections
  -h, --help                  Print help


## text

Usage: shin text --prompt <PROMPT>

Options:
  -p, --prompt <PROMPT>  The prompt to display
  -h, --help             Print help
