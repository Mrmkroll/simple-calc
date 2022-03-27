use std::io;

use question::{Question, Answer};

pub fn cls() {
    print!("{esc}c", esc = 27 as char);
}

pub fn pause() {
    Question::new("\nPress 'Enter' to continue . . .")
        .default(Answer::YES)
        .ask();
}

pub fn input() -> String {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input...");
        let input = input.trim().to_string();

        let result = check(&input);
        if result == 10 {
            match input.parse::<usize>() {
                Ok(num) => num,
                Err(_) => {
                    println!("\x1b[91mPlease input a number!\x1b[0m");
                    continue;
                }
            };
        }
        return input
    }
}

fn check(input: &str) -> usize {
    match input.find("0x") {
        None => {
            let result = 10;
            return result
        },
        _ => {
            let result = 16;
            return result
        }
    };
}

fn swap(input: &str) -> usize {
    let result = check(input);
    match result {
        10 => {
            let num = input.parse::<usize>().unwrap();
            return num
        },
        _ => {
            let num = input.trim_start_matches("0x");
            if num.chars().count() == 0 {
                let num = 0;
                return num
            }
            let num = usize::from_str_radix(num, 16).unwrap();
            return num
        }
    }
}

pub fn add() {
    cls();
    println!("-- Add --\n");
    println!("Please input 1st number");
    let input_1 = input();
    println!("\nPlease input 2nd number");
    let input_2 = input();

    let num_1 = swap(&input_1);
    let num_2 = swap(&input_2);
    let ans = num_1 + num_2;
    println!("\n-- Answer --\nHex: 0x{:X}\nDec: {}", ans, ans);
    pause();
}

pub fn sub() {
    cls();
    println!("-- Subtract --\n");
    println!("Please input 1st number");
    let input_1 = input();
    println!("\nPlease input 2nd number");
    let input_2 = input();

    let num_1 = swap(&input_1);
    let num_2 = swap(&input_2);
    if num_1 >= num_2 {
        let ans = num_1 - num_2;
        println!("\n-- Answer --\nHex: 0x{:X}\nDec: {}", ans, ans);
    } else {
        let ans = num_2 - num_1;
        println!("\n-- Answer --\nHex: -0x{:X}\nDec: -{}", ans, ans);
    }
    pause();
}

pub fn mul() {
    cls();
    println!("-- Multiply --\n");
    println!("Please input 1st number");
    let input_1 = input();
    println!("\nPlease input 2nd number");
    let input_2 = input();

    let num_1 = swap(&input_1);
    let num_2 = swap(&input_2);
    let ans = num_1 * num_2;
    println!("\n-- Answer --\nHex: 0x{:X}\nDec: {}", ans, ans);
    pause();
}

pub fn div() {
    cls();
    println!("-- Divide --\n");
    println!("Please input 1st number");
    let input_1 = input();
    println!("\nPlease input 2nd number");
    let input_2 = input();

    let num_1 = swap(&input_1);
    let num_2 = swap(&input_2);
    if num_2 == 0 {
        println!("\nAnswer: Null");
    } else if num_1 == 0 || num_1 == num_2 {
        let ans = num_1 / num_2;
        println!("\n-- Answer --\nHex: 0x{:X}\nDec: {}", ans, ans);
    } else {
        let ans = num_1 / num_2;
        let rem = num_1 % num_2;
        println!("\n-- Answer --\nHex: 0x{:X}...0x{:X}\nDec: {}...{}", ans, rem, ans, rem);
    }
    pause();
}