extern crate serde;
extern crate toml;

use crate::{
    error::{Error, Kind},
    throw,
};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

/// Config type for parsing config files
#[derive(Debug, serde::Deserialize)]
pub struct Config {
    data: PathBuf,
    steam: PathBuf,
    common: Option<PathBuf>,
}

impl Config {
    /// Opens and returns the user's config
    ///
    /// # Errors
    ///
    /// This function will fail if...
    /// * Can not read `XDG_CONFIG_HOME` or `HOME` from the environment
    /// * Can not open config file
    /// * Can not parse config into `Config`
    pub fn open() -> Result<Config, Error> {
        use std::fs::File;
        use std::io::Read;

        // Get default config location
        let loc: PathBuf = Config::config_location()?;

        // Open the config file
        let mut file: File = match File::open(&loc) {
            Ok(f) => f,
            Err(e) => throw!(Kind::ConfigOpen, "{}", e),
        };

        // Read the config into memory
        let mut buffer: Vec<u8> = Vec::new();

        if let Err(e) = file.read_to_end(&mut buffer) {
            throw!(Kind::ConfigRead, "{}", e);
        }

        // Parse the config into `Config`
        let slice: &[u8] = buffer.as_slice();

        let mut config: Config = toml::from_slice(slice)?;

        config.default_common();

        Ok(config)
    }

    /// Finds one of the two default config locations
    ///
    /// # Errors
    ///
    /// Will only fail if `XDG_CONFIG_HOME` and `HOME` do not exist in environment
    pub fn config_location() -> Result<PathBuf, Error> {
        use std::env::var;

        if let Ok(val) = var("XDG_CONFIG_HOME") {
            let path: String = format!("{}/proton.conf", val);
            return Ok(PathBuf::from(path));
        }

        match var("HOME") {
            Ok(var) => Ok(PathBuf::from(format!("{}/.config/proton.conf", var))),
            Err(_) => throw!(Kind::Environment, "XDG_CONFIG_HOME / HOME missing"),
        }
    }

    #[inline]
    /// Sets a default common if not given by user
    fn default_common(&mut self) {
        if self.common.is_none() {
            let common: PathBuf = self._default_common();
            self.common = Some(common);
        }
    }

    #[must_use]
    /// Generates a default common directory
    fn _default_common(&self) -> PathBuf {
        eprintln!("warning: using default common");
        let steam: Cow<str> = self.steam.to_string_lossy();
        let common_str: String = format!("{}/steamapps/common/", steam);
        PathBuf::from(common_str)
    }

    #[must_use]
    #[inline]
    /// Returns the in use common directory
    pub fn common(&self) -> PathBuf {
        if let Some(common) = &self.common {
            common.clone()
        } else {
            self._default_common()
        }
    }

    #[must_use]
    #[inline]
    /// Returns the in use steam directory
    pub fn steam(&self) -> PathBuf {
        self.steam.clone()
    }

    #[must_use]
    #[inline]
    /// Returns the in use compat data directory
    pub fn data(&self) -> PathBuf {
        self.data.clone()
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data: Cow<str> = self.data.to_string_lossy();
        let steam: Cow<str> = self.steam.to_string_lossy();

        let common: String = if let Some(common) = &self.common {
            common.to_string_lossy().to_string()
        } else {
            let pb: PathBuf = self._default_common();
            pb.to_string_lossy().to_string()
        };

        write!(f, "steam: {}\ndata: {}\ncommon: {}", steam, data, common)
    }
}
