use crate::{pass, throw, Error, Kind};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::path::{Path};
use std::str::FromStr;

/// Version type to handle Proton Versions
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum Version {
    /// Two number version
    Mainline(u8, u8),
    /// Experimental version
    Experimental,
    /// Custom version (will be replaced by Mainline if possible)
    Custom,
}

impl Default for Version {
    fn default() -> Self {
        Version::Mainline(6, 3)
    }
}

impl Version {
    #[must_use]
    /// Creates a new `Version::Mainline` instance
    pub fn new(major: u8, minor: u8) -> Version {
        Version::Mainline(major, minor)
    }

    /// Converts path to custon proton version into Version enum
    pub fn from_custom(name: &Path) -> Version {
        let name_osstr: &OsStr = name.file_name().unwrap_or(&OsStr::new("custom"));
        let name_str: Cow<str> = name_osstr.to_string_lossy();
        let version_str: &str = name_str.split(' ').last().unwrap_or("custom");
        version_str.parse().unwrap_or(Version::Custom)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::Mainline(mj, mn) => write!(f, "{}.{}", mj, mn),
            Version::Experimental => write!(f, "Experimental"),
            Version::Custom => write!(f, "Custom"),
        }
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_ascii_lowercase() == "experimental" {
            return pass!(Version::Experimental);
        }

        match s.split('.').collect::<Vec<&str>>().as_slice() {
            [maj, min] => pass!(Version::new(maj.parse()?, min.parse()?)),
            _ => throw!(Kind::VersionParse, "'{}'", s),
        }
    }
}
