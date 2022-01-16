use lliw::Fg::Red;
use lliw::Reset;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

/// Simple macro rapper for `Result::Ok(T)`
#[macro_export]
macro_rules! pass {
    () => {
        Ok(())
    };
    ($item:expr) => {{
        Ok($item)
    }};
}

/// Macro to throw an error, `Result::Err(e)`
#[macro_export]
macro_rules! throw {
    ($kind:expr, $fmt:literal) => ({
        return $crate::error::_throw($kind, std::format!($fmt))
    });
    ($kind:expr, $fmt:literal, $($arg:tt)*) => ({
        return $crate::error::_throw($kind, std::format!($fmt, $($arg)*))
    })
}

#[doc(hidden)]
pub fn _throw<T>(kind: Kind, inner: String) -> Result<T, Error> {
    Err(Error::new(kind, inner))
}

/// Error type
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Error {
    inner: String,
    // file: Option<String>,
    kind: Kind,
}

impl Error {
    #[must_use]
    /// creates new instance of `Error`
    pub fn new(kind: Kind, inner: String) -> Error {
        Error { inner, kind }
    }

    #[must_use]
    /// returns Error kind
    pub fn kind(&self) -> Kind {
        self.kind
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}: {}{}", Red, self.kind, Reset, self.inner)
    }
}

impl From<ParseIntError> for Error {
    fn from(pie: ParseIntError) -> Self {
        Error::new(Kind::VersionParse, pie.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(te: toml::de::Error) -> Self {
        Error::new(Kind::ConfigParse, te.to_string())
    }
}

impl From<jargon_args::Error> for Error {
    fn from(jae: jargon_args::Error) -> Self {
        match jae {
            jargon_args::Error::MissingArg(key) => {
                Error::new(Kind::ArgumentMissing, key.to_string())
            }
            jargon_args::Error::Other(s) => Error::new(Kind::JargonInternal, s),
        }
    }
}

impl std::error::Error for Error {}

/// Error Kinds
#[repr(i32)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Kind {
    /// for when something weird happens in the program
    Internal = 1,
    /// for when environment variables can't be read
    Environment,
    /// for when the config file fails to be opened
    ConfigOpen,
    /// for when the config file fails to be read
    ConfigRead,
    /// for when Toml fails to parse config
    ConfigParse,
    /// Pfor when creating a Proton directory fails
    ProtonDir,
    /// for when Proton fails to spawn
    ProtonSpawn,
    /// for when waiting for child process fails
    ProtonWait,
    /// for when Proton is not found
    ProtonMissing,
    /// for when requested program is not found
    ProgramMissing,
    /// for when Indexing fails to read common directory
    IndexReadDir,
    /// for when parsing a version number fails
    VersionParse,
    /// for when Proton exits with an error
    ProtonExit,
    /// for when a command line argument is missing
    ArgumentMissing,
    /// for when Jargon has an internal Error,
    JargonInternal,
    /// for when Index failes an action with cache
    IndexCache,
    /// for when parsing RuntimeOption fails,
    ParseRuntimeOpt,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Kind::Internal => "internal error",
                Kind::Environment => "failed to read environment",
                Kind::ConfigOpen => "failed to open config",
                Kind::ConfigRead => "failed to read config",
                Kind::ConfigParse => "failed to parse config",
                Kind::ProtonDir => "failed to create Proton directory",
                Kind::ProtonSpawn => "failed to spawn Proton",
                Kind::ProtonWait => "failed to wait for Proton child",
                Kind::IndexReadDir => "failed to Index",
                Kind::VersionParse => "failed to parse version",
                Kind::ProtonMissing => "cannot find Proton",
                Kind::ProgramMissing => "cannot find program",
                Kind::ProtonExit => "proton exited with",
                Kind::ArgumentMissing => "missing command line argument",
                Kind::JargonInternal => "jargon args internal error",
                Kind::IndexCache => "failed read/write to cache",
                Kind::ParseRuntimeOpt => "failed parsing runtime option",
            }
        )
    }
}
