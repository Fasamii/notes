#![allow (unused_mut)]
// Vector is ( Vec<T> ) allows for stroing multiple values in one data structure, vector puts all
// the values next to each other in the memory and can stroe only values with the same type.
// vectors are similar to arrays but then can automaticly resize themselves.
//
// get(i) : O(1)
// insert(i) : O(n-i)*
// remove(i) : O(n-i)
// append(Vec(m)) : O(m)
// split_off(i) : O(n-i)
// range : n/a
// append : n/a

fn main() {

    // this is vector creation (you have to add type annotation if using new() method) 
    let mut v: Vec<i32> = Vec::new();
    println!("new vector : {v:?}");

    // there also exists an macro that will create macro from values it contains
    // because we are passing values at creation of the vector the type annotation isnt nessesary
    let mut v2 = vec!(1, 2, 3, 4, 5);
    println!("new vector with passed vals : {v2:?}");

    // to push value to the end of the vector use .push() method
    // in order to do that the vector must be mutable
    let mut i = 1;
    while i <= 10 {
        v.push(i); 
        i += 1;
    }
    println!("vector with pushed vals : {v:?}");

    // to get value from the vector you can index through it or use .get() method
    // via indexing
    let vec_2_ref: &i32 = &v[2];
    println!("reference to 2 index of v : {vec_2_ref:?}");
    //
    // via .get()
    // you need to handle Oprion<T> enum
    // you can use vec_3_ref and &vec_3_ref in Some([valname])
    //
    // .get(3) retruns &T type so the value vec_3_ref is a refference to that value in vector
    // the if let Some(vec_3_ref) binds returned &T to that balue (even if we dont write & it is
    // still an refference)
    if let Some(vec_3_ref) = v.get(3) {
        println!("vec_3_ref : {vec_3_ref:?}");
    } else {
        println!("failed to get refference to 3 index element of vector");
    }
    //
    if let Some(mut vec_3_ref) = v.get(3) {
        vec_3_ref = &9; // <- this is uslles (the value at 3 index of vector isnt mutable but
                        // refference to that value so it only changes reference (now reffers to
                        // 9) instead to value at vector 3 index) !! remember that .get(index)
                        // returns &T so you can do operation wich can be done on immutable
                        // refference
        println!("vec_3_ref : {vec_3_ref:?}");
    }

    if let Some(&vec_5_ref) = v.get(5) {
        // this code destructes the inner refference by applying the & pattern efficienly
        // derefferencing it wihich results in this binds vec_5_ref to value of type T which is
        // copy of value at 5 index of vector (if value type implements Copy trait)
        println!("&vec_5_ref from .get() method : {vec_5_ref:?}");

        // you can create copy of that value (if implement's copy trait)
        // if you work with non copy types use .clone() method on them
        let mut copy = vec_5_ref;
        copy += 2;
        println!("mutable copy of vec_5_ref : {copy:?}");
    }

    match v.get(4) {
        None => println!("well well"),
        Some(mut val) => {
            // val += 1; // <- cannot be done because the val is reference to the element at index
                         // 4 in the vector and only thin mutable there is the refference to that
                         // value not the value itself because .get(index) retsurn &T not mut T
            println!("v at index 4 is : {:?}", val + 1);
        },
    };

    // iterating through elements in vector using for loop
    // immutable version
    let v3 = vec!(1, 2, 3, 4);
    // the i is only refference to item in vector
    for i in &v3 {
        print!("[i : {i}] ");
    };
    println!();
    // mutable version
    let mut v4 = vec!(1, 2, 3, 4);
    // i is actual item in vector
    // using mutable borrwo because i want to use v4 in the code outside the loop
    for mut i in &mut v4 {
        // because i is only mutable borrow to element in v4 we have to dereference it with *
        *i += 1;
        print!("[mut i : {i}] ");
        // v4.push(89); // <- we can't do that because the for loop borrows v4 so as .push() method
                        // so borrow checker will complain about that
    }
    v4.push(89); // <- but after for loop with mutable refference ends we can do another mutable
                 // refference (yay)
    println!("\nv4 after mut loop : {v4:?}");

    // TODO: analise bellow code with radare //

    // if you need to store multiple variable types in one vector consider making enumeration which
    // will hold evry type of needed variable
    // this will work similary to union in c
    // this enumeration allocates for itself the size of biggest variation + space for tag that
    // tells which variant is active
    #[derive (Debug)]
    enum SpreadsheetCell {
        Int(i32), // <- 32 bit
        Float(f32), // <- 32 bit
        String(String), // <- 24 bit
    } // <- and tag size

    // make vector hold that enum insetead of values directly so it holds only SpreadsheetCell type
    // instead of multipe types of values
    let mut multi_type_vec = vec!(
        // evry variant of SpreadsheetCell is the smae size because of how enum works so this is
        // valid (but not evry time 100% of memory assigned to enum will be used) at least they can
        // change at runtime
        SpreadsheetCell::Int(32),
        SpreadsheetCell::Float(32.5),
        SpreadsheetCell::String(String::from("woah")),
        SpreadsheetCell::Int(48),
    );

    println!();
    println!("multi_type_vec : {:p} (on stack)", &multi_type_vec); // stack addres of heap vector
    println!("Int : {:p} : (on heap)", &multi_type_vec[0]); // item of vector
    println!("String : {:p} : (on heap)", &multi_type_vec[2]); // item of vector
    if let SpreadsheetCell::String(string) = &multi_type_vec[2] { // <-(and bellow) item of vector
        // due to some optimistaions compiler might put string type metadata on stack but it should
        // reside on heap as any other vector item
        println!("String.len : {:p} : (on stack) <- cause of compiler optimisations + not always a case", &string.len());
    }
    println!();

    for cell in &mut multi_type_vec {
        // if you create that type of vector you need to use pattern matchin to use assosiated
        // values
        match cell {
            SpreadsheetCell::Int(val) => {
                println!(" -- loop int : {val} (conversion)");
                *cell = SpreadsheetCell::Float(*val as f32);
                // println!("{val}") // <- is wrong because upper code
            },
            SpreadsheetCell::Float(val) => println!(" -- loop float : {val}"),
            SpreadsheetCell::String(val) => println!(" -- loop String : {val}"),
        };
    };
    println!("{multi_type_vec:?}");

    // useful methods on vector
    // creating vector from provided array (vec item will not be array but vector will convert it
    // to items in vector)
    let mut vec: Vec<u32> = Vec::from([1, 2, 3, 4]);
    println!("vec from [1, 2, 3, 4] : {vec:?}");
    // the same functionality as ::from() but with existing vector
    vec.extend([5, 6, 7, 8]);
    println!("vec after .extend() : {vec:?}");
    // .pop() removes last element from vector and returns it as an Option<T> where T is T of vector Vec<T>
    let val = vec.pop().expect("whoops");
    // .push() add's value to the end of vector
    vec.push(val);
    // vec has vec!() macro but also vec![<what>; <how manyu>] wichich will fill vector with <what>
    // of <how many> quantity
    let vec_of_0 = vec![0; 10];
    println!("vec_of_0 : {vec_of_0:?}");
    let mut vec_e = vec!([1,2,3]); // <- because of this vector will consist of arrays of len 3
                                   // to prevent that use vec![] macro instead
    vec_e.push([1, 5, 8]); // <- you can only pass to it arrays with size 3
    println!("vec of array's len 3 : {vec_e:?}");
    // to initialize vector when we know the minimal capacity it should have we can use    
    // it can decrease need for reallocation which can be slow
    let vec_with_cap: Vec<f64> = Vec::with_capacity(10); // <- will create vec with capacity 10 items
    println!("vec with cap 10 : {vec_with_cap:?}");
    // you can acces vectors via index because Vec type implements index trait
    // !! if you use index wich doesnt exist in this vector program will panic
    println!("vec accesed via index : {:?}", vec[2]);
    // you can also create slice of vector
    let slice: &[u32] = &vec[..3];
    println!("slice of vector : {slice:?}");

    // vectors wont dealloc it's memory automaticly event if are empty with capacity 1000000 you
    // need to do that manualy by callin .shrink_to_fit() or .shrink_to() methods on them
    //
    // .shrink_to_fit(&mut self) will deallocate unused memory of vector
    //
    // .shrink_to(&mut self, min_size: usize) will shirink to specified size or if vector uses more
    // memory that is specified will shring to fit vector data

    // clears vector (now it is empty)
    vec.clear();

    // .len() <- returns lenght
    // .capacity() <- returns capacity of the vector
    // .is_empty() <- return True if vector is empty

} // <- at the end of scope vector are droped and assosiated heap memory is freed
