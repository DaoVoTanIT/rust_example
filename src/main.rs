mod helper;
mod student;
mod product;
mod invoice;
use helper::input::get_input;

fn main() {
    loop {
        println!("\n=== Rust Exercises Menu ===");
        println!("1. Student Manager");
        println!("2. Product Manager");
        println!("3. Invoice Manager");
        println!("4. Exercise 3: Ownership & Borrowing");
        println!("5. Exercise 4: Structs & Methods");
        println!("0. Exit");
        println!("Enter your choice:");

        let choice = get_input().expect("Please input your option");
        match choice.as_str() {
            "1" => student::student_main(),
            "2" => product::main_product(),
            "3" => invoice::main_invoice(),
            "0" => break,
            _ => println!("Invalid choice!"),
        }
    }
}
