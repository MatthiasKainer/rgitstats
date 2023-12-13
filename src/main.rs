use domain::Commit;
use is_terminal::IsTerminal as _;
use std::{
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

use clap::{arg, Arg, ArgAction, Command};

mod domain;
mod git;
mod io;
mod lists;
mod parser;
mod strings;

use git2::Repository;

fn cli() -> Command {
    Command::new("rgitstats")
        .about(
            "Stats on git repos using semantic commits
        
Can be sent into machine mode by passing data via stdin, then the Directories 
are expected from stdin.

Example: ls -d /home/mkainer/projects/* | cargo run -- -s --result scope -

As soon as data is passed via stdin, the output will be machine (`grep`, `awk`...) readable, not
human readable. ",
        )
        .arg_required_else_help(true)
        .arg(
            Arg::new("coe")
            .short('s')
            .long("skip-non-git")
            .action(ArgAction::SetTrue)
            .help("Will continue if there if one of the passed directories is not a valid git directory")
        ).arg(
            arg!(-r --result <RESULT>)
                .value_parser(["types", "scope", "authors", "every"])
                .default_value("types"),
        )
        .arg(arg!(<PATH> ... "Git repo(s) to check").value_parser(clap::value_parser!(PathBuf)))
}

fn main() {
    let matches = cli().get_matches();
    let paths = matches
        .get_many::<PathBuf>("PATH")
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let display = matches
        .get_one::<String>("result")
        .map(|s| s.as_str())
        .expect("defaulted...");

    let continue_on_err = matches.get_flag("coe");
    if paths.first() == Some(&&PathBuf::from("-")) {
        machine_mode(display, continue_on_err);
    } else {
        user_mode(display, paths);
    }
}

fn machine_mode(display: &str, continue_on_err: bool) {
    if stdin().is_terminal() {
        cli().print_help().unwrap();
        std::process::exit(2);
    }

    let mut full_history: Vec<Commit> = vec![];

    for line in BufReader::new(stdin().lock()).lines() {
        let history = match Repository::open(PathBuf::from(line.unwrap())) {
            Ok(it) => git::indy_jones_that_repo(it),
            Err(err) => {
                if !continue_on_err {
                    eprintln!("Failed to open repo. Is it a valid git repository?");
                    eprintln!("{}", err.message());
                    std::process::exit(1);
                }
                Ok(vec![])
            }
        };

        if history.is_err() {
            eprintln!("Failed to retrieve history.");
            std::process::exit(1);
        }
        full_history.extend(history.unwrap());
    }

    match parser::parse_entries(full_history) {
        Ok(it) => {
            io::machine_print(display, it);
        }
        Err(err) => {
            eprintln!("Failed to read history. Is it following conventional commits?");
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
}

fn user_mode(display: &str, paths: Vec<&PathBuf>) {
    for path in paths {
        let history = match Repository::open(path) {
            Ok(it) => git::indy_jones_that_repo(it),
            Err(err) => {
                eprintln!("Failed to open repo. Is it a valid git repository?");
                eprintln!("{}", err.message());
                std::process::exit(1);
            }
        };

        if history.is_err() {
            eprintln!("Failed to retrieve history.");
            std::process::exit(1);
        }

        match parser::parse_entries(history.unwrap()) {
            Ok(it) => {
                io::pretty_print(display, path, it);
            }
            Err(err) => {
                eprintln!("Failed to read history. Is it following conventional commits?");
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };
    }
}

#[cfg(test)]
mod tests {

    use assert_cmd::prelude::*;
    use predicates::str::contains;
    use std::process::Command;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let mut build_cmd = Command::new("cargo");
        build_cmd.arg("build");
        let output = build_cmd.output().expect("Failed to execute command");

        assert!(output.status.success());

        let mut cmd = Command::cargo_bin("rgitstats")?;

        cmd.arg("test/file/doesnt/exist");
        cmd.assert().failure().stderr(contains(
            "Failed to open repo. Is it a valid git repository",
        ));

        cmd = Command::cargo_bin("rgitstats")?;
        cmd.arg(".");
        cmd.assert().success().stdout(contains("feat"));

        Ok(())
    }
}
