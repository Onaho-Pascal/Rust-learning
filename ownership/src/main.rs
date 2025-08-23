// Moving ownership
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // ownership moves

    // println!("{}", s1); ❌ invalid, s1 no longer owns the string
    println!("{}", s2);

// Borrowing Ownership

    fn length(s: &String) -> usize {
        s.len()
    }
    let s1 = String::from("hello");
    let len = length(&s1); // borrow, don’t take ownership
    println!("'{}' is {} chars long", s1, len);

    let book = String::from("The Rust Programming Language");

    // We borrow 'book' with a reference (&book)
    let length = calculate_length(&book);

    println!("'{}' has {} characters.", book, length);
}

// The function borrows the string instead of taking ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}


