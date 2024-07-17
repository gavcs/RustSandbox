use adder::add_two;
use adder::Rectangle;

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}

#[test]
fn rect_test() {
    let smaller = common::setup();
    let bigger = Rectangle {
        width: 7,
        height: 8,
    };
    assert!(bigger.can_hold(&smaller));
}