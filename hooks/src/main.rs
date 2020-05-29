extern crate detour;
extern crate rand;

use detour::static_detour;
use rand::Rng;
use std::io::stdin;

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

fn initialize_hook() {
    unsafe {
        Random_number_detour
            .initialize(random_number, modified_random_number)
            .unwrap();
    }
}

fn hook_function(hooked: &mut bool, initialized: &mut bool) {
    match *hooked {
        true => {
            println!("Already hooked!");
            println!("");
        }
        false => {
            if *initialized == false {
                initialize_hook();
                *initialized = true;
            }
            unsafe { Random_number_detour.enable().unwrap() };
            *hooked = true;
        }
    }
}

fn hook_function_disable(hooked: &mut bool) {
    match *hooked {
        false => {
            print!("Hook is already disabled!");
            println!("");
        }
        true => {
            unsafe { Random_number_detour.disable().unwrap() };
            *hooked = false;
        }
    }
}

fn main() {
    let mut initialized: bool = false;
    let mut hooked: bool = false;

    loop {
        println!(" Press 1 to generate new number, 2 to hook function, 3 to disable hook.");
        let mut input_string = String::new();
        stdin().read_line(&mut input_string).unwrap();
        input_string = input_string.trim().to_string();
        if input_string.chars().all(char::is_numeric) == false {
            println!("Please enter a number between 1-3.");
            continue;
        }
        let input: u8 = input_string.parse().unwrap();

        match input {
            1 => print_random_number(),
            2 => hook_function(&mut hooked, &mut initialized),
            3 => hook_function_disable(&mut hooked),
            _ => println!("Please enter a number between 1-3."),
        }
        // pause();
    }
}
