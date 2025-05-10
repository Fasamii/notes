#![allow (dead_code)]
#[derive (Debug)]
struct Example {
    name: String,
    status: bool,
}

fn main() {

    let ex0 = str_builder("idontseem");
    let ex1 = string_builder(String::from("toundersteand"));
    let ex2 = string_ref_builder(&String::from("allalone"));

    println!("{:?}", ex0);
    println!("{:?}", ex1);
    println!("{:?}", ex2);

    // even after passing ownership to string_builder() foo
    // we can acces name string via struct
    println!("{}", ex1.name);

}

// takes &str wich points to read only embeded to executable string literal
fn str_builder(name: &str) -> Example {
    Example {
        status: true,
        // then converts it from string literal to String by creating new string type
        // includeing heap allocation and data copy
        name: String::from(name),
    }
}

// takes String as an argument taking ownership of it
fn string_builder(name: String) -> Example {
    Example {
        status: true,
        // and uses directly as part of the struct 
        // this doesnt include any copy or heap alloc
        name
    }
}

// this takes pointer to string
fn string_ref_builder(name: &String) -> Example {
    Example {
        status: true,
        // but it works the same as taking &str
        // it need to alloc new heap memory and copy string there
        name: name.to_string(),
    }
}
