use std::io::{self, Write};

#[inline]
fn clean(input: String) -> String {
    input.trim().to_string()
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
            Ok(_) => {
                input = clean(input);

                // Close the program if exit() is entered, compile otherwise
                if input.eq("exit()") {
                    break;
                } else {
                    pillow::compile(input)
                }
            }
            Err(error) => eprintln!("ERROR: Failed to read input\nDETAILS: {}", error),
        }
    }
}
