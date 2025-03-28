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
