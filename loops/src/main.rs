fn main () {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut var_ = 0;
    'outer_: loop {
        println!("{var_}");
        let mut remaining = 10;
        loop {
            println!("{remaining}");
            if remaining == 9 {
                break;
            }
            if var_ == 2 {
                break 'outer_;
            } else {
                remaining -= 1;
            }
            
        };
        var_ += 1;
    };
    println!("{var_}");

    let test_ = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    'rows: for row in 0..test_.len() {
        for col in 0..test_[row].len() {
            println!("Row = {row}, Column = {col}, Value = {}", test_[row][col]);
            if row == 2 && col == 2 {
                break 'rows;
            }
        }
    };
    println!("Search finished!");


}