use passwords::PasswordGenerator;

pub fn generate_simple_password(size: usize) -> String {
    let pg = PasswordGenerator {
        length: size,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: true,
        exclude_similar_characters: false,
        strict: true,
    };

    pg.generate_one().unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::utils::user::generate_simple_password;

    #[rstest]
    #[case(10_usize)]
    #[case(50_usize)]
    #[case(100_usize)]
    fn generate_simple_password_size(#[case] input: usize) {
        let pwd = generate_simple_password(input);
        assert_eq!(pwd.chars().count(), input);
    }
}
