use prettytable::{format, row, Table};
use std::{collections::HashMap, path::PathBuf};

use crate::{
    domain::{Analysis, Scope},
    lists::sort_by_values,
};

fn to_table(values: Vec<(String, u32)>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_DEFAULT);
    table.set_titles(row!["Type", "Count", "Percentage"]);
    let total = values
        .clone()
        .into_iter()
        .map(|(_, count)| count)
        .sum::<u32>();
    for value in values {
        table.add_row(row![
            value.0,
            value.1,
            format!("{:.2}%", (value.1 as f64 / total as f64) * 100.0)
        ]);
    }
    table.printstd();
}

fn to_scope_type_author_table(authors: HashMap<String, u32>) -> std::string::String {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_DEFAULT);
    table.set_titles(row!["Author", "Count", "Percentage"]);

    for entry in authors {
        table.add_row(row![entry.0, entry.1]);
    }
    table.to_string()
}

fn to_scope_type_table(
    types: HashMap<String, (u32, std::collections::HashMap<String, u32>)>,
) -> std::string::String {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_DEFAULT);
    table.set_titles(row!["Type", "Count", "Percentage", "Author"]);
    let total = types.clone().into_iter().map(|(_, s)| s.0).sum::<u32>();

    for entry in types {
        table.add_row(row![
            entry.0,
            entry.1 .0,
            format!("{:.2}%", (entry.1 .0 as f64 / total as f64) * 100.0),
            to_scope_type_author_table(entry.1 .1)
        ]);
    }
    table.to_string()
}

fn to_scope_table(values: Scope) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_DEFAULT);
    table.set_titles(row!["Scope", "Count", "Percentage", "Type"]);
    let total = values.clone().into_iter().map(|(_, s)| s.0).sum::<u32>();
    for value in values {
        table.add_row(row![
            value.0,
            value.1 .0,
            format!("{:.2}%", (value.1 .0 as f64 / total as f64) * 100.0),
            to_scope_type_table(value.1 .1)
        ]);
    }
    table.printstd()
}

pub(crate) fn pretty_print(display: &str, path: &PathBuf, result: Analysis) {
    if display == "every" {
        println!("{}", path.display());
    }
    if display == "every" || display == "types" {
        to_table(sort_by_values(result.types));
    }
    if display == "every" || display == "authors" {
        to_table(sort_by_values(result.authors));
    }
    if display == "every" || display == "scope" {
        to_scope_table(result.scope);
    }
}
