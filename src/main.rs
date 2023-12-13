use std::{collections::HashMap, path::PathBuf};

use clap::{arg, Command};
use git2::Repository;
use prettytable::{format, row, Table};

fn indy_jones_that_repo(repo: Repository) -> Result<Vec<String>, git2::Error> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut string_list: Vec<String> = Vec::new();
    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        string_list.push(commit.summary().unwrap_or("").into());
    }
    Ok(string_list)
}

fn parse_entries(entries: Vec<String>) -> Result<HashMap<String, i32>, String> {
    let mut types = HashMap::new();
    //let mut scopes = HashMap::new();

    for string in entries {
        let type_str = string.split(':').next().unwrap_or("").trim();
        let type_end_index = type_str
            .find("(")
            .unwrap_or(string.find(":").unwrap_or(usize::MAX));

        if type_end_index == usize::MAX {
            continue;
        }

        let type_str = &string[0..type_end_index];

        // there might be a number of merge-spam messages in there
        if type_str.to_lowercase().starts_with("merge") {
            *types.entry("merge".to_string()).or_insert(0) += 1
        } else if type_str.to_lowercase().starts_with("revert") {
            *types.entry("revert".to_string()).or_insert(0) += 1
        } else {
            *types
                .entry(type_str.to_lowercase().to_string())
                .or_insert(0) += 1;
        }
    }

    if types.is_empty() {
        Err("No types found".to_string())
    } else {
        Ok(types)
    }
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

fn cli() -> Command {
    Command::new("rgitstats")
        .about("Stats on git repos using semantic commits")
        .arg_required_else_help(true)
        .arg(arg!(<PATH> ... "Git repo(s) to check").value_parser(clap::value_parser!(PathBuf)))
}

fn main() {
    let matches = cli().get_matches();
    let paths = matches
        .get_many::<PathBuf>("PATH")
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
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
                println!("{}", path.display());
                to_table(sort_by_values(it))
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
        ];
        assert_eq!(
            parse_entries(strings.clone()).unwrap(),
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
    }
}
