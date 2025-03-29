//! A crate to handle numbers and their written forms
//!  
//! Provides utility to convert numbers to words
//! ```rust
//! use marble::number::serialize;
//! assert_eq!(serialize::display_number(123), "OneHundredTwentyThree");
//! ```
//!
//! And utility to parse such strings to numbers
//!
//! ```rust
//! use marble::number::deserialize;
//! assert_eq!(deserialize::parse_number("OneHundredTwentyThree"), Some(123));
//! ```
//!
//! Please note, that serialization will always return a string directly,
//! because all numbers can be converted to a word. However in contrast, desirialization
//! returns an Option, as it may fail. *```"Banana"``` isn't a number after all*.

pub mod deserialize;
pub mod serialize;

#[cfg(test)]
mod test {
    use super::{deserialize, serialize};

    #[test]
    pub fn convert_u64() {
        let mut i = 0;
        while i <= 100_000 {
            assert_eq!(
                i,
                deserialize::parse_number(&serialize::display_number(i))
                    .unwrap_or_else(|| panic!("{i} didnt convert"))
            );
            i += 1;
        }
    }

    #[test]
    pub fn convert_f64() {
        let mut i = 0.0;
        while i <= 150_000.0 {
            assert_eq!(
                i,
                deserialize::parse_fraction(&serialize::display_fraction(i))
                    .unwrap_or_else(|| panic!("{i} didnt convert"))
            );
            i += 1.5;
        }
    }

    #[test]
    pub fn all_dont_parse() {
        assert!(deserialize::parse_number("OneOne").is_none());
        assert!(deserialize::parse_number("ElevenOne").is_none());
        assert!(deserialize::parse_number("TwentyEleven").is_none());
        assert!(deserialize::parse_number("OneTwenty").is_none());
        assert!(deserialize::parse_number("TenHundred").is_none());
        assert!(deserialize::parse_number("TenThousandFiveMillion").is_none());

        assert!(deserialize::parse_fraction("OnePointOnePointOne").is_none());
        assert!(deserialize::parse_fraction("OnePointOneTen").is_none());
        assert!(deserialize::parse_fraction("OnePointOneMillion").is_none());
    }
}
