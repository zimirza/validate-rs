pub fn validate_phone(input: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_phone_test() {
        let result = validate_phone("");
        assert_eq!(result, false);
    }
}