#![allow (unused)]
#[derive (Debug)]
struct Data {
    active: bool,
    bit_map: u8,
}

fn main() {
    let data = Data {
        active: true,
        bit_map: dbg!(2 * 8),
    };
    // the debug macro takes ovnership of passed value and returns that value
    // but also logs the evaluation and result with line numbers and file in wich dbg! macro was
    // called

    // debug types
    let string = String::from("2333");
    let number: u16 = match string.trim().parse() {
        Ok(result) => result, 
        Err(e) if *e.kind() == std::num::IntErrorKind::InvalidDigit => {
            println!("when your string which you are parisng contains something other that numbers eg 'c'");
            0
        },
        Err(e) if *e.kind() == std::num::IntErrorKind::Empty => {
            println!("when string which is parsed is empty");
            0
        },
        Err(e) if *e.kind() == std::num::IntErrorKind::PosOverflow => {
            println!("when num wich will be created from parse will be to big to stroe in type to wich it will try to parse to");
            0
        },
        Err(e) if *e.kind() == std::num::IntErrorKind::NegOverflow => {
            println!("the same with PosOverflow but if the num is to small to be stored in that data type e.g.: -1 in u8");
            0
        },
        Err(_) => { 0 },
    };
}
