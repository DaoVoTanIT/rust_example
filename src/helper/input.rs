use std::io;

pub fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please try again")
    }
    let input = buffer.trim().to_owned();
    if input.is_empty() { None } else { Some(input) }
}
pub fn get_int() -> Option<i32> {
    println!("Enter age of student");
    let input = match get_input() {
        Some(input) => input,
        None => return None,
    };
    let parse_input: Result<i32, _> = input.parse();
    match parse_input {
        Ok(input) => Some(input),
        Err(_) => None,
    }
}