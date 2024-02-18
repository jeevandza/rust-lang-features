// use std::env;

fn main() {
    println!("Hello, world!");
    variable();
    mutable();
    scopes();
    modify();
    remove_variable();
    let book: Book = Book::new(
        "The Rust Programming Language",
        "Steve Kabuki & Carol Nichols",
        600,
    );
    book.description();
}

fn variable() {
    let x: i32 = 5;
    println!("{}", x)
}

fn mutable() {
    let mut x: i32 = 9;
    x += 2;
    println!("{}", x)
}

fn scopes() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y)
    }
    println!("The value of x is {} and value of y is {}", x, y)
}

fn modify() {
    let x: i32 = 5;
    {
        let x: i32 = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 5);
    let x: i32 = 42;
    println!("{}", x);
}

fn remove_variable() {
    let mut x: i32 = 1;
    x += 7;
    x += 3;

    // Shadowing
    let y: &str = "I can also be bound to text!";

    println!("the value of x {} and value of y {}", x, y);
}
struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    // Define the associated function `new`
    fn new(title: &str, author: &str, pages: u32) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
            pages,
        }
    }

    // Define the instance method `description`
    fn description(&self) {
        println!("'{}' by {}, {} pages", self.title, self.author, self.pages);
    }
}
