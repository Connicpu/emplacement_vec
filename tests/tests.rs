#![feature(placement_in_syntax)]

extern crate emplacement_vec;

use emplacement_vec::EmplacementVec;

#[test]
fn push_pop() {
    let mut vec = EmplacementVec::new();
    in vec.push() { "Berber!" };
    in vec.push() { "There!" };
    in vec.push() { "Hi!" };

    assert_eq!(vec.len(), 3);

    assert_eq!(vec.pop(), Some("Hi!"));
    assert_eq!(vec.pop(), Some("There!"));
    assert_eq!(vec.pop(), Some("Berber!"));
    assert_eq!(vec.pop(), None);
    assert_eq!(vec.pop(), None);

    assert_eq!(vec.len(), 0);
}
