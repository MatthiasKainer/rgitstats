use std::path::PathBuf;

use clap::{arg, Command};

mod domain;
mod git;
mod io;
mod lists;
mod parser;
mod strings;

use git2::Repository;

fn cli() -> Command {
    Command::new("rgitstats")
        .about("Stats on git repos using semantic commits")
        .arg_required_else_help(true)
        .arg(
            arg!(--result <RESULT>)
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
