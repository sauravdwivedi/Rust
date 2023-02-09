////
// Write a method that checks if there is at least one pair of
// numbers which sum equals target. arr=[1, 3, 4] and target=5
// result is true because the pair (1,4) sums to five.
////
use std::io;

fn two_sum(list_of_numbs: &Vec<i32>, target_num: &i32) -> String {
    let mut result: bool = false;
    let message: String;
    for p in list_of_numbs {
        for q in list_of_numbs {
            if p != q && p + q == *target_num {
                result = true;
            }
        }
    }
    if result == true {
        message = format!("At least one pair has sum equal to {target_num}!").to_string();
    } else {
        message = format!("No pair has sum equal to {target_num}!").to_string();
    }
    return message;
}

fn main() {
    println!("Enter list of integers (space separated e.g. '1 2 3'): ");
    let mut input_line: String = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    for number in input_line.trim().split(' ').collect::<Vec<&str>>().iter() {
        if let Err(_err) = number.parse::<i32>() {
            println!("Can only accept valid numbers. Please try again!");
            return main();
        }
    }
    let list_of_numbs: Vec<i32> = input_line
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|n| n.parse().expect("Input not an integer"))
        .collect();
    println!("Enter target integer: ");
    let mut input_line: String = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    if let Err(_err) = input_line.trim().parse::<i32>() {
        println!("Can only accept valid numbers. Please try again!");
        return main();
    }
    let target_num: i32 = input_line.trim().parse().expect("Input not an integer");
    let result: String = two_sum(&list_of_numbs, &target_num);
    println!("{}", &result);
}
