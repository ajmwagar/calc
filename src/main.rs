extern crate rand;

use std::env;
use rand::prelude::*;
use std::f64::consts::PI;
use std::io::{stdin, stdout, Write};
use std::process::Command;

macro_rules! block {
    ($xs:block) => {
        loop { let _ = $xs; break; }
    };
}

static HELP: &str = "Welcome to calc!\n================\n\nUsage:\n\n 'calc'  opens an interactive session\n 'calc help' opens this help menu\n 'calc <expression>'  solves expression and quits\n\nOperators:\n\n ^  Raise to power\n %  Modulus\n +  Addition\n -  Subtraction\n *  Mutiplication\n /  Division\n\nEquations:\n\n PI  first 14 digits of Π\n ans  previous answer\n\nCommands:\n\n help  opens this help menu\n clear  clear the screen\n quit  close calc";

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
        println!("{}", HELP);
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
        run(0.0);
    }

    /* END Argument handling */
}

fn calc(calc_arr: Vec<&str>, index: usize, mut val: f64, first: bool, ans: f64) -> f64 {

    if index != 0 || index != calc_arr.len() {
        if first {
            val = handle_equ(calc_arr[0], ans);
        }
        let next_idx = (index + calc_arr.len() + 1) % calc_arr.len();

        let right_hand = handle_equ(calc_arr[next_idx], ans);

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
            return calc(calc_arr, index + 1, val, false, ans);
        }
    }
    return calc(calc_arr, index + 1, val, false, ans);
}

fn handle_command (string: &str, ans: f64)  -> String {
    match string {
        "quit"  => return exit(0),
        "clear" => return clear(),
        "help"  => return "\n".to_string(),
        _ => return parse(string.to_string(), ans)
    }
}


fn handle_equ(string: &str, ans: f64) -> f64 {
    match string {
        "+" => return 0.0,
        "-" => return 0.0,
        "^" => return 0.0,
        "*" => return 0.0,
        "/" => return 0.0,
        "%" => return 0.0,
        "pi" => return PI,
        "ans" => return ans,
        _ => return string.parse::<f64>().unwrap(),
    }
}

fn run(mut ans: f64) {
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

    let string_help = HELP.to_string();
    // handle commands
    let res = handle_command(&*s, ans);
    let print: bool;
    match &*res {
        // Break
        "Cleared!" => block!({ break; }),
        // Break
        "Goodbye!" => block!({ break; }),
        // Break
        "Please enter a valid expression." => block!({ break; }),
        "\n" => println!("{}", HELP),
        _ => ans = res.parse::<f64>().unwrap()

    }

    // Print res
    if &res != "Cleared!"{
        println!("{}", res);
    }
    // recurse

    if &*res != "Goodbye" {
        run(ans);
    }
}


fn parse(s: String, mut ans: f64) -> String {

    // New vector of splitted strings for command
    let calc_arr: Vec<&str>;

    // Populate vector
    calc_arr = s.split(" ").collect();

    // println!("{:?}", calc_arr);

    // Handle commands
    if calc_arr.len() > 1 {
        let index = 1;

        let res = calc(calc_arr, index, 0.0, true, ans);

        return res.to_string();
    } else {
        return "Please enter a valid expression.".to_string();
    }
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
        let res = calc(calc_arr, index, 0.0, true, 0.0);

        println!("{}", res);
    } else {
        println!("{}", calc_arr[0]);
    }
}

fn clear() -> String {
    // Clear screen and start
    let output = Command::new("clear")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    // Print out clear terminal
    println!("{}", String::from_utf8_lossy(&output.stdout));

    return "Cleared!".to_string();
}

fn exit(code: i32) -> String {
    clear();
    std::process::exit(code);
    return "Goodbye!".to_string();
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

