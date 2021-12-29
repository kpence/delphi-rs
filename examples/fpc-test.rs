#![feature(asm)]
//use std::{error, result};
use delphi::*;
use std::fs::File;
use std::process::Command;
use std::io::prelude::*;

macro_rules! fpc {
    ($DELPHI_SRC_CODE:literal) => {{
        let delphi_src_dir: &str = "/tmp/delphi.pp";
        File::create(delphi_src_dir).unwrap()
            .write_all($DELPHI_SRC_CODE).unwrap();
        Command::new("fpc").args(&["-s",delphi_src_dir])
            .current_dir("/tmp")
            .status().unwrap();
        include_bytes!("/tmp/delphi.o")
    }};
}

fn main() -> () {
    test();
    let delphi_bin = fpc!(
        br###"
        unit delphi;

        interface

            implementation

            function WF: Integer;
        begin
            While true do
            begin
            end;
        WF:= 1
            end;

        end.
        "###
    );
    println!("test");
    for i in 0..500 {
        println!("test: {:x}", delphi_bin[i]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_test() {
        assert_eq!(0, test());
    }
}
