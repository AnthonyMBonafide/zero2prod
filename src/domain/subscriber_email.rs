use validator::ValidateEmail;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SubscriberEmail {
    pub fn parse(email: String) -> Result<Self, String> {
        if email.validate_email() {
            return Ok(SubscriberEmail(email));
        }

        Err(format!("{} is not a valid email address", &email))
    }
}

#[cfg(test)]
mod test {

    use fake::{faker::internet::en::SafeEmail, Fake};
    use rand::{rngs::StdRng, SeedableRng};

    use super::SubscriberEmail;

    #[test]
    fn invalid_email_addresses() {
        let test_cases = vec![("a", "single letter"), ("test@.com", "no domain")];
        for test in test_cases {
            claims::assert_err!(
                SubscriberEmail::parse(test.0.to_string()),
                "did not correctly reject invalid email {}",
                test.1
            );
        }
    }

    #[derive(Clone, Debug)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);

            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_email_addresses(email: ValidEmailFixture) {
        claims::assert_ok!(SubscriberEmail::parse(email.0));
    }
}
