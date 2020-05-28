extern crate detour;
extern crate rand;

use detour::static_detour;
use rand::Rng;
use std::io::{stdin, stdout, Read, Write};

// fn pause() {
//     let mut stdout = stdout();
//     stdout.write(b"Press Enter to continue...").unwrap();
//     stdout.flush().unwrap();
//     stdin().read(&mut [0]).unwrap();
// }

static_detour! {
    static Random_number_detour: fn() -> i8;
}

fn random_number() -> i8 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn print_random_number() {
    let num = random_number();
    println!("Your random number is {}", num);
}

fn modified_random_number() -> i8 {
    69
}

fn hook_function() {
    unsafe {
        Random_number_detour.initialize(random_number, modified_random_number).unwrap();
        Random_number_detour.enable().unwrap();
    };
}

fn hook_function_disable() {
    unsafe { Random_number_detour.disable().unwrap() };
}

fn main() {
    loop {
        println!(" Press 1 to generate new number, 2 to hook function, 3 to disable hook.");
        let mut input_string = String::new();
        stdin().read_line(&mut input_string).unwrap();
        let input = input_string.trim().parse().expect("Please enter a number between 1-3.");

        match input {
            1 => print_random_number(),
            2 => hook_function(),
            3 => hook_function_disable(),
            _ => println!("Please enter a number between 1-3."),
        }
        // pause();
    }
}