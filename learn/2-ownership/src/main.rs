#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use book::*;

mod book;

#[cfg(feature="ownership")]
fn ownership() {
    {
        let s = "hello";
        println!("{}", s);
    }

    println!("{}", s);
}

#[cfg(feature="borrow")]
fn borrow() {
    let i_robot = Book{};
    loan(&i_robot, "foo"); // borrow
    loan(&i_robot, "bar"); // borrow

    remove_book(i_robot); // give up ownership

    loan(&i_robot, "baz");
}

#[cfg(feature="mu_borrow")]
fn mutable_borrow() {
    let mut i_robot = Book{};

    edit(&mut i_robot, "foo"); // borrow mutably
    edit(&mut i_robot, "bar");

    sell(i_robot);
    edit(&mut i_robot, "baz");
}

fn main() {
    #[cfg(feature="ownership")]
    ownership();

    #[cfg(feature="borrow")]
    borrow();

    #[cfg(feature="mut_borrow")]
    mutable_borrow();
}
