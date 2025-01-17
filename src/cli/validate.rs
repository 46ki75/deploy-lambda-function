// ---------- common functions for validation ----------

fn common_validate_length(s: &str, min: usize, max: usize) -> Result<(), String> {
    if s.len() >= min && s.len() <= max {
        Ok(())
    } else {
        Err(format!(
            "Length must be between {} and {} characters.",
            min, max
        ))
    }
}

fn common_validate_regex(s: &str, pattern: &str) -> Result<(), String> {
    if regex::Regex::new(pattern).unwrap().is_match(s) {
        Ok(())
    } else {
        Err(format!("Does not match the pattern: {}", pattern))
    }
}

// ---------- validation functions ----------

pub fn validate_description(s: &str) -> Result<(), String> {
    common_validate_length(s, 0, 256)?;
    Ok(())
}

// ---------- tests ----------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_validate_length() {
        assert_eq!(common_validate_length("", 0, 0), Ok(()));
        assert_eq!(common_validate_length("", 0, 1), Ok(()));
        assert_eq!(
            common_validate_length("", 1, 1),
            Err("Length must be between 1 and 1 characters.".to_string())
        );
        assert_eq!(
            common_validate_length("a", 0, 0),
            Err("Length must be between 0 and 0 characters.".to_string())
        );
        assert_eq!(common_validate_length("a", 0, 1), Ok(()));
        assert_eq!(common_validate_length("a", 1, 1), Ok(()));
        assert_eq!(
            common_validate_length("ab", 0, 1),
            Err("Length must be between 0 and 1 characters.".to_string())
        );
        assert_eq!(
            common_validate_length("ab", 1, 1),
            Err("Length must be between 1 and 1 characters.".to_string())
        );
        assert_eq!(common_validate_length("ab", 2, 2), Ok(()));
    }

    #[test]
    fn test_common_validate_regex() {
        assert_eq!(common_validate_regex("", ""), Ok(()));
        assert_eq!(
            common_validate_regex("", "a"),
            Err("Does not match the pattern: a".to_string())
        );
        assert_eq!(common_validate_regex("a", ""), Ok(()));
        assert_eq!(common_validate_regex("a", "a"), Ok(()));
        assert_eq!(
            common_validate_regex("a", "b"),
            Err("Does not match the pattern: b".to_string())
        );
        assert_eq!(common_validate_regex("ab", "ab"), Ok(()));
        assert_eq!(
            common_validate_regex("ab", "ba"),
            Err("Does not match the pattern: ba".to_string())
        );
    }
}
