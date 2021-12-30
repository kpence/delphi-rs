use std::process::Command;
use glob::glob;

fn main() {
    for entry in glob("tests/delphi/*.pp").unwrap() {
        if let Ok(path) = entry {   
            Command::new("fpc").arg(&path.as_os_str()).arg("-otests/delphi/")
                .spawn().unwrap();
        }
    }
    println!("cargo:rerun-if-changed=tests/delphi/*.pp");
    println!("cargo:rerun-if-changed=build.rs");
}
