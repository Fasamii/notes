use std::{panic, process};

const PANIC: bool = false;
const INDEX: bool = false;
const FILE: bool = true;

// Sometimes Rust requires you to acknowledge the possibility of an error and take action before
// you code will compile
//
// Rust groups error int two categories:
//  - Recoverable -> e.g.: the error may be returned to the user and program may retry operation
//  - Unrecoverable -> are mostly symptoms of bugs e.g: accessing memory not belonging to the
//                     program which will immediately stop it
//
//  Rust cover errors with Result<T, E> type and panic macro
fn main() {
    // Unrecoverable errors with panic!()
    // By default panic!() will
    //  - print failure message
    //  - unwind -> which means that rust will walk back up the stack and clean data from each
    //              function it encounters, but that is much operations to do so Rust gives you a
    //              way to change that default behavior to "aborting" which will close the program
    //              without cleaning up the memory. (memory that program was using will then need
    //              to be cleaned up by the operating system). Aborting also have benefit it makes
    //              the target binary smaller.
    //              INFO: (see Cargo.toml file for more explanations)
    //  - clean up the stack
    //  - quit the program

    if PANIC {
        panic!("woah")
    };

    // sometimes panic may be called from someone else code which your code calls in that case it
    // would be useful to see which call to that code caused panic, to do that you can use "RUST_BACKTRACE"
    // env variable which contains number of how deep the back trace should be

    // example code in which "panic!" is called
    #[allow(clippy::useless_vec)]
    let vec = vec![1, 2, 3];
    let _element: Option<usize> = if INDEX { Some(vec[3]) } else { None }; // <- trying to access

    // You can use Result type if operation have chance to fail and then handle errors using match
    // keyword e.g.: do A if Result is Ok<T> or B if Result is Err(E)
    //
    // Result is defined as:
    //  enum Result<T, E> {
    //      Ok(T),
    //      Err(E),
    //  }
    // The T and E are generic type parameters INFO: (generics will be covered in chapter 10 of the rust book)
    // byt you need to know that:
    //  - T -> represents the type of the value returned in a success case with the Ok() variant
    //  - E -> represents the type of the value returned in a failure case with the Err() variant
    //
    // example use case of Result:
    if FILE {
        // importing file functionality form std lib
        use std::fs::File;
        // opening file with relative path ./hello.txt and then matching result enum against
        // possible variants
        let file = match File::open("hello.txt") {
            Ok(file_handler) => file_handler,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(file_handler) => {
                        log("creating file");
                        file_handler
                    }
                    Err(err) => {
                        log(&format!("error : {}", err));
                        panic!();
                    }
                }
            }
            Err(err) => {
                log(&format!("error : {}", err));
                panic!();
            }
        };

        // the thing that is inside Err() variant from File::open() or create() is io::Error type
        // it is a struct provided by std library
        // we can call .kind() on that struct to get io::ErrorKind this is enum provided by std
        // library and contains various variants of errors that might result from io operation
        println!("file : {file:?}");
    }

    // the Result enum has many helper methods defined on it.
    //
    // e.g.: the .unwrap() method is a shortcut method implemented just like match expression which
    // returns value on Ok(T) and panics on Err(err)
    //
    // and the .expect() method which works very similarly to .unwrap() but lets us chose the error
    // message

}

fn log(msg: &str) {
    println!("\x1b[38;5;5m◖\x1b[0;48;5;5;30;1m {msg} \x1b[0;38;5;5m◗\x1b[0m");
}
