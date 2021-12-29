#![feature(asm)]
//use std::{error, result};
use delphi::*;
use std::fs::File;
use std::process::Command;
use std::io::prelude::*;
use std::iter;

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
