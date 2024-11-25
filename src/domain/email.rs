use validator::ValidateEmail;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EmailUser {
    pub email: String,
}

impl EmailUser {
    pub fn parse(s: String) -> Result<EmailUser, String> {
        if s.validate_email() {
            Ok(Self { email: s })
        } else {
            Err(format!("{} is not a valid register email address.", s))
        }
    }
}

impl std::fmt::Display for EmailUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.email.fmt(f)
    }
}

impl AsRef<str> for EmailUser {
    fn as_ref(&self) -> &str {
        &self.email
    }
}

#[cfg(test)]
mod tests {
    use super::EmailUser;
    use claims::assert_err;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(EmailUser::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(EmailUser::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(EmailUser::parse(email));
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        EmailUser::parse(valid_email.0).is_ok()
    }
}
