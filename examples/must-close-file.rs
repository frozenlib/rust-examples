// Force the user to call `MustCloseFile::close` so that errors that occur when dropping `File` are not ignored.

use std::{
    fs::File,
    io::{Result, Write},
    ops::{Deref, DerefMut},
    path::Path,
};

fn main() {
    MustCloseFile::open_with("file.txt", |mut file| {
        file.write_all(&[1, 2, 3])?;
        file.close()
    })
    .expect("write failed.");

    /*
    MustCloseFile::open_with("file.txt", |mut file| {
        file.write_all(&[1, 2, 3])?;
        Ok(()) // Compilation error because IO errors may be ignored by auto-drop
    })
    .expect("write failed.");
    */
}

pub struct MustCloseFile(File);

impl MustCloseFile {
    pub fn open_with<T>(
        path: impl AsRef<Path>,
        f: impl FnOnce(Self) -> Result<ClosedFile<T>>,
    ) -> Result<T> {
        Ok(f(MustCloseFile(File::open(path)?))?.0)
    }
    pub fn close(self) -> Result<ClosedFile> {
        self.0.sync_all()?;
        Ok(ClosedFile(()))
    }
}

impl Deref for MustCloseFile {
    type Target = File;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MustCloseFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[non_exhaustive]
pub struct ClosedFile<T = ()>(T);
