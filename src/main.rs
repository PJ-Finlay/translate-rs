use std::process::Command;

fn main() {
    println!("Hello World!!");

    // Run command
    let mut command = Command::new("ls");
    command.status().expect("process failed to execute");

    // Get environment variable
    // https://programming-idioms.org/idiom/205/get-an-environment-variable/3700/rust
    let _foo = match std::env::var("FOO") {
        Ok(val) => val,
        Err(_e) => "none".to_string(),
    };
}
