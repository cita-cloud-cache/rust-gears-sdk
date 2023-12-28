/*
  FISCO BCOS/rust-SDK is a rust client for FISCO BCOS2.0 (https://github.com/FISCO-BCOS/)
  FISCO BCOS/rust-SDK is free software: you can redistribute it and/or modify it under the
  terms of the MIT License as published by the Free Software Foundation. This project is
  distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
  the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
  @author: kentzhang
  @date: 2021-07
*/
#![allow(
    clippy::unreadable_literal,
    clippy::upper_case_acronyms,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    overflowing_literals,
    unused_variables,
    unused_assignments
)]
use failure::{AsFail, Fail};

#[macro_export]
macro_rules! kisserr {
            ($x:expr,$($arg:tt)*) => {
                Err(KissError::new(
                    ($x),
                    -1,
                    format!($($arg)*).as_str()
                ))
            };
}

#[macro_export]
macro_rules! kisserrcode {
            ($x:expr,$code:expr,$($arg:tt)*) => {
                Err(KissError::new(
                    ($x),
                    ($code),
                    format!($($arg)*).as_str()
                ))
            };
}

//Kiss: Keep It Simple & Stupid
#[derive(Fail, Clone, Debug, Eq, PartialEq, Default)]
pub enum KissErrKind {
    #[fail(display = "error")]
    #[default]
    Error,
    #[fail(display = "Argument is invalid")]
    EArgument,
    #[fail(display = "Data format error")]
    EFormat,
    #[fail(display = "net work error")]
    ENetwork,
    #[fail(display = "timeout")]
    ETimeout,
    #[fail(display = "sign fail")]
    ESign,
    #[fail(display = "try again")]
    EAgain,
    #[fail(display = "file not exist")]
    EFileMiss,
    #[fail(display = "file open")]
    EFileOpen,
    #[fail(display = "file write")]
    EFileWrite,
    #[fail(display = "file read")]
    EFileRead,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KissError {
    pub kind: KissErrKind,
    pub code: i64,
    pub msg: String,
}

impl KissError {
    pub fn err(kind: KissErrKind) -> KissError {
        KissError {
            kind,
            code: 0,
            msg: String::new(),
        }
    }
    pub fn new(kind: KissErrKind, code: i64, msg: &str) -> KissError {
        KissError {
            kind,
            code,
            msg: msg.to_string(),
        }
    }
}

//----------------------------------------------------------------------------------------------

pub fn test_fire_error(i: u32) -> Result<String, KissError> {
    if i > 10 {
        Ok("ok done".to_string())
    } else {
        kisserr!(KissErrKind::ENetwork, "")
    }
}
pub fn test_enum_error() -> Result<String, KissErrKind> {
    Err(KissErrKind::EArgument)
}
pub fn test_bcos_error() {
    let r = test_fire_error(10);
    match r {
        Ok(v) => {
            println!("{:?}", v);
        }
        Err(e) => {
            println!("{:?}", e.kind.cause());
            println!("{:?}", e);
        }
    }

    let rr = test_enum_error();
    if let Err(e) = rr {
        println!("{:?}", e.as_fail().cause());
    }
}
