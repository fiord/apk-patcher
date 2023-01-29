use std::{path::PathBuf, str::FromStr, process::Command, borrow::Borrow};
use execute::Execute;
use clap::Parser;
use regex::Regex;

#[derive(Parser)]
pub struct Compress {
    #[arg(short, long="input", help="The name of folder to be compressed")]
    input: std::path::PathBuf,
    #[arg(short, long="output", help="The name of output apk file")]
    output: Option<std::path::PathBuf>,
    #[arg(long="use-aapt2", help="Upgrades apktool to use experimental aapt2 binary.")]
    use_aapt2: bool,
}

impl Compress {
    // execute shell command.
    // return value: true if command finished successfully
    fn execute_command(mut cmd: Command) -> bool {
        let output = cmd.execute_output().unwrap();
        if let Some(exit_code) = output.status.code() {
            if exit_code != 0 {
                eprintln!("[apk-patcher] finished with error code: {}", exit_code);
                return false;
            }
        } else {
            let program = cmd.get_program();
            eprintln!("[apk-patcher] interrupted: {:?}", program);
            return false;
        }
        return true;
    }

    pub fn execute(&self) {
        let output = match self.output.clone() {
            Some(v) => v,
            None => {
                let input_str = self.input.to_str().unwrap();
                let output_str = format!("{}.packed.apk", input_str);
                PathBuf::from_str(&output_str).unwrap()
            }
        };

        // apktool
        let mut cmd = Command::new("apktool");
        cmd.arg("b");
        cmd.arg("-o");
        cmd.arg(output.to_str().unwrap());
        cmd.arg(self.input.to_str().unwrap());
        if self.use_aapt2 {
            cmd.arg("--use-aapt2");
        }
        if !Compress::execute_command(cmd) {
            return;
        }

        // zipalign
        cmd = Command::new("zipalign");
        cmd.args(["-f", "-v", "4"]);
        let re = Regex::new(r"\.packed\.apk$").unwrap();
        let replaced = re.replace(output.to_str().unwrap(), ".signed.apk");
        let signed_path: &str = replaced.borrow();
        cmd.arg(output.to_str().unwrap());
        cmd.arg(signed_path);
        if !Compress::execute_command(cmd) {
            return;
        }

        // apksigner
        cmd = Command::new("apksigner");
        cmd.args(["sign", "--ks", "apk-patcher.keystore", "--ks-key-alias", "apk-patcher", "--ks-pass", "pass:apk-patcher"]);
        cmd.arg(signed_path);
        if !Compress::execute_command(cmd) {
            return;
        }
        println!("[apk-patcher] finished.");
    }
}