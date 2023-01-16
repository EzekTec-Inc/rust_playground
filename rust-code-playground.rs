#![allow(dead_code)] // This macros decl suppresses, globally, all dead code/functions not called/consumed in 'fn main()'
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, Timelike};
use std::fmt; // bringing the standard library into scope in order to leverage functions/methods in the library // datetime crate for ease of managing/processing date & time. Must be imported in 'Cargo.toml' file.

#[derive(Debug, Copy, Clone)]
struct StructExample<'a> {
    name: Option<&'a str>,
    topics: Option<Vec<&'a str>>,
    updated_on: Local, // NOTE: this can potentially yield an Error state where the date remains the same even after the user has updated the other struct properties.
}

impl<'a> Default for StructExample<'a> {
    fn default() -> Self {
        Self {
            name: None,
            topics: None,
            updated_on: Local::now,
        }
    }
}
