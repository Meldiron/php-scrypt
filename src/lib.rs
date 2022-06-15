#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;
use scrypt::{scrypt, Params};
use std::str;

#[php_function]
pub fn rust_scrypt(password: &str, salt: &str, log_n: u8, r: u32, p: u32, length: usize) -> String {
    let mut result = vec![0u8; length];

    let params = Params::new(log_n, r, p).unwrap();
    scrypt(password.as_bytes(), salt.as_bytes(), &params, &mut result).unwrap();
    let hash = str::from_utf8(&result).unwrap();
    
    format!("{}", hash)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}