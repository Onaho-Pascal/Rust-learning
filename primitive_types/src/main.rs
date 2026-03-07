fn main() {
    let a: i32 = 46;
    let b: u32 = 99;
    let pi: f64 = 3.14;
    let is_sunny: bool = true;
    let vowel_letter: char = 'u';
    println!("The signed integer is: {}", a);
    println!("The unsigned integer is: {}", b);
    println!("The value of pi is: {}", pi);
    println!("Is it actually sunny? {}", is_sunny);
    println!("The fifth vowel letter is: {}", vowel_letter);

    
    // Arrays: Homogenous of fixed size
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("The numbers array is: {:?}", numbers); // :? is a debug format that prints out as it is internally.

    // let mix = [1,4,"apple", true];
    // println!("Does this array mix? {}", mix);

    let mix: [&str; 4] = ["Apple", "orange", "Pineapple", "Mango"];

    println!("These are the fruiots in the array: {:?}", mix); // :? is a debug printing format used in tuples, a
    println!("These are the fruiots in the array: {}", mix[0]);
    println!("These are the fruiots in the array: {}", mix[1]);
    println!("These are the fruiots in the array: {}", mix[2]);
    println!("These are the fruiots in the array: {}", mix[3]);


    // Tuples: Heterogenous of fixed size

    let games = ("Elden ring", 2018, true);
    let states: (&str, i32, bool) = ("Oyo State", 23, false);

    println!("The tuple for games is: {:?}", games); // :? is a debug printing format used for arrays, tuples
    println!("The tuple for states is: {:?}", states);



} 

