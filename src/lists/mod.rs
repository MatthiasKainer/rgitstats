use std::collections::HashMap;

pub(crate) fn sort_by_values(values: HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut sorted_values: Vec<(String, u32)> = values.into_iter().collect::<Vec<(String, u32)>>();
    sorted_values.sort_by_key(|&(_, count)| -(count as i32));

    sorted_values
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
}
