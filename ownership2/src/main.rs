fn main() {
    let name = String::from("Pascal");
    let new_name = name; //ownership has been transfered t "new_name".

    //If you want "new_name" to have the same value as "name", without it being pointed to but created newly,
    // then let new_name = name.clone(); 

    println!("{}", new_name);

    let s = String::from ("Rust");

    take_ownership(s);
    // println!("{}", s); will throw back an error cos Rust has been technically moved from s to "take_ownership" function
    let a: i32 = 34;
    let x: i32 = a;

    println!("{} {}", a, x);

}


fn take_ownership(str_val: String) {
    println!("{}", str_val);
}