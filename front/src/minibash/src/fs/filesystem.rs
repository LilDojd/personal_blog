/// Stripped down version of mini-fs crate
use std::collections::LinkedList;
use std::io::{Cursor, Error, ErrorKind, Read, Result, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::fs::index;
use crate::fs::{Entries, Entry, Store};
use crate::impl_file;

impl_file! {
    /// Seekable and readable file
    pub enum File {
        InMemory(RamFile),
    }
}

struct Mount {
    path: PathBuf,
    store: Box<dyn Store<File = File>>,
}

/// Virtual filesystem.
pub struct MiniFs {
    mount: LinkedList<Mount>,
}

impl Store for MiniFs {
    type File = File;

    fn open_path(&self, path: &Path) -> Result<File> {
        let next = self.mount.iter().rev().find_map(|mnt| {
            if let Ok(np) = path.strip_prefix(&mnt.path) {
                Some((np, &mnt.store))
            } else {
                None
            }
        });
        if let Some((np, store)) = next {
            store.open_path(np)
        } else {
            Err(Error::from(ErrorKind::NotFound))
        }
    }

    fn entries_path(&self, path: &Path) -> Result<Entries> {
        // FIXME creating a new PathBuf because otherwise I'm getting lifetime errors.
        let path = path.to_path_buf();

        Ok(Entries::new(
            self.mount
                .iter()
                .rev()
                .find(|m| path.strip_prefix(&m.path).is_ok())
                .into_iter()
                .flat_map(move |m| match path.strip_prefix(&m.path) {
                    Ok(np) => m.store.entries_path(np).unwrap(),
                    Err(_) => Entries::new(None),
                }),
        ))
    }
}

impl Default for MiniFs {
    fn default() -> Self {
        Self::new()
    }
}

impl MiniFs {
    pub fn new() -> Self {
        Self {
            mount: LinkedList::new(),
        }
    }

    pub fn mount<P, S, T>(mut self, path: P, store: S) -> Self
    where
        P: Into<PathBuf>,
        S: Store<File = T> + 'static,
        T: Into<File>,
    {
        let path = path.into();
        let store = Box::new(crate::fs::store::MapFile::new(store, |file: T| file.into()));
        self.mount.push_back(Mount { path, store });
        self
    }

    pub fn umount<P>(&mut self, path: P) -> Option<Box<dyn Store<File = File>>>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        if let Some(p) = self.mount.iter().rposition(|p| p.path == path) {
            let mut tail = self.mount.split_off(p);
            let fs = tail.pop_front().map(|m| m.store);
            self.mount.append(&mut tail);
            fs
        } else {
            None
        }
    }
}

/// In-memory file storage
pub struct RamFs {
    index: index::Index<Rc<[u8]>>,
}
pub struct RamFile(Cursor<Rc<[u8]>>);

impl Read for RamFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.read(buf)
    }
}

impl Seek for RamFile {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.0.seek(pos)
    }
}

impl Store for RamFs {
    type File = RamFile;

    fn open_path(&self, path: &Path) -> Result<Self::File> {
        match self.index.get(path) {
            Some(file) => Ok(RamFile(Cursor::new(Rc::clone(file)))),
            None => Err(Error::from(ErrorKind::NotFound)),
        }
    }

    fn entries_path(&self, path: &Path) -> Result<Entries> {
        Ok(Entries::new(self.index.entries(path).map(|ent| {
            Ok(Entry {
                name: ent.name.to_os_string(),
                kind: ent.kind,
            })
        })))
    }
}

impl Default for RamFs {
    fn default() -> Self {
        Self::new()
    }
}

impl RamFs {
    pub fn new() -> Self {
        Self {
            index: index::Index::new(),
        }
    }

    pub fn clear(&mut self) {
        self.index.clear();
    }

    pub fn rm<P: AsRef<Path>>(&mut self, path: P) -> Option<Rc<[u8]>> {
        self.index.remove(path)
    }

    pub fn touch<P, F>(&mut self, path: P, file: F)
    where
        P: Into<PathBuf>,
        F: Into<Rc<[u8]>>,
    {
        self.index.insert(path.into(), file.into());
    }

    pub fn index(self) -> Self {
        self
    }
}
