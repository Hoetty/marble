pub mod serialize;
pub mod deserialize;

#[cfg(test)]
mod test {
    use super::{deserialize, serialize};

    #[test]
    pub fn convert_u64() {
        let mut i = 0;
        while i <= 100_000 {
            assert_eq!(i, deserialize::parse_number(&serialize::display_number(i)).expect(&format!("{i} didnt convert")));
            i += 1;
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
    }

}