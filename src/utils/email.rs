use regex::Regex;

const REGEX: &str = r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b";

pub fn email_validator(address: &String) -> Result<(), serde_valid::validation::Error> {
    let re = Regex::new(REGEX).unwrap();
    if re.is_match(address) {
        Ok(())
    } else {
        Err(serde_valid::validation::Error::Custom(
            "Invalid email".to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::utils::email::email_validator;

    #[rstest]
    #[case("test@test.com".to_string())]
    #[case("joe@doe.com".to_string())]
    #[case("a@b.pt".to_string())]
    fn positive_check_valid_email(#[case] input: String) {
        assert_eq!(email_validator(&input).is_ok(), true);
    }

    #[rstest]
    #[case("test@.com".to_string())]
    #[case("@doe.com".to_string())]
    #[case("a@b".to_string())]
    fn negative_check_valid_email(#[case] input: String) {
        assert_eq!(email_validator(&input).is_err(), true);
    }
}
