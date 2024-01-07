use std::{
    convert::Infallible,
    fmt::Display,
    path::{Path, PathBuf},
    process::ExitStatus,
    str::FromStr,
};

use crate::{
    error::{Error, Kind},
    pass, throw, Proton,
};

#[derive(Debug)]
pub struct Runtime {
    version: RunTimeVersion,
    path: PathBuf,
    proton: Proton,
}

impl Runtime {
    pub fn from_proton(version: RunTimeVersion, proton: Proton) -> Result<Self, Error> {
        Ok(Self {
            version,
            path: Self::find(&proton.common, version)?,
            proton,
        })
    }

    pub fn execute(self) -> Result<ExitStatus, Error> {
        use std::process::{Child, Command};

        let envs: Vec<(String, String)> = self.proton.gen_options();

        let mut child: Child = match Command::new(&self.path)
        .arg(&self.proton.path)
        .arg("runinprefix")
        .arg(&self.proton.program)
        .args(&self.proton.args)
        .env("STEAM_COMPAT_DATA_PATH", &self.proton.compat)
        .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &self.proton.steam)
        .envs(envs)
        .spawn() {
            Ok(child) => child,
            Err(e) => throw!(Kind::ProtonExit, "{}", e),
        };


        let status: ExitStatus = match child.wait() {
            Ok(e) => e,
            Err(e) => throw!(Kind::ProtonWait, "'{}': {}", child.id(), e),
        };

        pass!(status)
    }

    pub fn find(common: &Path, version: RunTimeVersion) -> Result<PathBuf, Error> {
        let tmp = format!("{}/{}/run", common.display(), version);
        let path = PathBuf::from(tmp);

        if path.exists() {
            pass!(path)
        } else {
            throw!(Kind::RuntimeMissing, "{}", version)
        }
    }
}

/// Enum to represet Steam runtime versions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RunTimeVersion {
    /// Default version of Steam's runtime
    Default,
    /// Sniper version of Steam's runtime
    Sniper,
    /// Soldier version of Steam's runtime
    Soldier,
    /// BattleEye version of Steam's runtime
    BattleEye,
    /// EasyAntiCheat version of Steam's runtime
    EasyAntiCheat,
}

impl Display for RunTimeVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunTimeVersion::Default => write!(f, "SteamLinuxRuntime"),
            RunTimeVersion::Sniper => write!(f, "SteamLinuxRuntime_sniper"),
            RunTimeVersion::Soldier => write!(f, "SteamLinuxRuntime_soldier"),
            RunTimeVersion::BattleEye => write!(f, "Proton BattlEye Runtime"),
            RunTimeVersion::EasyAntiCheat => write!(f, "Proton EasyAntiCheat Runtime"),
        }
    }
}

impl FromStr for RunTimeVersion {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "default" => Self::Default,
            "soldier" => Self::Soldier,
            "sniper" => Self::Sniper,
            "battleeye" => Self::BattleEye,
            "eac" | "easyanticheat" => Self::EasyAntiCheat,
            _ => Self::Default,
        })
    }
}
