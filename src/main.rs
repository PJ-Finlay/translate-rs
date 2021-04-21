use std::process::Command;

fn main() {
    // Run command
    // ~/CTranslate2/build/cli/translate --model ~/argos-translate/en_zh/model
    let ctranslate = "/home/argosopentech/CTranslate2/build/cli/translate";
    let mut command = Command::new(ctranslate);
    command.arg("--model");
    command.arg("/home/argosopentech/argos-translate/en_zh/model");
    command.status().expect("process failed to execute");
}
