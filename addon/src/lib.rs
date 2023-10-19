#![deny(clippy::all)]
#![feature(absolute_path)]
#[macro_use]
extern crate napi_derive;

pub mod clipboard;
pub mod monitor;
pub mod shotcut;
pub mod simulation;
