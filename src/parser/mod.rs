use std::collections::HashMap;

use crate::{
    domain::{Analysis, Commit, ParsedScope, Scope},
    strings::some_string,
};

pub(crate) fn parse_entries(entries: Vec<Commit>) -> Result<Analysis, String> {
    let mut types = HashMap::new();
    let mut authors = HashMap::new();
    let mut scopes = Scope::new();

    for entry in entries {
        let message = entry.message;
        let author = entry.author;
        let commit_type = get_type(message.clone());
        if commit_type == "".to_string() {
            continue;
        }

        *types
            .entry(commit_type.to_lowercase().to_string())
            .or_insert(0) += 1;
        if !author.is_empty() {
            *authors.entry(author.clone()).or_insert(0) += 1
        }

        if let Some(scope) = get_scope(message) {
            add_to_scope(
                ParsedScope {
                    scope,
                    type_: commit_type.to_lowercase().to_string(),
                    author: some_string(author),
                },
                &mut scopes,
            );
        }
    }

    if types.is_empty() {
        Err("No types found".to_string())
    } else {
        Ok(Analysis {
            types,
            authors,
            scope: scopes,
        })
    }
}

fn add_to_scope(item: ParsedScope, scopes: &mut Scope) {
    let (count, types) = scopes.entry(item.scope.clone()).or_insert((
        0,
        std::collections::HashMap::<String, (u32, std::collections::HashMap<String, u32>)>::new(),
    ));
    *count += 1;

    let (count_for_type, authors) = types
        .entry(item.type_.clone())
        .or_insert((0, HashMap::<String, u32>::new()));

    *count_for_type += 1;

    if let Some(author) = &item.author {
        let authors_for_type = authors.entry(author.to_string()).or_insert(0);
        *authors_for_type += 1;
    }
}

fn get_scope(commit_message: String) -> Option<String> {
    let split_message = commit_message.split(":").collect::<Vec<&str>>();
    if split_message.len() == 1 {
        return None;
    }
    let split_definition = split_message[0]
        .split(&['(', ')'][..])
        .filter(|&val| !val.is_empty())
        .collect::<Vec<&str>>();
    if split_definition.clone().len() < 2 {
        return None;
    }

    let scope = split_definition.iter().last().unwrap_or(&"").trim();
    if scope.is_empty() {
        return None;
    }
    Some(scope.to_string())
}

fn get_type(message: String) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_scope() {
        let strings = vec![
            "build: release new version 0.1.1",
            "feat(authors): Produces author stats as result",
            "perf: add precommit (ie: commit-msg) hooks",
            "feat(cli): Use clamp for cli parsing (careful)",
            "docs: add installation description",
        ];
        let expectations = vec![
            None,
            Some("authors".to_string()),
            None,
            Some("cli".to_string()),
            None,
        ];
        for index in 0..strings.len() {
            assert_eq!(get_scope(strings[index].to_string()), expectations[index]);
        }
    }

    #[test]
    fn test_scope() {
        let vec: Vec<ParsedScope> = vec![
            ParsedScope {
                scope: "scope1".to_string(),
                type_: "type1".to_string(),
                author: Some("author1".to_string()),
            },
            ParsedScope {
                scope: "scope2".to_string(),
                type_: "type2".to_string(),
                author: Some("author2".to_string()),
            },
            ParsedScope {
                scope: "scope2".to_string(),
                type_: "type2".to_string(),
                author: Some("author1".to_string()),
            },
            ParsedScope {
                scope: "scope2".to_string(),
                type_: "type2".to_string(),
                author: Some("author2".to_string()),
            },
            ParsedScope {
                scope: "scope2".to_string(),
                type_: "type3".to_string(),
                author: Some("author3".to_string()),
            },
        ];
        let mut scope = Scope::new();
        for parsed_scope in vec {
            add_to_scope(parsed_scope, &mut scope);
        }
        assert_eq!(
            scope,
            [
                (
                    "scope1".to_string(),
                    (
                        1,
                        [(
                            "type1".to_string(),
                            (1, [("author1".to_string(), 1)].iter().cloned().collect())
                        )]
                        .iter()
                        .cloned()
                        .collect()
                    )
                ),
                (
                    "scope2".to_string(),
                    (
                        4,
                        [
                            (
                                "type2".to_string(),
                                (
                                    3,
                                    [("author2".to_string(), 2), ("author1".to_string(), 1)]
                                        .iter()
                                        .cloned()
                                        .collect()
                                )
                            ),
                            (
                                "type3".to_string(),
                                (1, [("author3".to_string(), 1)].iter().cloned().collect())
                            )
                        ]
                        .iter()
                        .cloned()
                        .collect()
                    )
                )
            ]
            .iter()
            .cloned()
            .collect()
        )
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
        let Analysis {
            types,
            authors,
            scope: _,
        } = parse_entries(strings).unwrap();
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
