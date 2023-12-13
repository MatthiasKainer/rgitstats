use std::{collections::HashMap, path::PathBuf};

use clap::{arg, Command};
use git2::Repository;
use prettytable::{format, row, Table};

struct Commit {
    message: String,
    author: String,
}

#[derive(Debug)]
struct Analysis {
    types: HashMap<String, i32>,
    authors: HashMap<String, i32>,
}

fn indy_jones_that_repo(repo: Repository) -> Result<Vec<Commit>, git2::Error> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut commit_list: Vec<Commit> = Vec::new();
    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        commit_list.push(Commit {
            message: commit.summary().unwrap_or("").into(),
            author: commit.author().name().unwrap_or("").into(),
        });
    }
    Ok(commit_list)
}

fn parse_entries(entries: Vec<Commit>) -> Result<Analysis, String> {
    let mut types = HashMap::new();
    let mut authors = HashMap::new();
    //let mut scopes = HashMap::new();

    for entry in entries {
        let message = entry.message;
        let commit_type = get_types(message);
        if commit_type == "".to_string() {
            continue;
        }

        *types
            .entry(commit_type.to_lowercase().to_string())
            .or_insert(0) += 1;
        if entry.author != "" {
            *authors.entry(entry.author).or_insert(0) += 1
        }
    }

    if types.is_empty() {
        Err("No types found".to_string())
    } else {
        Ok(Analysis { types, authors })
    }
}

fn get_types(message: String) -> String {
    let type_str = message.split(':').next().unwrap_or("").trim();
    let type_end_index = type_str
        .find("(")
        .unwrap_or(message.find(":").unwrap_or(usize::MAX));

    if type_end_index == usize::MAX {
        return "".to_string();
    }

    let type_str = &message[0..type_end_index];
    return if type_str.to_lowercase().starts_with("merge") {
        "merge".to_string()
    } else if type_str.to_lowercase().starts_with("revert") {
        "revert".to_string()
    } else {
        type_str.to_lowercase().to_string()
    };
}

fn sort_by_values(values: HashMap<String, i32>) -> Vec<(String, i32)> {
    let mut sorted_values: Vec<(String, i32)> = values.into_iter().collect::<Vec<(String, i32)>>();
    sorted_values.sort_by_key(|&(_, count)| -(count as i32));

    sorted_values
}

fn to_table(values: Vec<(String, i32)>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_DEFAULT);
    table.set_titles(row!["Type", "Count", "Percentage"]);
    let total = values
        .clone()
        .into_iter()
        .map(|(_, count)| count)
        .sum::<i32>();
    for value in values {
        table.add_row(row![
            value.0,
            value.1,
            format!("{:.2}%", (value.1 as f64 / total as f64) * 100.0)
        ]);
    }
    table.printstd();
}

fn pretty_print(display: &str, path: &PathBuf, result: Analysis) {
    if display == "every" {
        println!("{}", path.display());
    }
    if display == "every" || display == "types" {
        to_table(sort_by_values(result.types));
    }
    if display == "every" || display == "authors" {
        to_table(sort_by_values(result.authors));
    }
}

fn cli() -> Command {
    Command::new("rgitstats")
        .about("Stats on git repos using semantic commits")
        .arg_required_else_help(true)
        .arg(
            arg!(--result <RESULT>)
                .value_parser(["types", "scope", "authors", "every"])
                .default_value("every"),
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
            Ok(it) => indy_jones_that_repo(it),
            Err(err) => {
                println!("Failed to open repo. Is it a valid git repository?");
                println!("{}", err);
                std::process::exit(1);
            }
        };

        if history.is_err() {
            println!("Failed to retrieve history.");
            std::process::exit(1);
        }

        match parse_entries(history.unwrap()) {
            Ok(it) => {
                pretty_print(display, path, it);
            }
            Err(err) => {
                println!("Failed to read history. Is it following conventional commits?");
                println!("{}", err);
                std::process::exit(1);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorting() {
        let input = [
            ("fix".to_string(), 2),
            ("perf".to_string(), 1),
            ("feat".to_string(), 3),
            ("docs".to_string(), 1),
        ]
        .iter()
        .cloned()
        .collect();

        let expected = vec![
            ("feat".to_string(), 3),
            ("fix".to_string(), 2),
            ("docs".to_string(), 1),
            ("perf".to_string(), 1),
        ];
        assert_eq!(sort_by_values(input)[0..2], expected[0..2]);
    }

    #[test]
    fn test_parse_entries() {
        assert_eq!(
            parse_entries(vec![]).unwrap_err(),
            "No types found".to_string()
        );

        let strings = vec![
            "feat(core): added stuff (this is the stuff)".to_string(),
            "feat(ui): new buttons".to_string(),
            "fix(ui): fix buttons".to_string(),
            "perf: removed react (because why not)".to_string(),
            "skip that one because not following conventional commits".to_string(),
            "Merged PR 42: blablabla".to_string(),
            "Merge branch some/branch: blablabla".to_string(),
            "fix(perf): fixed broken app".to_string(),
            "docs: improved readme".to_string(),
            "feat(core): added more stuff".to_string(),
        ]
        .into_iter()
        .map(|line| Commit {
            message: line,
            author: "Matthias Kainer".to_string(),
        })
        .collect();
        let Analysis { types, authors } = parse_entries(strings).unwrap();
        assert_eq!(
            types,
            ([
                ("feat".to_string(), 3),
                ("fix".to_string(), 2),
                ("perf".to_string(), 1),
                ("docs".to_string(), 1),
                ("merge".to_string(), 2),
            ]
            .iter()
            .cloned()
            .collect())
        );
        assert_eq!(
            authors,
            ([("Matthias Kainer".to_string(), 9),]
                .iter()
                .cloned()
                .collect())
        );
    }
}
