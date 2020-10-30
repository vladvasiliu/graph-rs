#![feature(trace_macros)]

pub extern crate inflector;
pub extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate from_as;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

pub mod traits;

pub mod builder;
pub mod diff;
pub mod generator;
pub mod macros;
pub mod parser;
