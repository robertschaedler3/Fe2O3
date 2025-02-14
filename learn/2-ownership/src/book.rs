#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

pub struct Book{}

pub fn loan(_book: &Book, borrower: &str) {
    println!("{} borrows the book", borrower);
}

pub fn remove_book(_book: Book) {
    println!("Book is removed");
}

pub fn sell(_book: Book) {
    println!("Book is sold");
}

pub fn edit(_book: &mut Book, editor: &str) {
    println!("Book is edited by {}", editor);
}