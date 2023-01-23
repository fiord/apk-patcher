use std::{process::Command, path::PathBuf, str::FromStr};
use execute::Execute; 
use clap::Parser;
use regex::Regex;

#[derive(Parser)]
pub struct Decompress {
    #[arg(short, long, help="The name of apk file to be decompressed.")]
    input: std::path::PathBuf,
    #[arg(short, long, value_name="FILE", help="The name of folder that gets written. Default is same as input.")]
    output: Option<std::path::PathBuf>,
    #[arg(short, long, help="Force delete destination directory.")]
    force: bool,
    #[arg(long="only-main-classes", help="decompress only main classes.")]
    only_main_classes: bool,
    #[arg(short='r', long="no-res", help="Do not decode resources.")]
    no_resources: bool,
    #[arg(short='s', long="no-src", help="Do not decode sources.")]
    no_sources: bool,
}

impl Decompress {
    pub fn execute(&self) {
        let output = match self.output.clone() {
            Some(v) => v,
            None => {
                let re = Regex::new(r"\.apk$").unwrap();
                let input_str = self.input.to_str().unwrap();
                let output_str = re.replace_all(input_str, "");
                PathBuf::from_str(output_str.as_ref()).unwrap()
            }
        };

        let mut cmd = Command::new("apktool");        
        cmd.arg("d");
        cmd.arg("-o");
        cmd.arg(output.to_str().unwrap());
        cmd.arg(self.input.to_str().unwrap());
        if self.force {
            cmd.arg("-f");
        }
        if self.only_main_classes {
            cmd.arg("--only-main-classes");
        }
        if self.no_resources {
            cmd.arg("-r");
        }
        if self.no_sources {
            cmd.arg("-s");
        }

        let output = cmd.execute_output().unwrap();
        if let Some(exit_code) = output.status.code() {
            if exit_code == 0 {
                println!("[apk-patcher] finished.");
            } else {
                eprintln!("[apk-patcher] finished with error code: {}", exit_code);
            }
        } else {
            eprintln!("[apk-patcher] interrupted.");
        }
    }
}