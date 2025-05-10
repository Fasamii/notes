#![allow (unused)]

// you can define standard enum just like in c
#[derive (Debug)]
enum Fieldless {
    Okay,
    NotOk,
}

// you can define enums wich can carry data witch them
// the data doesnt have to be the same (explained in main foo)
#[derive (Debug)]
enum DataCarrying {
    // the named filed data in this variant behaves similarry to struct but it is enum
    NamedFileds { 
        x: i32, 
        y: bool 
    },
    // struct for comparison
    // struct Name {
    //      x: i32,
    //      y: bool,
    // }
    //
    // the SingleString and MultipleVals variants of DataCarrying enum are tuples but String one
    // carry only one filed
    SingleString(String),
    MultipleVals(i32, u32, char),
}

// you can define enum wich data is another enum
#[derive (Debug)]
enum EnumEnum {
    Crazy(Fieldless),
    Omaga(DataCarrying),
}

use Fieldless::NotOk;

fn main() {

    // Fieldless enum behaves similary to c lang enums
    let c_like_enum = Fieldless::Okay;
    // that enum is of type: Fieldless and can be in state of variant Ok or NotOk
    // these can be matched or compared using match or if
    match c_like_enum {
        // you have to specify enum for variants
        // or use them like above use for Fieldless::NotOk
        Fieldless::Okay => println!("Ok"),
        NotOk => println!("not ok"),
    };

    print!("\n");

    // the named filed variant of enum can be definied similary to struct
    // they type is DataCarrying is DataCarrying
    let enum_with_data = DataCarrying::NamedFileds { 
        x:23,
        y:true
    };
    // you can pater match for that enum e.g.: if it's NamedFileds variant and get variables from
    // that named fileds
    match enum_with_data {
        // x as y and y as x 
        // first one name provided for the scope of prinln!(...) the second one variable inside
        // enum filed
        DataCarrying::NamedFileds { x:y , y:x } => println!("{x}, {y}"),
        _ => println!("what the fuck"),
    };

    print!("\n");

    // you can also define tuple like enum e.g.: MultipleVals and SingleString variants in
    // DataCarrying enum
    let enum_with_data = DataCarrying::SingleString(String::from("woah"));
    // they work the same as any other enum but carry toupe with itself

    // example for enum impl
    enum_with_data.print_contents();

    // you can define enum with other enum as data e.g.:
    let inception = EnumEnum::Omaga(DataCarrying::MultipleVals(23, 34, 'h'));
    // the type for inception is EnumEnum

}


// you can also implement for enums similarry like for structs e.g.:
impl DataCarrying {
    // creator of enum variant
    fn new_s() -> Self {
        DataCarrying::SingleString(String::new())
    }
    
    // fo to print content for evry possible variant inside enum
    fn print_contents(&self) {
        match self {
            Self::SingleString(s) => println!("{:?}", s),
            Self::MultipleVals(i, u, c) => println!("{i}, {u}, {c}"),
            Self::NamedFileds { x , y } => println!("{x}, {y}"),
        };
    }
}
