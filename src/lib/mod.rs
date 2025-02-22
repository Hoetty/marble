use environment::Value;

pub mod scanner;
pub mod source;
pub mod token;
pub mod compiler;
pub mod ast;
pub mod error;
pub mod interpreter;
pub mod environment;
pub mod builtin;

/// A crate to handle numbers and their written forms
///  
/// Provides utility to convert numbers to words
/// ```rust 
/// use marble::number::serialize;
/// assert_eq!(serialize::display_number(123), "OneHundredTwentyThree");
/// ```
/// 
/// And utility to parse such strings to numbers
/// 
/// ```rust
/// use marble::number::deserialize;
/// assert_eq!(deserialize::parse_number("OneHundredTwentyThree"), Some(123));
/// ```
/// 
/// Please note, that serialization will always return a string directly, 
/// because all numbers can be converted to a word. However in contrast, desirialization
/// returns an Option, as it may fail. *```"Banana"``` isn't a number after all*.
pub mod number;