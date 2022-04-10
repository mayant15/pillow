use std::io::{self, Write};

fn abort() {
    panic!("Exit the program...");
}

#[inline]
fn clean(input: &mut str) -> &str {
    input.trim()
}

fn process_input(input: &mut str) {
    let cleaned = clean(input);

    // Close the program if exit() is entered, compile otherwise
    if cleaned.eq("exit()") {
        abort();
    } else if let Err(error) = pillow::compile(cleaned) {
        eprintln!("ERROR: Failed to compile input\nDETAILS: {}", error);
    }
}

fn repl() -> ! {
    println!("Pillow REPL, 2021, v0.1.0");
    println!("THIS IS A WORK IN PROGRESS");
    println!("Enter 'exit()' to exit");

    loop {
        print!(">>> ");

        // Flush all remaining print statements before asking for more user input
        if let Err(error) = io::stdout().flush() {
            eprintln!("ERROR: Cannot flush stdout\nDETAILS: {}", error);
            abort();
        }

        // Read the next line and take user input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => process_input(input.as_mut_str()),
            Err(error) => eprintln!("ERROR: Failed to read input\nDETAILS: {}", error),
        }
    }
}

fn process_file(path: String) {
    let contents = std::fs::read_to_string(path).unwrap_or_default();
    println!("input file: {}", contents);
}

fn main() {
    let mut args = std::env::args();
    if args.len() > 1 {
        // File provided, don't run the repl
        process_file(args.nth(1).unwrap());
    } else {
        // No args, run the REPL
        repl();
    }
}
