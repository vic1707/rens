/* Built-in imports */
use core::{iter::Peekable, str::Chars};

/// A single regex flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    /// Regex should be case insensitive. Corresponds to `i`.
    CaseInsensitive,
    /// Regex should be case sensitive. Corresponds to `I`.
    CaseSensitive,
    /// Regex is run for every match in a string. Corresponds to `g`.
    Global,
    /// Regex is run N times in a string.
    Numbered(usize),
    /// "Greedy swap" flag. Corresponds to `U`.
    GreedySwap,
    /// Ignore whitespaces. Corresponds to `x`.
    IgnoreWhitespaces,
}

impl Flag {
    pub fn list_from_chars(
        mut chars: Peekable<Chars>,
    ) -> Result<Box<[Self]>, Error> {
        let mut flags = Vec::new();

        while let Some(ch) = chars.next() {
            if ch.is_ascii_digit() {
                let num = 10 * char_to_digit(ch)
                    + chars
                        .next_if(char::is_ascii_digit)
                        .into_iter()
                        .fold(0, |acc, cur| acc * 10 + char_to_digit(cur));
                flags.push(Self::Numbered(num));
            } else {
                flags.push(Self::try_from(ch)?);
            }
        }

        Ok(flags.into_boxed_slice())
    }
}

#[inline]
#[allow(clippy::as_conversions)]
fn char_to_digit(ch: char) -> usize {
    debug_assert!(ch.is_ascii_digit(), "ch is not in '0'..'9'");
    usize::from(ch as u8 - b'0')
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("Unknown sed flag: {0}")]
    UnknownFlag(char),
}

impl TryFrom<char> for Flag {
    type Error = Error;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'i' => Ok(Self::CaseInsensitive),
            'I' => Ok(Self::CaseSensitive),
            'g' => Ok(Self::Global),
            'U' => Ok(Self::GreedySwap),
            'x' => Ok(Self::IgnoreWhitespaces),
            _ => Err(Self::Error::UnknownFlag(ch)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_flags() {
        let chars: Peekable<Chars> = "i12g34U".chars().peekable();
        let flags = Flag::list_from_chars(chars).unwrap();
        assert_eq!(
            flags.as_ref(),
            vec![
                Flag::CaseInsensitive,
                Flag::Numbered(12),
                Flag::Global,
                Flag::Numbered(34),
                Flag::GreedySwap,
            ]
        );
    }

    #[test]
    fn test_invalid_flag() {
        let chars: Peekable<Chars> = "iX".chars().peekable();
        let result = Flag::list_from_chars(chars);
        assert_eq!(result.unwrap_err(), Error::UnknownFlag('X'));
    }

    #[test]
    fn test_empty_input() {
        let chars = "".chars().peekable();
        let flags = Flag::list_from_chars(chars).unwrap();
        assert!(flags.is_empty());
    }
}
