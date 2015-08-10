#![feature(placement_in_syntax)]

extern crate emplacement_vec;

use std::borrow::Borrow;
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

#[test]
fn borrowing() {
    let mut vec = EmplacementVec::new();
    in vec.push() { "Berber!" };
    in vec.push() { "There!" };
    in vec.push() { "Hi!" };

    let borrowed: &[&str] = vec.borrow();
    assert_eq!(borrowed[0], "Berber!");
    assert_eq!(borrowed[1], "There!");
    assert_eq!(borrowed[2], "Hi!");
}

#[test]
fn reserving() {
    let mut vec = EmplacementVec::new();
    vec.reserve(3);

    assert_eq!(vec.capacity(), 3);

    in vec.push() { "Berber!" };
    in vec.push() { "There!" };
    in vec.push() { "Hi!" };

    assert_eq!(vec.capacity(), 3);
}
