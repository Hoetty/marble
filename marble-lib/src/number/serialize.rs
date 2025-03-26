const LARGEST: u64 = 1_000_000_000_000_000_000;

/// Generates the name for a factor
///
/// A factor is anything like Thousand, Million, etc.
fn factor_name(number: u64) -> &'static str {
    match number {
        1 => "",
        1_000 => "Thousand",
        1_000_000 => "Million",
        1_000_000_000 => "Billion",
        1_000_000_000_000 => "Trillion",
        1_000_000_000_000_000 => "Quadrillion",
        1_000_000_000_000_000_000 => "Quintillion",
        _ => panic!("Factor does not conform to 1000 ** n"),
    }
}

/// Adds a triplet of digits to the string
///
/// A triplet is anywhere in the range of 0..1000
fn append_triplet_to_name(word: &mut String, mut triplet: u64) {
    if triplet >= 100 {
        // If the triplet is larger than 100,
        // the first digit needs to be added, followed by ```"Hundred"```
        append_triplet_to_name(word, triplet / 100);
        word.push_str("Hundred");

        triplet %= 100;

        // If the remaining two digits aren't zero, they are further processed
        // We stop at zero, because OneHundredZero isn't a nice number
        if triplet == 0 {
            return;
        }
    }

    // Checks for the next part in the string. This can be the second or third digit,
    // depending on if the second is 0
    word.push_str(match triplet {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        11 => "Eleven",
        12 => "Twelve",
        13 => "Thirteen",
        14 => "Fourteen",
        15 => "Fifteen",
        16 => "Sixteen",
        17 => "Seventeen",
        18 => "Eighteen",
        19 => "Nineteen",
        20..30 => "Twenty",
        30..40 => "Thirty",
        40..50 => "Forty",
        50..60 => "Fifty",
        60..70 => "Sixty",
        70..80 => "Seventy",
        80..90 => "Eighty",
        90..100 => "Ninety",
        _ => "",
    });

    // If the second digit added a syllable, that is still missing the third syllable,
    // and the third syllable is not zero, then we add it through this function.
    // We dont add zeros, because 20 != TwentyZero and its only done for numbers above
    // 20, because 18 != EighteenEight
    if matches!(triplet, 21..100) && triplet % 10 != 0 {
        append_triplet_to_name(word, triplet % 10);
    }
}

/// Converts a whole number to its written form
///
/// The number is formatted in ```PascalCase```, meaning each part starts with an uppercase letter.
///
/// ```rust
/// use marble::number::serialize;
/// assert_eq!(serialize::display_number(42), "FortyTwo");
/// ```
pub fn display_number(mut number: u64) -> String {
    let mut factor = LARGEST;
    let mut word = String::new();

    // We process the number in triplets - batches of three digits
    // This is easy, as phrases like OneHundredFortyTwo can have any factor following them
    while factor != 0 {
        // Extract the triplet from the number
        let triplet = number / factor;

        // We don't check 000, as that would lead to ZeroMillionZeroThousand ...
        // Except for when the factor is one and nothing has been output yet
        // This allows 0 -> Zero
        if triplet > 0 || (factor == 1 && word.is_empty()) {
            append_triplet_to_name(&mut word, triplet);
            word.push_str(factor_name(factor));
        }

        // Remove the triplet and go to the next factor
        number %= factor;
        factor /= 1000;
    }

    word
}

/// Converts a decimal number to its written form
///
/// The number is formatted in ```PascalCase```, meaning each part starts with an uppercase letter.
/// The decimal seperator is the word ```"Point"```
///
/// ```rust
/// use marble::number::serialize;
/// assert_eq!(serialize::display_fraction(3.14), "ThreePointOneFour");
/// ```
pub fn display_fraction(mut number: f64) -> String {
    number = number.abs();

    // The number is built from two parts, the whole and the fraction
    let whole = display_number(number as u64);

    number = number.fract();

    let mut fraction = String::new();

    while number != 0.0 {
        // The number is multiplied by ten, meaning the digits after the decimal point,
        // move from right to left
        number *= 10.0;

        // If the number is close to some other number, we round it
        // This avoids 1.33 -> OnePointThreeThreeZeroZero...One
        if (number.round() - number).abs() < 0.0001 {
            number = number.round();
        }

        // The triplet function is used, but number is always in the range 0..10
        append_triplet_to_name(&mut fraction, number as u64);

        // At last, the processed digit is removed
        number = number.fract();
    }

    // If the fraction is empty, just the whole part is returned
    if !fraction.is_empty() {
        return format!("{whole}Point{fraction}");
    }

    whole
}
