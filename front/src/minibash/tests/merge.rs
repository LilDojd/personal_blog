use std::io::Read;

#[test]
fn merge_tup() {
    use minibash::fs::prelude::*;
    use minibash::fs::{MiniFs, RamFs};

    let mut a = RamFs::new();
    let mut b = RamFs::new();

    a.touch("a.txt", String::from("a.txt").into_bytes());
    a.touch("b.txt", String::from("b.txt").into_bytes());
    b.touch("a.txt", String::from("overriden").into_bytes());
    b.touch("c.txt", String::from("c.txt").into_bytes());

    let fs: MiniFs = MiniFs::new().mount("/files", (b, a));

    assert!(fs.open("/files/a.txt").is_ok());
    assert!(fs.open("/files/b.txt").is_ok());
    assert!(fs.open("/files/c.txt").is_ok());

    let mut atxt = String::new();

    let mut file = fs.open("/files/a.txt").unwrap();
    file.read_to_string(&mut atxt).unwrap();

    assert_eq!("overriden", atxt);
}

#[test]
fn merge_vec() {
    use minibash::fs::prelude::*;
    use minibash::fs::{MiniFs, RamFs};

    let mut a = RamFs::new();
    let mut b = RamFs::new();

    a.touch("a.txt", String::from("a.txt").into_bytes());
    a.touch("b.txt", String::from("b.txt").into_bytes());
    b.touch("a.txt", String::from("overriden").into_bytes());
    b.touch("c.txt", String::from("c.txt").into_bytes());

    let fs: MiniFs = MiniFs::new().mount("/files", vec![b, a]);

    assert!(fs.open("/files/a.txt").is_ok());
    assert!(fs.open("/files/b.txt").is_ok());
    assert!(fs.open("/files/c.txt").is_ok());

    let mut atxt = String::new();

    let mut file = fs.open("/files/a.txt").unwrap();
    file.read_to_string(&mut atxt).unwrap();

    assert_eq!("overriden", atxt);
}
