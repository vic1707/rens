/* Dependencies */
use clap::{builder::PossibleValue, ValueEnum};

#[derive(Debug, Default, Clone)]
pub enum GlobalOrNumbered {
    #[default]
    Global,
    Numbered(usize),
}

impl ValueEnum for GlobalOrNumbered {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Global, Self::Numbered(0)]
    }

    fn from_str(input: &str, _: bool) -> Result<Self, String> {
        match input {
            "g" | "global" => Ok(Self::Global),
            str => match str.parse::<usize>() {
                Ok(num) => Ok(Self::Numbered(num)),
                Err(err) => Err(err.to_string()),
            },
        }
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match *self {
            Self::Global => Some(
                PossibleValue::new("Global").help("Replace all found matches."),
            ),
            Self::Numbered(_) => Some(
                PossibleValue::new("<number>")
                    .help("Replace <number> matches."),
            ),
        }
    }
}

impl From<GlobalOrNumbered> for usize {
    fn from(val: GlobalOrNumbered) -> Self {
        match val {
            GlobalOrNumbered::Global => 0,
            GlobalOrNumbered::Numbered(num) => num,
        }
    }
}
