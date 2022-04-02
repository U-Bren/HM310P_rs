use std::{fmt, str::{FromStr, ParseBoolError}, string::ParseError};
use flagset::{FlagSet, flags};

pub mod hm310p;

flags! {
    pub enum RWCapabilities: u8 {
        Read,
        Write,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseRWError;

impl fmt::Display for ParseRWError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "provided string was not `Read` or `Write`".fmt(f)
    }
}


impl FromStr for RWCapabilities {
    type Err = ParseRWError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "read" => Ok(Self::Read),
            "write" => Ok(Self::Write),
            _ => Err(ParseRWError{}),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Feature {
    pub address: u16,
    pub name: &'static str,
    pub is_rw: FlagSet<RWCapabilities>,
    pub quantity: u16, //number of bytes taken as arg
}

impl Feature {
    pub fn new(address: u16, name: &'static str, is_rw: impl Into<FlagSet<RWCapabilities>>, quantity: u16) -> Self { 
        Self { address,
            name,
            is_rw: is_rw.into(),
            quantity
        } 
    }
    pub fn from_str<'a>(features: &'a Vec<Feature>, feature: &str) -> Option<&'a Feature> {
        features
            .iter()
            .filter(|x| x.name.eq_ignore_ascii_case(&feature))
            .next()
    }
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
