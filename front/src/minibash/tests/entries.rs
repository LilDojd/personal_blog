use minibash::fs::prelude::*;
use minibash::fs::{EntryKind, RamFs};
use std::collections::BTreeMap;
use std::ffi::OsStr;

// https://github.com/germangb/mini-fs/issues/6
#[test]
fn ram_fs_entries_kind() {
    let mut ram = RamFs::new();

    ram.touch("/a.txt", b"low a".to_vec());
    ram.touch("/A.TXT", b"high a".to_vec());
    ram.touch("/b/b.txt", b"low b".to_vec());
    ram.touch("/B/B.TXT", b"high b".to_vec());

    let mut map = BTreeMap::new();
    for entry in ram.entries("/").unwrap() {
        let entry = entry.unwrap();
        map.insert(entry.name, entry.kind);
    }

    assert_eq!(Some(&EntryKind::File), map.get(OsStr::new("a.txt")));
    assert_eq!(Some(&EntryKind::File), map.get(OsStr::new("A.TXT")));
    assert_eq!(Some(&EntryKind::Dir), map.get(OsStr::new("b")));
    assert_eq!(Some(&EntryKind::Dir), map.get(OsStr::new("B")));
}
