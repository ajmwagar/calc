extern crate rand;

use rand::prelude::*;

use std::env;
use rand::prelude::*;
use std::f64::consts::PI;
use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main() {

    // Collect args
    let args: Vec<String> = env::args().collect();

    let mut command = "";
    // Check if the length of args has moret than one i.e argument passed
    if *&args.len() as i32 > 1 {
        command = &args[1];
    }

    /* START Argument handling */

    if command == "help" {
        help();
    } else if *&args.len() as i32 > 2 {
        // Create new string
        let mut stringer: String = String::new();
        // Convert the length to a i32
        let len: i32 = *&args.len() as i32;

        // Add up all additional args
        for i in 1..len {
            let index: usize = i as usize;
            if i != 1 {
                stringer.push_str(" ");
            }
            stringer.push_str(&args[index]);
        }
        // Push newline
        // stringer.push_str(&"\n");

        // Run once
        run_once(stringer);
    } else {
        // Clear screen
        clear();
        // Print hint
        hint();
        // Start listner
        run();
    }

    /* END Argument handling */
}

fn calc(calc_arr: Vec<&str>, index: usize, mut val: f64, first: bool) -> f64 {
    if index != 0 || index != calc_arr.len() {
        if first {
            val = handle_equ(calc_arr[0]);
        }
        let next_idx = (index + calc_arr.len() + 1) % calc_arr.len();

        let right_hand = handle_equ(calc_arr[next_idx]);

        if calc_arr[index].contains("/") {
            val /= right_hand;
        } else if calc_arr[index].contains("*") {
            val *= right_hand;
        } else if calc_arr[index].contains("-") {
            val -= right_hand;
        } else if calc_arr[index].contains("+") {
            val += right_hand;
        } else if calc_arr[index].contains("%") {
            val %= right_hand;
        } else if calc_arr[index].contains("^") {
            val = val.powf(right_hand);
        }
        if index == calc_arr.len() - 1 {
            return val;
        } else {
            return calc(calc_arr, index + 1, val, false);
        }
    }
    return calc(calc_arr, index + 1, val, false);
}

fn handle_command (string: &str) {
    match string {
        "quit"  => exit(0),
        "clear" => clear(),
        "help"  => help(),
        _ => parse(string.to_string())
    }
}


fn handle_equ(string: &str) -> f64 {
    match string {
        "+" => return 0.0,
        "-" => return 0.0,
        "^" => return 0.0,
        "*" => return 0.0,
        "/" => return 0.0,
        "%" => return 0.0,
        "pi" => return PI,
        _ => return string.parse::<f64>().unwrap(),
    }
}

fn run() {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    // handle commands
    handle_command(&*s);
    // recurse
    run();
}

fn parse(s: String){

    // New vector of splitted strings for command
    let calc_arr: Vec<&str>;

    // Populate vector
    calc_arr = s.split(" ").collect();

    // println!("{:?}", calc_arr);

    // Handle commands
    if calc_arr.len() > 1 {
        let index = 1;

        let res = calc(calc_arr, index, 0.0, true);

        println!("{}", res);
    } else {
        println!("Please enter a valid expression.");
    }

    run();
}

fn run_once(com: String) {
    let calc_arr: Vec<&str>;

    // Collect on spaces
    calc_arr = com.split(" ").collect();

    // println!("{:?}", calc_arr);

    if calc_arr.len() > 1 {
        // Start at one
        let index = 1;

        // Calculate
        let res = calc(calc_arr, index, 0.0, true);

        println!("{}", res);
    } else {
        println!("Please enter a valid expression.");
    }
}

fn clear() {
    // Clear screen and start
    let output = Command::new("clear")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    // Print out clear terminal
    print!("{}", String::from_utf8_lossy(&output.stdout));
}

fn exit(code: i32){
    clear();
    println!("Goodbye!");
    std::process::exit(code);
}

fn help(){
    let help = "Welcome to calc!\n================\n\nUsage:\n\n 'calc'  opens an interactive session\n 'calc help' opens this help menu\n 'calc <expression>'  solves expression and quits\n\nOperators:\n\n ^  Raise to power\n %  Modulus\n +  Addition\n -  Subtraction\n *  Mutiplication\n /  Division\n\nEquations:\n\n PI  first 14 digits of Π\n\nCommands:\n\n help  opens this help menu\n clear  clear the screen\n quit  close calc";
    // Print help
    println!("{}", help);
}

fn hint(){
    let x: u8 = rand::random();

    let prefix: String = "Hint: ".to_string();

    let hint0 = "Type \"help\" for help..";
    let hint1 = "Type \"clear\" To clear the screen...";
    let hint2 = "Type 1 + 1...";


    print!("{}", prefix);
    match x {
        0 => println!("{}", hint0),
        1 => println!("{}", hint1),
        2 => println!("{}", hint2),
        _ => println!("{}", hint0)
    }

}

