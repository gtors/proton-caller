#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![forbid(unstable_features)]
#![forbid(missing_fragment_specifier)]
#![warn(clippy::all, clippy::pedantic)]

/*!
# Proton Caller API

This defines the internal API used in `proton-call` to run Proton
*/

mod config;
mod index;
mod runtime;
mod runtime_options;
mod version;

/// Contains the `Error` and `ErrorKind` types
pub mod error;

pub use config::Config;
use error::{Error, Kind};
pub use index::Index;
pub use runtime::RunTimeVersion;
use runtime::Runtime;
pub use runtime_options::RuntimeOption;
use std::borrow::Cow;
use std::fs::create_dir;
pub use version::Version;

use std::path::PathBuf;
use std::process::ExitStatus;

/// Type to handle executing Proton
#[derive(Debug)]
pub struct Proton {
    version: Version,
    path: PathBuf,
    program: PathBuf,
    args: Vec<String>,
    options: Vec<RuntimeOption>,
    compat: PathBuf,
    steam: PathBuf,
    runtime: Option<RunTimeVersion>,
    common: PathBuf,
}

impl Proton {
    #[must_use]
    /// Creates a new instance of `Proton`
    pub fn new(
        version: Version,
        path: PathBuf,
        program: PathBuf,
        args: Vec<String>,
        options: Vec<RuntimeOption>,
        compat: PathBuf,
        steam: PathBuf,
        runtime: Option<RunTimeVersion>,
        common: PathBuf,
    ) -> Proton {
        Proton {
            version,
            path,
            program,
            args,
            options,
            compat,
            steam,
            runtime,
            common,
        }
        .update_path()
    }

    /// Appends the executable to the path
    fn update_path(mut self) -> Proton {
        let str: Cow<str> = self.path.to_string_lossy();
        let str: String = format!("{}/proton", str);
        self.path = PathBuf::from(str);
        self
    }

    fn create_p_dir(&mut self) -> Result<(), Error> {
        let name: Cow<str> = self.compat.to_string_lossy();
        let newdir: PathBuf = PathBuf::from(format!("{}/Proton {}", name, self.version));

        if !newdir.exists() {
            if let Err(e) = create_dir(&newdir) {
                throw!(Kind::ProtonDir, "failed to create Proton directory: {}", e);
            }
        }

        self.compat = newdir;

        pass!()
    }

    fn check_proton(&self) -> Result<(), Error> {
        if !self.path.exists() {
            throw!(Kind::ProtonMissing, "{}", self.version);
        }

        pass!()
    }

    fn check_program(&self) -> Result<(), Error> {
        if !self.program.exists() {
            throw!(Kind::ProgramMissing, "{}", self.program.to_string_lossy());
        }

        pass!()
    }

    fn gen_options(&self) -> Vec<(String, String)> {
        let mut opts = Vec::new();
        for opt in &self.options {
            opts.insert(opts.len(), (opt.to_string(), "1".to_string()))
        }
        opts
    }

    /// Changes `compat` path to the version of Proton in use, creates the directory if doesn't already exist
    ///
    /// # Errors
    ///
    /// Will fail on:
    /// * Creating a Proton compat env directory fails
    /// * Executing Proton fails
    pub fn run(mut self) -> Result<ExitStatus, Error> {
        self.create_p_dir()?;
        self.check_proton()?;
        self.check_program()?;
        if let Some(runtime) = self.runtime {
            let runtime = Runtime::from_proton(runtime, self)?;
            return runtime.execute();
        }
        self.execute()
    }

    /// Executes Proton
    fn execute(self) -> Result<ExitStatus, Error> {
        use std::process::{Child, Command};

        let envs: Vec<(String, String)> = self.gen_options();

        println!(
            "Running Proton {} for {} with:\n{:#?}",
            self.version,
            self.program.to_string_lossy(),
            envs,
        );

        let mut child: Child = match Command::new(&self.path)
            .arg("run")
            .arg(&self.program)
            .args(&self.args)
            .env("STEAM_COMPAT_DATA_PATH", &self.compat)
            .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &self.steam)
            .envs(envs)
            .spawn()
        {
            Ok(c) => c,
            Err(e) => throw!(Kind::ProtonSpawn, "{}\nDebug:\n{:#?}", e, self),
        };

        let status: ExitStatus = match child.wait() {
            Ok(e) => e,
            Err(e) => throw!(Kind::ProtonWait, "'{}': {}", child.id(), e),
        };

        pass!(status)
    }
}
