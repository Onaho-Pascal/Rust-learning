use std::io;

fn main() {
    let arr: [u32; 4] = [1, 2, 3, 4];

    let mut index = String::new();

    println!("To see an element in this array, you have to input an index from 0 to 3:");
    io::stdin()
        .read_line(&mut index)
        .expect("This failed to get an input");


    let index: usize = index.trim().parse().expect("This was not a number");

    let element = arr[index];

    println!("The element at the index is {}", element);



}