#[macro_export]
macro_rules! fpc_first_function_address {
    ($DELPHI_SRC_CODE:literal) => {{
        use std::iter;

        let delphi_base_address = fpc!($DELPHI_SRC_CODE);
        delphi_base_address.as_ptr().add(delphi_base_address.iter().position(|&x| x == 0x55).unwrap())
    }};
}

#[macro_export]
macro_rules! fpc {
    ($DELPHI_SRC_CODE:literal) => {{
        use std::fs::File;
        use std::process::Command;
        use std::io::prelude::*;

        File::create("/tmp/delphi.pp").unwrap()
            .write_all($DELPHI_SRC_CODE).unwrap();
        Command::new("fpc").args(&["/tmp/delphi.pp"])
            .current_dir("/tmp")
            .spawn().unwrap();
        include_bytes!("/tmp/delphi.o")
    }};
}
