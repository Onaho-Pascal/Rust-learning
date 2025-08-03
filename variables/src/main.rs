fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    let x = 3;
    let x = x + 7; {
        let x = x * 3;
        println!("The value of x in this inner scope is: {x}");
    }
    println!("The value of x is: {x}")
}
