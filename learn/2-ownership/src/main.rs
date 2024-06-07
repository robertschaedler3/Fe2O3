use book::*;

mod book;

fn ownership() {

    {
        let s = "hello";

        println!("{}", s);
    }
    println!("{}", s);

}

fn borrow() {
    let i_robot = Book{};
    loan(&i_robot, "foo"); // borrow
    loan(&i_robot, "bar"); // borrow
    
    remove_book(i_robot); // give up ownership
    
    loan(&i_robot, "baz");
}

fn mutable_borrow() {
    let mut i_robot = Book{};

    edit(&mut i_robot, "foo"); // borrow mutably
    edit(&mut i_robot, "bar");

    sell(i_robot);
    edit(&mut i_robot, "baz");
}

fn main() {
    ownership();
    borrow();
    mutable_borrow();
}
