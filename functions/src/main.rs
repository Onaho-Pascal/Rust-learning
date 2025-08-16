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

}





