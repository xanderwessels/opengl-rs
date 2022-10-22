use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Please input a number.");

    let mut num1 = String::new();
    
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");

    println!("Please input a second number.");

    let mut num2 = String::new();

    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");

    match num2.cmp(&num1) {
        Ordering::Less => println!("Your first number is bigger!"),
        Ordering::Greater => println!("Your second number is bigger!"),
        Ordering::Equal => println!("Those numbers are the same!"),
    }
}
