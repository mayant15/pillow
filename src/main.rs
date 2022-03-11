use std::io::{self, Write};

#[inline]
fn clean(input: &mut str) -> &str {
    input.trim()
}

fn process_input(input: &mut str) {
    let cleaned = clean(input);
    println!("cleaned input: {}", cleaned);

    // Close the program if exit() is entered, compile otherwise
    if cleaned.eq("exit()") {
        panic!("Exiting the program...");
    } else {
        match pillow::compile(cleaned) {
            Err(error) => {
                eprintln!("ERROR: Failed to compile input\nDETAILS: {}", error)
            }
            _ => (),
        }
    }
}

fn main() {
    println!("Pillow REPL, 2021, v0.1.0");
    println!("THIS IS A WORK IN PROGRESS");
    println!("Enter 'exit()' to exit");

    loop {
        print!(">>> ");

        // Flush all remaining print statements before asking for more user input
        match io::stdout().flush() {
            Err(error) => eprintln!("ERROR: Cannot flush stdout\nDETAILS: {}", error),
            _ => (),
        }

        // Read the next line and take user input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => process_input(input.as_mut_str()),
            Err(error) => eprintln!("ERROR: Failed to read input\nDETAILS: {}", error),
        }
    }
}
