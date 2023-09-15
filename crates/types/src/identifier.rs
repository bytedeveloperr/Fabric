use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Identifier {
    U32(u32),
    U64(u64),
    String(String),
    Custom(Vec<u8>)
}

impl Identifier {
    pub fn u32(value: u32) -> Identifier {
        Identifier::U32(value)
    }

    pub fn u64(value: u64) -> Identifier {
        Identifier::U64(value)
    }

    pub fn string(value: String) -> Identifier {
        Identifier::String(value)
    }

    pub fn custom(value: Vec<u8>) -> Identifier {
        Identifier::Custom(value)
    }
}

impl FromStr for Identifier {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Identifier::String(s.to_string()))
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Identifier::string(value)
    }
}

impl From<u32> for Identifier {
    fn from(value: u32) -> Self {
        Identifier::u32(value)
    }
}

impl From<u64> for Identifier {
    fn from(value: u64) -> Self {
        Identifier::u64(value)
    }
}

impl From<Vec<u8>> for Identifier {
    fn from(value: Vec<u8>) -> Self {
        Identifier::custom(value)
    }
}

impl From<Identifier> for u32 {
    fn from(value: Identifier) -> Self {
        match value {
            Identifier::U32(value) => value,
            _ => panic!("Error: Identifier must be of type u32")
        }
    }
}

impl From<Identifier> for u64 {
    fn from(value: Identifier) -> Self {
        match value {
            Identifier::U64(value) => value,
            _ => panic!("Error: Identifier must be of type u64")
        }
    }
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        match value {
            Identifier::String(value) => value,
            _ => panic!("Error: Identifier must be of type String")
        }
    }
}

impl From<Identifier> for Vec<u8> {
    fn from(value: Identifier) -> Self {
        match value {
            Identifier::U32(value) => value.to_le_bytes().to_vec(),
            Identifier::U64(value) => value.to_le_bytes().to_vec(),
            Identifier::String(value) => value.into_bytes(),
            Identifier::Custom(value) => value,
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
         match self {
            Identifier::U32(value) => writeln!(f, "{}", value),
            Identifier::U64(value) => writeln!(f, "{}", value),
            Identifier::String(value) => writeln!(f, "{}", value),
            Identifier::Custom(value) => writeln!(f, "{:?}", value),
        }
    }
}