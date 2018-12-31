#![cfg(windows)]

#[macro_use]
extern crate derive_com_wrapper;

#[macro_use]
extern crate auto_enum;

#[macro_use]
extern crate derive_com_impl;

pub extern crate math2d;

pub use crate::error::{Error, Status};

pub use winapi::shared::guiddef::GUID;

pub mod error;
pub mod helpers;
pub mod minwindef;
pub mod objidl;
pub mod propvariant;
