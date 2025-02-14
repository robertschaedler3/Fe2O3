mod book;

use book::{loan, remove_book, Book};

fn main() {
    let i_robot = Book{};
    loan(&i_robot, "foo"); // borrow
    loan(&i_robot, "bar"); // borrow

    remove_book(i_robot); // give up ownership

    // loan(&i_robot, "baz"); // error: value borrowed here after move
}