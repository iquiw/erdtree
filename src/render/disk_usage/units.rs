use clap::ValueEnum;
use std::{
    convert::From,
    fmt::{self, Display},
};

/// Determines whether to use SI prefixes or binary prefixes.
#[derive(Copy, Clone, Debug, ValueEnum, Default)]
pub enum PrefixKind {
    /// Displays disk usage using binary prefixes.
    #[default]
    Bin,

    /// Displays disk usage using SI prefixes.
    Si,
}

/// Binary prefixes.
#[derive(Debug)]
pub enum BinPrefix {
    Base,
    Kibi,
    Mebi,
    Gibi,
    Tebi,
}

/// SI prefixes.
#[derive(Debug)]
pub enum SiPrefix {
    Base,
    Kilo,
    Mega,
    Giga,
    Tera,
}

pub trait UnitPrefix {
    fn base_value(&self) -> u64;
}

impl UnitPrefix for SiPrefix {
    fn base_value(&self) -> u64 {
        match self {
            Self::Base => 1,
            Self::Kilo => 10_u64.pow(3),
            Self::Mega => 10_u64.pow(6),
            Self::Giga => 10_u64.pow(9),
            Self::Tera => 10_u64.pow(12),
        }
    }
}

impl UnitPrefix for BinPrefix {
    fn base_value(&self) -> u64 {
        match self {
            Self::Base => 1,
            Self::Kibi => 2_u64.pow(10),
            Self::Mebi => 2_u64.pow(20),
            Self::Gibi => 2_u64.pow(30),
            Self::Tebi => 2_u64.pow(40),
        }
    }
}

/// Get the closest human-readable unit prefix for value.
impl From<f64> for BinPrefix {
    fn from(value: f64) -> Self {
        let log = value.log2();

        if log < 10. {
            Self::Base
        } else if log < 20. {
            Self::Kibi
        } else if log < 30. {
            Self::Mebi
        } else if log < 40. {
            Self::Gibi
        } else {
            Self::Tebi
        }
    }
}

/// Get the closest human-readable unit prefix for value.
impl From<f64> for SiPrefix {
    fn from(value: f64) -> Self {
        let log = value.log10();

        if log < 3. {
            Self::Base
        } else if log < 6. {
            Self::Kilo
        } else if log < 9. {
            Self::Mega
        } else if log < 12. {
            Self::Giga
        } else {
            Self::Tera
        }
    }
}

impl Display for BinPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base => f.pad("B"),
            Self::Kibi => f.pad("KiB"),
            Self::Mebi => f.pad("MiB"),
            Self::Gibi => f.pad("GiB"),
            Self::Tebi => f.pad("TiB"),
        }
    }
}

impl Display for SiPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base => f.pad("B"),
            Self::Kilo => f.pad("KB"),
            Self::Mega => f.pad("MB"),
            Self::Giga => f.pad("GB"),
            Self::Tera => f.pad("TB"),
        }
    }
}
