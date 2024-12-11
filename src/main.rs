mod book;

use book::Book;
fn main() {
    let book1 = Book::new(
        "The Rust Programming Language",
        "Steve Klabnik and Carol Nichols",
        2018,
    );
    let book2 = Book::new("Programming Rust", "Jim Blandy and Jason Orendorff", 2017);
    book1.display();
    book2.display();
}
