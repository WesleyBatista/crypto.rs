// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[macro_use]
mod macros;

pub mod ciphers;
pub mod hashes;
pub mod keys;
pub mod macs;
pub mod signatures;
pub mod utils;

#[cfg(test)]
#[macro_use]
#[allow(unused_imports)]
extern crate alloc;

use core::fmt;

/// Error type of crypto.rs
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Buffer Error
    BufferSize { needs: usize, has: usize },
    ///  Cipher Error
    CipherError { alg: &'static str },
    /// Convertion Error
    ConvertError { from: &'static str, to: &'static str },
    /// Private Key Error
    PrivateKeyError,
    /// InvalidArgumentError
    InvalidArgumentError { alg: &'static str, expected: &'static str },
    /// System Error
    SystemError {
        call: &'static str,
        raw_os_error: Option<i32>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BufferSize { needs, has } => write!(f, "buffer needs {} bytes, but it only has {}", needs, has),
            Error::CipherError { alg } => write!(f, "error in algorithm {}", alg),
            Error::ConvertError { from, to } => write!(f, "failed to convert {} to {}", from, to),
            Error::PrivateKeyError => write!(f, "Failed to generate private key."),
            Error::InvalidArgumentError { alg, expected } => write!(f, "{} expects {}", alg, expected),
            Error::SystemError {
                call,
                raw_os_error: None,
            } => write!(f, "system error when calling {}", call),
            Error::SystemError {
                call,
                raw_os_error: Some(errno),
            } => write!(f, "system error when calling {}: {}", call, errno),
        }
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
