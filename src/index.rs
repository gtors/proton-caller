use crate::error::{Error, Kind};
use crate::{pass, throw, Version};
use lliw::Fg::LightYellow as Yellow;
use lliw::Reset;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use std::fs::{DirEntry, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Index type to Index Proton versions in common
#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    dir: PathBuf,
    inner: HashMap<Version, PathBuf>,
}

impl Display for Index {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str: String = format!(
            "Indexed Directory: {}\n\nIndexed {} Proton Versions:\n",
            self.dir.to_string_lossy(),
            self.len()
        );

        for (version, path) in &self.inner {
            str = format!("{}\nProton {}: {}", str, version, path.display());
        }

        write!(f, "{}", str)
    }
}

impl Index {
    /// Creates an index of Proton versions in given path
    ///
    /// # Errors
    ///
    /// Will fail if Indexing fails to read the directory
    pub fn new(index: &Path) -> Result<Index, Error> {
        let mut idx = Index {
            dir: index.to_path_buf(),
            inner: HashMap::new(),
        };

        idx.load()?;
        Ok(idx)
    }

    #[must_use]
    #[inline]
    /// Returns the number of Indexed Protons
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    #[inline]
    /// Returns true if Index is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[must_use]
    #[inline]
    /// Retrieves the path of the requested Proton version
    pub fn get(&self, version: &Version) -> Option<PathBuf> {
        self.inner.get(version).map(std::clone::Clone::clone)
    }

    fn cache_location() -> Result<PathBuf, Error> {
        use std::env::var;

        if let Ok(val) = var("HOME") {
            let path = format!("{}/.cache/proton/index", val);
            Ok(PathBuf::from(path))
        } else {
            throw!(Kind::Environment, "XDG_CONFIG_HOME / HOME missing")
        }
    }

    fn load(&mut self) -> Result<(), Error> {
        if let Ok(path) = Self::cache_location() {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(path)
            {
                let mut buf: Vec<u8> = Vec::new();
                if file.read_to_end(&mut buf).is_ok() {
                    if let Ok(index) = bincode::deserialize::<Self>(&buf) {
                        self.dir = index.dir;
                        self.inner = index.inner;
                        return Ok(());
                    }
                }
            }
        }

        eprintln!("{}warning:{} failed to load index… reindexing...", Yellow, Reset);

        self.index()?;
        self.save();

        Ok(())
    }

    fn save(&self) {
        if let Ok(path) = Self::cache_location() {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(path)
            {
                if let Ok(index) = bincode::serialize(self) {
                    if file.write(&index).is_err() {
                        eprintln!("{}warning:{} failed writing to cached index", Yellow, Reset);
                    }
                }
            }
        }

        eprintln!("{}warning:{} failed opening cached index", Yellow, Reset);
    }

    /// Indexes Proton versions
    /// # Errors
    /// An error is returned when the function cannot read the common directory
    pub fn index(&mut self) -> Result<(), Error> {
        if let Ok(rd) = self.dir.read_dir() {
            for result_entry in rd {
                let entry: DirEntry = if let Ok(e) = result_entry {
                    e
                } else {
                    eprintln!("{}warning:{} failed indexing a directory...", Yellow, Reset);
                    continue;
                };

                let entry_path: PathBuf = entry.path();

                if entry_path.is_dir() {
                    let name: OsString = entry.file_name();
                    let name: String = name.to_string_lossy().to_string();
                    if let Some(version_str) = name.split(' ').last() {
                        if let Ok(version) = version_str.parse() {
                            self.inner.insert(version, entry_path);
                        }
                    }
                }
            }
        } else {
            throw!(Kind::IndexReadDir, "can not read common dir");
        }

        pass!()
    }
}
