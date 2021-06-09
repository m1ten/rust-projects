/* 
zlib License

(C) 2021 miten

This software is provided 'as-is', without any express or implied
warranty.  In no event will the authors be held liable for any damages
arising from the use of this software.

Permission is granted to anyone to use this software for any purpose,
including commercial applications, and to alter it and redistribute it
freely, subject to the following restrictions:

1. The origin of this software must not be misrepresented; you must not
   claim that you wrote the original software. If you use this software
   in a product, an acknowledgment in the product documentation would be
   appreciated but is not required.
2. Altered source versions must be plainly marked as such, and must not be
   misrepresented as being the original software.
3. This notice may not be removed or altered from any source distribution.
*/

use std::io::Write;

fn main() {
    println!("RustyCalc!");
    println!("\nEnter equation [x + y] - Enter \"exit\" to exit.\n");

    calculator();
}

fn calculator() {
    loop {
        print!(">> ");
        let input = readln();
        let equation: Vec<&str> = input.split(' ').collect();

        if equation.contains(&"exit") {
            return;
        }

        if equation.len() != 3 {
            println!("NaN\n");
            calculator();
        }

        let num1: f64 = match equation[0].parse() {
            Ok(i) => i,
            Err(e) => {
                print!("{} ", e);
                0.0
            }
        };

        let op: char = match equation[1].parse() {
            Ok(i) => i,
            Err(e) => {
                print!("{} ", e);
                'e'
            }
        };

        let num2: f64 = match equation[2].parse() {
            Ok(i) => i,
            Err(e) => {
                print!("{} ", e);
                0.0
            }
        };

        match op {
            '+' => println!("> {}\n", num1 + num2),
            '-' => println!("> {}\n", num1 - num2),
            '*' => println!("> {}\n", num1 * num2),
            '/' => println!("> {}\n", num1 / num2),
            _ => println!("> {}\n", 0.0 / 0.0),
        }
    }
}

fn readln() -> String {
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.replace("\n", ""),
        Err(e) => println!("{}", e.to_string()),
    }
}