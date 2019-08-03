#![cfg(windows)]

pub extern crate math2d;

pub use crate::error::{Error, Status};

pub use winapi::shared::guiddef::GUID;

pub mod error;
pub mod helpers;
pub mod idltypes;
pub mod minwindef;
pub mod oaidl;
pub mod objidl;
pub mod ocidl;
