#[macro_export]
macro_rules! fpc_first_function_address {
    ($DELPHI_SRC_CODE:literal) => {{
        use std::iter;

        let delphi_base_address = fpc!($DELPHI_SRC_CODE);
        delphi_base_address.get_unchecked(delphi_base_address.iter().position(|&x| x == 0x55).unwrap())
    }};
}

#[macro_export]
macro_rules! fpc {
    ($DELPHI_SRC_CODE:literal) => {{
        use std::fs::File;
        use std::process::Command;
        use std::io::prelude::*;

        let delphi_src_dir: &str = "/tmp/delphi.pp";
        File::create(delphi_src_dir).unwrap()
            .write_all($DELPHI_SRC_CODE).unwrap();
        Command::new("fpc").args(&["-s",delphi_src_dir])
            .current_dir("/tmp")
            .status().unwrap();
        include_bytes!("/tmp/delphi.o")
    }};
}
