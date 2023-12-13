pub(crate) fn some_string(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_string() {
        assert_eq!(some_string("".to_string()), None);
        assert_eq!(some_string("value".to_string()), Some("value".to_string()))
    }
}
