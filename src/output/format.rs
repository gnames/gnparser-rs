use std::convert::From;

#[derive(Debug)]
pub enum Format {
    Simple,
    Compact,
    Pretty,
}

impl Default for Format {
    fn default() -> Self {
        Format::Compact
    }
}

impl From<&str> for Format {
    fn from(s: &str) -> Self {
        match s {
            "simple" => Format::Simple,
            "compact" => Format::Compact,
            "pretty" => Format::Pretty,
            _ => Format::Compact,
        }
    }
}
