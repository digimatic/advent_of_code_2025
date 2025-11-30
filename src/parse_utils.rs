pub fn parse_numbers<T>(input: &str) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let re = regex::Regex::new(r"\d+").expect("Failed to compile regex");
    re.find_iter(input)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

pub fn parse_signed_numbers<T>(input: &str) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let re = regex::Regex::new(r"-?\d+").expect("Failed to compile regex");
    re.find_iter(input)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        assert_eq!(parse_numbers::<i32>("123 456 789"), vec![123, 456, 789]);
        assert_eq!(parse_numbers::<i32>("abc123def456ghi"), vec![123, 456]);
        assert_eq!(parse_numbers::<i32>("no numbers here"), vec![]);
        assert_eq!(parse_numbers::<i32>("42"), vec![42]);
        assert_eq!(parse_numbers::<i32>("1a2b3c"), vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_signed_numbers() {
        assert_eq!(
            parse_signed_numbers::<i32>("123 -456 789"),
            vec![123, -456, 789]
        );
        assert_eq!(
            parse_signed_numbers::<i32>("abc-123def456ghi"),
            vec![-123, 456]
        );
        assert_eq!(parse_signed_numbers::<i32>("no numbers here"), vec![]);
        assert_eq!(parse_signed_numbers::<i32>("-42"), vec![-42]);
        assert_eq!(parse_signed_numbers::<i32>("1a-2b3c"), vec![1, -2, 3]);
        // assert_eq!(parse_signed_numbers("--5"), vec![5]);
    }
}
