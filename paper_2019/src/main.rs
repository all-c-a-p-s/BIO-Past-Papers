//problem: given any number, output smallest palindromic number that is higher than the input

use std::io;

fn is_palindrome(num: String) -> bool {
    let nums: Vec<char> = num.chars().collect();  
    let reversed: Vec<char> = nums.clone().into_iter().rev().collect();  
    nums == reversed         
}

fn next_palindrome(num: i128) -> i128 {
    let mut n = num + 1;
    loop {
        if is_palindrome(n.to_string()) {
            print!("{}", n);
            return n;
        }
        n += 1;
    }
}

fn main() {
    println!("Enter the number: ");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: i128 = num
        .trim()
        .parse()
        .expect("Failed to convert to i128");

    next_palindrome(num); 
}
