use std::{time::Duration, thread::sleep};

mod lib;

fn main() {
    loop {
        lib::cls();
        println!("-- Simple Calculator --\n");
        println!("1. Add     2. Subtract     3. Multiply     4. Divide");
        println!("\n0. Exit");
        let num = lib::input();
    
        let num = match num.parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("\x1b[91mError: Invalid input\x1b[0m");
                sleep(Duration::from_millis(1400));
                continue;
            }
        };
    
        match num {
            1 => lib::add(),
            2 => lib::sub(),
            3 => lib::mul(),
            4 => lib::div(),
            0 => {
                lib::cls();
                println!("See you!");
                sleep(Duration::from_millis(2000));
                break
            },
            _ => {
                println!("\x1b[91mError: Invalid input\x1b[0m");
                sleep(Duration::from_millis(1400));
                continue
            },
        }
    }
}
