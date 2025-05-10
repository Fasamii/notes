#![allow (unused)]

// uses module with path std/collections
use std::collections;

// allows to make multiple imports in one line with nested import syntax
// note that self refers to itself that you can make import for io and io::Write
use std::{io::{self, Write}, cmp::Ordering};

// this is glob operator which allow us to import all public items inside specified module
use std::collections::*;

fn main() {
    println!("this program makes so much ahhh");
}
