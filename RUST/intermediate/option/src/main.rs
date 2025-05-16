#![allow (unused)]
// the option enum is definied in standard lib as follows:
// it provides functionality for returning nothing from foo that have possibility for not getting
// desired data
// enum Option<T> { // <- the <T> indicades that Option type can be of type Option<T(type)> so e.g.:
//                  // Option<i8> or Option<bool> or even Option<String>. That type thing is usful
//                  // because it allows to declare on enum wich can have multiple of data types
//                  // without declaration for each one
//    None,
//    Some(T), // <- the <T> is utilized here
// }
// INFO: commented out due to this declaration already exist in rust stdlib

fn main() {

    // examples of using option to hold values
    let x: Option<char> = Some('x');
    let x: Option<i8> = Some(23);
    let x = Some(String::from("woah"));
    // or not
    // here the None isnt any keyword it is variant of Option enum
    // and thust x is of type Option None can be assigned to it
    let x: Option<u32> = None;
    // you cant assign None to variable without specyfing type because compiler wont know
    // what is the type for Some(T) so below statment is invalid
    // let x = None;

    let z = 5;
    let y = Some(4);
    // the type of y is Option<i32> but we can't do operations betwen Option<i32> and i32 because
    // these are other types e.g.:
    // let sum = y + z // will result in compile error
    // before we can use value held in Some(T) we must convert it from Option(T) to T

    let some = Some(4);
    println!(".is_some_and() -> {}", some.is_some_and(|x| x == 4));
    let none: Option<i32> = None;
    println!(".is_none_or() -> {}", none.is_none_or(|x| x < 2));
    println!(".is_none_or() -> {}", some.is_none_or(|x| x < 2));

    let mut some = Some(123);
    let none: Option<char> = None;
    let option_slice = some.as_slice();
    let none_slice = none.as_slice();

    println!("{some:?}");
    println!("{option_slice:?}");
    println!("{none_slice:?}");

    println!("{}", some.expect("woops"));
    // println!("{}", none.expect("woops"));

    let string = Some("what the sigma".to_string());

    let maybe_len = string.clone().map(|x| {x.len()});

    println!("len of string Option is {:?}", maybe_len);
    println!("len of string Option is {:?}", maybe_len.unwrap());
    println!("len of string Option is {:?}", maybe_len.expect("something broke"));

    string.map(|x| {
        for i in x.chars() {
            print!("[\x1b[38;5;2m{i}\x1b[0m]");
        }
        println!();
    });

    some.inspect(|x| println!("some is {x}"));
    println!("{some:?}");

    let result = some.ok_or(0);
    println!("{result:?}");
    assert_eq!(result, Ok(123));

    let result = some.ok_or_else(|| 0);
    println!("{result:?}");
    assert_eq!(result, Ok(123));
}
