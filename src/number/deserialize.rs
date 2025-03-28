/// Peeks the next part (digit, factor, etc.)
///
/// A part ends at the end of the string or before the next capitalized letter
fn peek_next_part(word: &str) -> &str {
    if word.is_empty() {
        return word;
    }

    let start = first_char_length(word);

    word[start..]
        .find(|c: char| c.is_ascii_uppercase())
        .map_or_else(|| (word), |u| &word[..start + u])
}

fn first_char_length(word: &str) -> usize {
    word.chars().next().unwrap().len_utf8()
}

/// Consumes the next part and returns a peek and the rest of the word
///
/// Using this function, the peeked value (.0) is the first part of the word (.1)
fn consume_and_peek(mut word: &str) -> (&str, &str) {
    (_, word) = split_next_part(word);
    (peek_next_part(word), word)
}

/// Splits the next part (digit, factor, etc.) off the word
///
/// Returns the next part, which starts at the next uppercase letter and the remaining word
fn split_next_part(word: &str) -> (&str, &str) {
    if word.is_empty() {
        return (word, word);
    }

    word[first_char_length(word)..]
        .find(|c: char| c.is_ascii_uppercase())
        .map_or_else(|| (word, &word[0..0]), |u| word.split_at(u + 1))
}

/// Converts a digit word to its number
///
/// If the provided word is a digit, then this function returns its numeral value
pub fn parse_digit(digit: &str) -> Option<u64> {
    match digit {
        "Zero" => Some(0),
        "One" => Some(1),
        "Two" => Some(2),
        "Three" => Some(3),
        "Four" => Some(4),
        "Five" => Some(5),
        "Six" => Some(6),
        "Seven" => Some(7),
        "Eight" => Some(8),
        "Nine" => Some(9),
        _ => None,
    }
}

/// Converts a factor to its number
///
/// If the provided word is a factor, then this function returns its numeral value
pub fn parse_factor(word: &str) -> Option<u64> {
    match word {
        "" => Some(1),
        "Thousand" => Some(1_000),
        "Million" => Some(1_000_000),
        "Billion" => Some(1_000_000_000),
        "Trillion" => Some(1_000_000_000_000),
        "Quadrillion" => Some(1_000_000_000_000_000),
        "Quintillion" => Some(1_000_000_000_000_000_000),
        _ => None,
    }
}

/// Splits off a triplet of digits from the word and tries to parse it
///
/// Then returns the parsed number and the remaining string if successfull
pub fn parse_triplet(mut word: &str) -> Option<(u64, &str)> {
    let mut number = 0;

    let mut next_part = peek_next_part(word);

    if let Some(digit) = parse_digit(next_part) {
        // Consume the digit
        (next_part, word) = consume_and_peek(word);

        if next_part != "Hundred" {
            // Its a number between 0 and 9
            return Some((digit, word));
        }

        // Consume "Hundred"
        (next_part, word) = consume_and_peek(word);

        number += digit * 100;
    }

    if let Some(literal) = match next_part {
        "Ten" => Some(10),
        "Eleven" => Some(11),
        "Twelve" => Some(12),
        "Thirteen" => Some(13),
        "Fourteen" => Some(14),
        "Fifteen" => Some(15),
        "Sixteen" => Some(16),
        "Seventeen" => Some(17),
        "Eighteen" => Some(18),
        "Nineteen" => Some(19),
        _ => None,
    } {
        (_, word) = split_next_part(word);

        // After matching a literal, nothing can follow this
        return Some((number + literal, word));
    }

    if let Some(dec) = match next_part {
        "Twenty" => Some(20),
        "Thirty" => Some(30),
        "Forty" => Some(40),
        "Fifty" => Some(50),
        "Sixty" => Some(60),
        "Seventy" => Some(70),
        "Eighty" => Some(80),
        "Ninety" => Some(90),
        _ => None,
    } {
        (next_part, word) = consume_and_peek(word);
        number += dec;
    }

    if let Some(digit) = parse_digit(next_part) {
        (_, word) = split_next_part(word);
        number += digit;
    }

    // If nothing was matched, we return None
    if number == 0 {
        return None;
    }

    Some((number, word))
}

/// Tries to parse an integer from its written form
///
/// ```rust
/// use marble::number::deserialize;
/// assert_eq!(deserialize::parse_number("OneHundredTwentyThree"), Some(123));
/// ```
///
/// This is an operation that can fail, as not every word is a valid number. Thus it returns an Option
pub fn parse_number(mut word: &str) -> Option<u64> {
    let mut smallest_factor = u64::MAX;
    let mut number = 0;

    loop {
        // First parse a triplet (OneHundredNineteen) and then its factor (Thousand)
        let parsed_number;
        (parsed_number, word) = parse_triplet(word)?;
        let factor_word;
        (factor_word, word) = split_next_part(word);
        let factor = parse_factor(factor_word)?;

        // You can't have million after thousand
        if factor > smallest_factor {
            return None;
        }

        smallest_factor = factor;

        number += parsed_number * factor;

        // If the word is empty or the last factor was parsed, we stop
        if factor == 1 || word.is_empty() {
            break;
        }
    }

    // If there is still something, then the parse wasn't successfull
    if !word.is_empty() {
        return None;
    }

    Some(number)
}

/// Tries to parse a double from its written form
///
/// The decimal seperator is "Point"
///
/// ```rust
/// use marble::number::deserialize;
/// assert_eq!(deserialize::parse_fraction("ThreePointOneFour"), Some(3.14));
/// ```
///
/// This is an operation that can fail, as not every word is a valid number. Thus it returns an Option
pub fn parse_fraction(word: &str) -> Option<f64> {
    word.split_once("Point").map_or_else(
        // If there was no decimal seperator, we can just parse a number and convert to a double
        || parse_number(word).map(|u| u as f64),
        // If there is a decimal seperator, we first parse the whole number and then the fraction
        |(whole_word, mut fraction_word)| {
            parse_number(whole_word).and_then(|whole| {
                let mut factor = 0.1;
                let mut fraction = 0.0;
                while !fraction_word.is_empty() {
                    let digit;
                    // As long there are parts, we assume they're digits
                    (digit, fraction_word) = split_next_part(fraction_word);
                    fraction += parse_digit(digit)? as f64 * factor;
                    factor *= 0.1;
                }

                Some(whole as f64 + fraction)
            })
        },
    )
}
