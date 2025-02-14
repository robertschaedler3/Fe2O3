mod book;

use book::{edit, sell, Book};

fn main() {
    let mut i_robot = Book {};

    edit(&mut i_robot, "foo"); // borrow mutably
    edit(&mut i_robot, "bar");

    sell(i_robot); // give up ownership

    // edit(&mut i_robot, "baz"); // error: value borrowed here after move
}
