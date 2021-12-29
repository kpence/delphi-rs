#![feature(asm)]

#[macro_use]
mod macros;

//use std::{error, result};

//type TResult<T> = result::Result<T, TError>;
//type TError = Box<dyn error::Error>;
//type PtrSize = usize;

//struct CallableDelphiFn<F> {
//    foreign_fn: usize,
//}
//
//trait TrampolineFn<Args, Output> {
//    unsafe extern "C" fn trampoline();
//}

//cmake_minimum_required(VERSION 3.12)
//project(MyCoolProject LANGUAGES CXX)
//
//find_package(Corrosion REQUIRED)
//
//corrosion_import_crate(MANIFEST_PATH rust-lib/Cargo.toml)
//
//add_executable(cpp-exe main.cpp)
//target_link_libraries(cpp-exe PUBLIC rust-lib)

//impl<F> TrampolineFn<F> for CallableDelphiFn<F> {
//    #[naked]
//    unsafe extern "C" fn trampoline() {
//        let foreign_fn: usize = self::foreign_fn as usize;
//        unsafe {
//            asm!(
//                "mov byte ptr al, [esp+4]",
//                "jmp {}", sym foreign_fn,
//                options(noreturn)
//            )
//        }
//    }
//}

pub fn test() -> u32 {
    0
}
