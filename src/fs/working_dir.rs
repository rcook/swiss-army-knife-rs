// Copyright (c) 2023 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use std::env::{current_dir, set_current_dir};
use std::io::Result as IOResult;
use std::path::{Path, PathBuf};

pub struct WorkingDirectory {
    saved_dir: Option<PathBuf>,
}

#[allow(unused)]
impl WorkingDirectory {
    pub fn change<P>(dir: P) -> IOResult<Self>
    where
        P: AsRef<Path>,
    {
        let saved_dir = current_dir()?;
        set_current_dir(dir)?;
        Ok(Self {
            saved_dir: Some(saved_dir),
        })
    }

    pub fn close(&mut self) -> IOResult<()> {
        Ok(match &self.saved_dir {
            Some(d) => {
                set_current_dir(&d)?;
                self.saved_dir = None
            }
            None => (),
        })
    }
}

impl Drop for WorkingDirectory {
    fn drop(&mut self) {
        _ = self.close()
    }
}

#[cfg(test)]
mod tests {
    use super::WorkingDirectory;
    use anyhow::Result;
    use std::env::current_dir;
    use std::path::Path;
    use tempdir::TempDir;

    #[cfg(target_os = "macos")]
    fn compare_dirs<P, Q>(left: P, right: Q) -> bool
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        let left = crate::path_to_str(left.as_ref());
        let left = left.strip_prefix("/private").unwrap_or(left);
        let right = crate::path_to_str(right.as_ref());
        let right = right.strip_prefix("/private").unwrap_or(right);
        left == right
    }

    #[cfg(not(target_os = "macos"))]
    fn compare_dirs<P, Q>(left: P, right: Q) -> bool
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        left.as_ref() == right.as_ref()
    }

    #[test]
    fn test_drop() -> Result<()> {
        let temp_dir = TempDir::new("joatmon-test")?;
        let original_dir = current_dir()?;
        assert!(!compare_dirs(temp_dir.path(), &original_dir));
        let working_dir = WorkingDirectory::change(&temp_dir)?;
        assert!(compare_dirs(temp_dir.path(), current_dir()?));
        drop(working_dir);
        assert!(compare_dirs(&original_dir, current_dir()?));
        Ok(())
    }

    #[test]
    fn test_close_then_drop() -> Result<()> {
        let temp_dir = TempDir::new("joatmon-test")?;
        let original_dir = current_dir()?;
        assert!(!compare_dirs(temp_dir.path(), &original_dir));
        let mut working_dir = WorkingDirectory::change(&temp_dir)?;
        assert!(compare_dirs(temp_dir.path(), current_dir()?));
        working_dir.close()?;
        assert!(compare_dirs(&original_dir, current_dir()?));
        drop(working_dir);
        assert!(compare_dirs(&original_dir, current_dir()?));
        Ok(())
    }
}
