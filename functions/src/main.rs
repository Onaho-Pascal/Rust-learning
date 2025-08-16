const KNOCK_OUT: u32 = 200;

fn main() {

    let val = 30;
    if val > KNOCK_OUT {
        println!("Above maximum allowed!");
    } else {
        println!("Below maximum allowed!");
    }
    let mut is_paused = false;

    // Some game event occurs
    is_paused = !is_paused; // toggle

    let status = if is_paused { "Game is paused" } else { "Game is running" };
    println!("{status}");

    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");


    let pascal = true;
    let name = if pascal {10} else {6};
    println!("the name tag is {name}");

    let mut is_logged_in = true;
    let mut message = if is_logged_in {
        "You have successfully logged in"
    } else {
        "Try again!"
    };
    println!("{message}");
    is_logged_in = false;
    message =  if is_logged_in {
        "You have successfully logged in"
    } else {
        "Try again!"
    };
    println!("{message}");

}





