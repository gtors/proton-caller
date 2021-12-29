use crate::error::{Error, Kind};
use crate::{pass, throw, Version};
use lliw::Fg::LightYellow as Yellow;
use lliw::Reset;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use std::fs::{DirEntry, File, OpenOptions};
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
            return Ok(PathBuf::from(path));
        }

        throw!(Kind::Environment, "$HOME does not exist")
    }

    fn open_cache() -> Result<File, Error> {
        let path: PathBuf = Self::cache_location()?;
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
        {
            Ok(cache) => pass!(cache),
            Err(e) => throw!(Kind::IndexCache, "{}", e),
        }
    }

    fn load(&mut self) -> Result<(), Error> {
        if let Err(e) = self._load() {
            eprintln!("{}warning{}: {}\nreindexing...", Yellow, Reset, e);
            self.index()?;

            if let Err(e) = self.save() {
                eprintln!("{}warning:{} {}\n", Yellow, Reset, e);
            }
        }
        
        Ok(())
    }

    fn _load(&mut self) -> Result<(), Error> {
        let cache: File = Self::open_cache()?;
        self.read_index(cache)?;
        Ok(())
    }

    fn read_index(&mut self, mut f: File) -> Result<(), Error> {
        let mut buf: Vec<u8> = Vec::new();

        if let Err(e) = f.read_to_end(&mut buf) {
            throw!(Kind::IndexCache, "{}", e);
        }

        self.inner = match bincode::deserialize::<Self>(&buf) {
            Ok(c) => c.inner,
            Err(_) => throw!(Kind::IndexCache, "can't deserialize"),
        };

        Ok(())
    }

    fn save(&self) -> Result<(), Error> {
        let mut cache: File = Self::open_cache()?;

        let bytes: Vec<u8> = match bincode::serialize(self) {
            Ok(b) => b,
            Err(e) => throw!(Kind::IndexCache, "{}", e),
        };

        if let Err(e) = cache.write(&bytes) {
            throw!(Kind::IndexCache, "{}", e);
        }

        Ok(())
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
