use unicode_segmentation::UnicodeSegmentation;

const INVALID_CHARACTERS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

pub struct NewSubscriber {
    pub name: SubscriberName,
    pub email: String,
}
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct SubscriberName(String);

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl SubscriberName {
    pub fn parse(s: String) -> Result<Self, String> {
        if s.trim().is_empty() {
            return Err(format!("{} is empty or only contains whitespace", s));
        }

        if s.graphemes(true).count() > 256 {
            return Err(format!("{} is more than 256 characters long", s));
        }

        if s.chars().any(|c| INVALID_CHARACTERS.contains(&c)) {
            return Err(format!("{} contains an invalid character", s));
        }

        Ok(Self(s))
    }
}

#[cfg(test)]
mod test {
    use super::{SubscriberName, INVALID_CHARACTERS};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let mut name = String::with_capacity(256);
        for _ in 0..256 {
            name.push('ά');
        }

        claims::assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_grapheme_is_rejected() {
        let mut name = String::with_capacity(257);
        for _ in 0..257 {
            name.push('ά');
        }

        claims::assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn white_space_only_names_are_rejected() {
        claims::assert_err!(SubscriberName::parse("  ".to_string()));
    }

    #[test]
    fn empty_name_is_rejected() {
        claims::assert_err!(SubscriberName::parse("".to_string()));
    }

    #[test]
    fn names_containing_invliad_characters_are_rejected() {
        for name in INVALID_CHARACTERS {
            claims::assert_err!(SubscriberName::parse(name.to_string()));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        claims::assert_ok!(SubscriberName::parse("Anthony Bonafide".to_string()));
    }
}
