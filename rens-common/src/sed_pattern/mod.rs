/* Modules */
mod flag;
/* Built-in imports */
use core::str::FromStr;
/* Crate imports */
use flag::Flag;
/* Dependencies */
use regex::{Regex, RegexBuilder};

#[derive(Debug, Clone)]
pub struct SedPattern {
    pattern: Regex,
    with: String,
    limit: usize,
}

impl SedPattern {
    #[must_use]
    #[inline]
    pub fn export(self) -> (Regex, String, usize) {
        (self.pattern, self.with, self.limit)
    }
}

impl FromStr for SedPattern {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut chars = input.chars().peekable();
        let separator = chars.next().ok_or(Self::Err::Empty)?;

        let mut parse_segment = || -> Result<String, Self::Err> {
            let mut buf = String::new();
            loop {
                match chars.next() {
                    None => return Err(Self::Err::MissingSegment),
                    Some('\\') if chars.peek() == Some(&separator) => {
                        chars.next();
                        buf.push(separator);
                    },
                    Some(ch) if ch == separator => break,
                    Some(ch) => buf.push(ch),
                }
            }
            Ok(buf)
        };

        let mut rb = RegexBuilder::new(&parse_segment()?);
        let with = parse_segment()?;
        let mut limit = 0;

        Flag::list_from_chars(chars)?.iter().for_each(|&flag| {
            match flag {
                Flag::Global => limit = 0,
                Flag::CaseInsensitive => {
                    rb.case_insensitive(true);
                },
                Flag::CaseSensitive => {
                    rb.case_insensitive(false);
                },
                Flag::GreedySwap => {
                    rb.swap_greed(true);
                },
                Flag::IgnoreWhitespaces => {
                    rb.ignore_whitespace(true);
                },
                Flag::Numbered(num) => limit = num,
            };
        });

        Ok(Self {
            pattern: rb.build()?,
            with,
            limit,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Empty pattern")]
    Empty,
    #[error("Missing segment")]
    MissingSegment,
    #[error("{0}")]
    Regex(#[from] regex::Error),
    #[error("{0}")]
    Flag(#[from] flag::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    #[test]
    fn test_valid_pattern_with_options() {
        let sed_pattern = SedPattern::from_str("/foo/bar/g").unwrap();

        assert_eq!(sed_pattern.pattern.as_str(), "foo");
        assert_eq!(sed_pattern.with, "bar");
        assert_eq!(sed_pattern.limit, 0);
    }

    #[test]
    fn test_valid_pattern_with_various_separators() {
        for separator in ['x', ';', '/'] {
            let pat = format!("{separator}foo{separator}bar{separator}gi");
            let sed_pattern = SedPattern::from_str(&pat).unwrap();

            assert_eq!(sed_pattern.pattern.as_str(), "foo");
            assert_eq!(sed_pattern.with, "bar");
            assert_eq!(sed_pattern.limit, 0);
        }
    }

    #[test]
    fn test_empty_segments() {
        SedPattern::from_str("/foo//").unwrap();
        SedPattern::from_str("///").unwrap();
        SedPattern::from_str("//ee/").unwrap();
        SedPattern::from_str("///g").unwrap();
    }

    #[test]
    fn test_pattern_with_escaped_separator() {
        let sed_pattern = SedPattern::from_str("/foo\\/bar/baz/").unwrap();

        assert_eq!(sed_pattern.pattern.as_str(), "foo/bar");
        assert_eq!(sed_pattern.with, "baz");
        assert_eq!(sed_pattern.limit, 0);
    }

    #[test]
    fn test_replacement_with_escaped_separator() {
        let sed_pattern = SedPattern::from_str("/foo/bar\\/baz/").unwrap();

        assert_eq!(sed_pattern.pattern.as_str(), "foo");
        assert_eq!(sed_pattern.with, "bar/baz");
        assert_eq!(sed_pattern.limit, 0);
    }

    #[test]
    fn test_special_characters() {
        let sed_pattern = SedPattern::from_str("/f.o*/bar/g").unwrap();

        assert_eq!(sed_pattern.pattern.as_str(), "f.o*",);
        assert!(sed_pattern.pattern.is_match("foooooooo"));
        assert_eq!(sed_pattern.with, "bar",);
        assert_eq!(sed_pattern.limit, 0);
    }

    #[test]
    fn test_empty_input_pattern() {
        assert!(
            matches!(SedPattern::from_str(""), Err(Error::Empty)),
            "Expected Error::Empty"
        );
    }

    #[test]
    fn test_missing_segment() {
        for pat in ["//", "/", "/foo/", "/foo\\/", "/foo/bar\\/"] {
            let res = SedPattern::from_str(pat);
            assert!(
                matches!(res, Err(Error::MissingSegment)),
                "Expected Error::MissingSegment, got {res:#?} for {pat}"
            );
        }
    }
}
