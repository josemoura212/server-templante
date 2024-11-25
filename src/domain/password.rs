use secrecy::SecretString;
use validator::ValidateLength;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PasswordUser {
    pub password: SecretString,
}

impl PasswordUser {
    pub fn parse(s: String) -> Result<PasswordUser, String> {
        if s.validate_length(Some(6), Some(50), None) {
            Ok(Self {
                password: SecretString::from(s),
            })
        } else {
            Err("Invalid password, password must contain between 6 and 50 characters".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PasswordUser;
    use claims::assert_err;
    use fake::faker::internet::en::Password;
    use fake::Fake;

    #[test]
    fn empty_string_is_rejected() {
        let password = "".to_string();
        assert_err!(PasswordUser::parse(password));
    }

    #[test]
    fn password_missing_at_min_length_rejected() {
        let password = "12345".to_string();
        assert_err!(PasswordUser::parse(password));
    }

    #[test]
    fn password_with_more_than_50_characters_is_rejected() {
        let password = "123456789012345678901234567890123456789012345678901".to_string();
        assert_err!(PasswordUser::parse(password));
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub String);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = Password(7..50).fake_with_rng(g);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        PasswordUser::parse(valid_password.0).is_ok()
    }
}
