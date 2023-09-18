//problem: given any integer, find its zackendorf representation (find a series of fibonacci numbers which 
// sum to this integer)

use std::io;

fn get_highest_fibonacci(target: i32) -> i32 {
    let mut previous_fib: i32 = 0;
    let mut current_fib: i32 = 1;

    while current_fib <= target {
        let temp: i32 = current_fib;
        current_fib += previous_fib;
        previous_fib = temp;
    }
    previous_fib
}

fn zackendorf_representation(num: i32) -> Vec<i32>{
    let mut zackendorf_representation: Vec<i32> = vec![];
    let mut x: i32 = num;

    while x > 0 {
        let highest_fib: i32 = get_highest_fibonacci(x);
        zackendorf_representation.push(highest_fib);
        x -= highest_fib
    }

    for num in zackendorf_representation.iter() {
        print!("{}, ", num);
    }
    zackendorf_representation
}

fn main() {
    println!("Enter number to represent");

    let mut target_num = String::new();
    io::stdin().read_line(&mut target_num)
               .expect("Failed to read line");

    let target_num: i32 = match target_num.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("failed to convert to int"),
    };    

    zackendorf_representation(target_num);
}
