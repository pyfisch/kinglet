extern crate kinglet;

use kinglet::IterListHeader;

#[test]
fn test1() {
    let values = vec![];
    let mut iter = IterListHeader::new(&values);
    assert!(iter.next().is_none());
}

#[test]
fn test2() {
    let values = vec![b"foo".to_vec()];
    let mut iter = IterListHeader::new(&values);
    assert_eq!(iter.next(), Some(&b"foo"[..]));
    assert!(iter.next().is_none());
}

#[test]
fn test3() {
    let values = vec![b"foo, bar".to_vec()];
    let mut iter = IterListHeader::new(&values);
    assert_eq!(iter.next(), Some(&b"foo"[..]));
    assert_eq!(iter.next(), Some(&b"bar"[..]));
    assert!(iter.next().is_none());
}

#[test]
fn test4() {
    let values = vec![b"foo , bar".to_vec()];
    let mut iter = IterListHeader::new(&values);
    assert_eq!(iter.next(), Some(&b"foo"[..]));
    assert_eq!(iter.next(), Some(&b"bar"[..]));
    assert!(iter.next().is_none());
}

#[test]
fn test5() {
    let values = vec![b"one".to_vec(), b"two".to_vec(), b"three".to_vec()];
    let mut iter = IterListHeader::new(&values);
    assert_eq!(iter.next(), Some(&b"one"[..]));
    assert_eq!(iter.next(), Some(&b"two"[..]));
    assert_eq!(iter.next(), Some(&b"three"[..]));
    assert!(iter.next().is_none());
}

#[test]
fn test6() {
    let values = vec![b"one".to_vec(), b"two,,three".to_vec()];
    let mut iter = IterListHeader::new(&values);
    assert_eq!(iter.next(), Some(&b"one"[..]));
    assert_eq!(iter.next(), Some(&b"two"[..]));
    assert_eq!(iter.next(), Some(&b"three"[..]));
    assert!(iter.next().is_none());
}

#[test]
fn test7() {
    let values = vec![b", , , ,,,,,, ,".to_vec(), b"       ,,, ,,,, ,, ".to_vec()];
    let mut iter = IterListHeader::new(&values);
    assert!(iter.next().is_none());
}

#[test]
fn test8() {
    let values = vec![b"!, this value has spaces, ;,  ".to_vec()];
    let mut iter = IterListHeader::new(&values);
    assert_eq!(iter.next(), Some(&b"!"[..]));
    assert_eq!(iter.next(), Some(&b"this value has spaces"[..]));
    assert_eq!(iter.next(), Some(&b";"[..]));
    assert!(iter.next().is_none());
}

#[test]
fn test9() {
    let values = vec![b"anton, bertha,".to_vec(), b"caesar, \t".to_vec()];
    let mut iter = IterListHeader::new(&values);
    assert_eq!(iter.next(), Some(&b"anton"[..]));
    assert_eq!(iter.next(), Some(&b"bertha"[..]));
    assert_eq!(iter.next(), Some(&b"caesar"[..]));
    assert!(iter.next().is_none());
}
