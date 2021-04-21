use std::process::Command;

fn main() {
    // Get environment variable
    // https://programming-idioms.org/idiom/205/get-an-environment-variable/3700/rust
    let _foo = match std::env::var("FOO") {
        Ok(val) => val,
        Err(_e) => "none".to_string(),
    };

    // Run command
    let translate_command = "~/CTranslate2/build/cli/translate --model ~/argos-translate/en_zh/model";

    // let translate_command = "ls";

    let mut command = Command::new(translate_command);
    command.status().expect("process failed to execute");

    // Print line
    println!("Hello World");
}
