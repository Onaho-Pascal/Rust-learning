#[derive(Debug, Clone)]
struct User {
    name: String,
    email: String,
}

fn main() {
    let user1 = User {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
    };

    // Transfer ownership of user1 into system
    register_user(user1);


    // If we still need user data afterwards, we can clone
    let user2 = User {
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
    };
    let user3 = user2.clone(); // deep copy
    register_user(user2);
    println!("We still have {:?}", user3);

    // Borrowing: check user’s email length without taking ownership
    let email_length = check_email_length(&user3);
    println!("{}'s email length is {}", user3.name, email_length);
}

fn register_user(u: User) {
    println!("Registered user: {:?}", u);
} // u goes out of scope here → memory freed safely

fn check_email_length(u: &User) -> usize {
    u.email.len()

    let book = String::from("The Rust Programming Language");

    // We borrow 'book' with a reference (&book)
    let length = calculate_length(&book);

    println!("'{}' has {} characters.", book, length);

// The function borrows the string instead of taking ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}



}
