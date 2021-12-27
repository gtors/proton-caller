use crate::{pass, throw, Error, Kind};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::path::Path;
use std::str::FromStr;

/// Version type to handle Proton Versions
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
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

    #[must_use]
    /// Tries parsing custom Proton path into `Version::Mainline`
    pub fn from_custom(name: &Path) -> Version {
        if let Some(n) = name.file_name() {
            let name_str = n.to_string_lossy().to_string();
            if let Some(version_str) = name_str.split(' ').last() {
                if let Ok(version) = version_str.parse() {
                    return version;
                }
            }
        }

        Version::Custom
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

impl Hash for Version {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
