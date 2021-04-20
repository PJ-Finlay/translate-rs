use std::process::Command;

fn main() {
    println!("Hello World!!");
    let mut command = Command::new("ls");
    command.status().expect("process failed to execute");
}
