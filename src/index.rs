use crate::error::{Error, Kind};
use crate::{pass, throw, Version};
use lliw::Fg::LightYellow as Yellow;
use lliw::Reset;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

/// Index type to Index Proton versions in common
#[derive(Debug)]
pub struct Index {
    dir: PathBuf,
    inner: BTreeMap<Version, PathBuf>,
}

impl Display for Index {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str: String = format!(
            "Indexed Directory: {}\n\nIndexed {} Proton Versions:\n",
            self.dir.to_string_lossy(),
            self.len()
        );

        for (version, path) in &self.inner {
            str = format!("{}\nProton {} `{}`", str, version, path.display());
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
            inner: BTreeMap::new(),
        };

        idx.index()?;

        Ok(idx)
    }

    #[must_use]
    /// Returns the number of Indexed Protons
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    /// Returns true if Index is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[must_use]
    /// Retrieves the path of the requested Proton version
    pub fn get(&self, version: Version) -> Option<PathBuf> {
        let path = self.inner.get(&version)?;
        Some(path.clone())
    }

    /// Indexes Proton versions
    fn index(&mut self) -> Result<(), Error> {
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
