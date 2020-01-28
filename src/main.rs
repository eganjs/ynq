#[macro_use]
extern crate clap;
extern crate rprompt;

use clap::{App, Arg};
use rprompt::prompt_reply_stdout;
use std::process::exit;

fn main() {
    let matches = App::new("ynq")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Very simple utility to ask a yes/no question")
        .arg(
            Arg::with_name("default choice")
                .help("The default choice for the answer")
                .index(1)
                .required(true)
                .possible_values(&["Y", "N"]),
        )
        .arg(
            Arg::with_name("question")
                .help("The yes/no question to ask")
                .index(2)
                .required(true)
                .multiple(true),
        )
        .get_matches();

    let default_choice = matches.value_of("default choice").unwrap();

    let question = matches
        .values_of("question")
        .unwrap()
        .collect::<Vec<_>>()
        .join(" ");

    let question = format!(
        "{} {}: ",
        question,
        match default_choice {
            "Y" => "[Y/n]",
            "N" => "[y/N]",
            _ => panic!("Unreachable code"),
        },
    );

    loop {
        let response = &*prompt_reply_stdout(&question).unwrap();

        let response = match response {
            "" => default_choice,
            _ => response,
        };

        match response {
            "Y" | "y" => exit(0),
            "N" | "n" => exit(1),
            _ => (),
        };
    }
}
