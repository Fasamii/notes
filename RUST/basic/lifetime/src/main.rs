fn main() {
    let str = String::from("hello");
    println!("{str} - {:p}", &str);
    // the stack values are being coppied to the new str_b bur pointer to the heap memory
    // is being the same so str_b is pointing to the same memory as str
    let str_b = str;
    // let str_b = str.clone();
    println!("{str_b} - {:p}", &str_b);
    {
        let str_c = str_b;
        println!("{str_c} - {:p}", &str_c);
    }
    // in this situation we can't acces string in any way because the string was src_c variable
    // which was destroyed at the end of scope so string (array) on the heap doesnt exits
    // println!("{str_b}", str_b); // <- doenst work (str /b/c) doesnt exitst anymore

    print!("\n\n");

    let tuple: (i8, String) = (3, String::from("woah"));
    println!("{:?}", tuple);

    // if tuple uses only simple values it works as a simpe value and doenst need clonning
    // but if we include in tuple some val that uses heap then tuple need to be cloned or
    // you can't acces the old tuple variable
    let tuple_b = tuple.clone();
    println!("{:?}", tuple);
    println!("{:?}", tuple_b);

    print!("\n\n");

    // array is a variable type that implents copy trait so
    // it can be asigned to another variable and be usable
    let arr = [1, 2, 3];
    println!("{:?}", arr);
    let arr_b = arr;
    println!("{:?}", arr);
    println!("{:?}", arr_b);

    print!("\n\n");

    // example 1
    let s = String::from("hello");
    // because strings cannot be trivialy copied (they use some heap meory) 
    // the foo takes ovnership of the variable and gets dropped at the end of
    // the foo so s can't be accessed anymore (just like asigning string variable to new variable)
    //
    // you can use .clone() on s while passing to fo to pass only clone of s but it can be expensive
    //
    // you can also return that string back and use let on it again to gain ownership back
    takes_ownership(s);
    // println!("{s}"); // <- you cant do that because s contains heap allocations
    //
    // and because ints are trivialy copied (they implement Copy trait) foo only copies the
    // contents of variable and it gets dropped at the end off foo but because only copy was
    // dropped it can be stil accesed even after call to foo
    let x = 5;
    makes_copy(x);
    println!("{x}"); // <- you can do that because x is only stack variable
    //
    // foos can also only give ovnership if you return new value from it and use let on it

    // exampe 2
    // it needs to be mutable because of mutable reference to changes() foo
    // it doesnt need to be mutable if fo takes immutable reference
    let mut s_b = String::from("woah");
    // it takes pointer to s_b and foo drops the pointer but not string
    // so s_b can be still used
    borrows(&s_b);
    println!("{s_b}");
    // takes pointer to s_b wich can modify the variable it is pointing to
    // and still s_b can be used because onlyt mutable pointer to variable was droped
    changes(&mut s_b);
    //
    // you can have only one mutable refference to a variable at a time
    // the bellow example uses two mutable references on s_b but foo changes(r1); droops
    // the r1 reference so there is still only on mutable reference when calling changes(&mut s_b);
    // if you will create r1 and call changes(&mut s_b); there will be an error because &mut s_b is
    // used while r1 still exist
    let r1 = &mut s_b;
    changes(r1); // <- reference r1 drooped in that foo
    println!("{r1}"); // so r1 can be still accesed and AS IT IS IT LAST OCCURENCE COMPILER DROPS
                      // IT HERE and you are gaining possiblity to make references to s_b again
    changes(&mut s_b); // <- because of previous r1 droop it still can be used as ref
    // println!("{r1}"); // <- if you use r1 here it isnt droped on println!() line but here so
                         // changes() call with mut reference to s_b is invalid
                         
    // so invalid version would be:
    // let r1 = &mut s_b;
    // changes(&mut s_b); // <- tryin to use mut ref whe r1 exist
    // changes(r1); // <- r1 is dropped in that foo but code never compiles because of previous foo
    //
    // you can also use {...} to droop variables e.g.: mut ref's
    {
        let r2 = &mut s_b;
        println!("{r2}");
    } // <- r2 goes out of scope here
    // println!("{r2}"); // <- you can't do that because r2 is dropped in upper line because of
    // closing bracket if you remove {...} the r2 will be still accesible and droped automaticly
    // after last usage and because of using {...} we force it to be droped right now
    let r3 = &mut s_b;
    println!("{r3}");

    // if you have multiple references its ok but when you create mutable reference to variable
    // wich alredy have references you are fucked i mean cooked
    let ir1 = &s_b;
    let ir2 = &s_b;
    // let r4 = &mut s_b;  <- we cant do that because references to s_b alredy exist
    println!("{ir1}, {ir2}"); // <- if we remove the println!() call the ir1 and ir2 will
                              // be automaticly droped and you will gain ability to make mutable
                              // reference to s_b again but because of println!() macro the ir1/2
                              // is still in the scope (references to s_b exist) and we cant create
                              // mutable one
    let r4 = &mut s_b; // <- we can create mutable reference to s_b because last usage of
                       // other references was before creation of mutable reference so liftime of
                       // these already ended
    println!("{r4}");
    
    print!("\n");

    // created num
    let num: i8 = 3;
    // created reference to that num
    let r_num = &num;
    // it will print pointer to num
    // but you can also write &r_num it will print the pointer to the
    // pointer to num so *->*->num and without & it will be *->num
    // note thet rust automaticly tells type of var when writing &num as pointer to something
    // so you can fell like r_num is even macro to &num but it is real varible which is proved by
    // existence other pointer after trying to print &r_num;
    // (as always let puts new values on the stack)
    println!("{:p} |-| {:p}", &num, r_num);
    println!("{:p}", &r_num);

}

// foos for exampe 1 
fn takes_ownership(string: String) {
    println!("{string}");
}
fn makes_copy(inteager: i32) {
    println!("{inteager}");
}
// end of example 1
// foos for example 2
fn borrows(string: &String) {
    println!("{}", string);
}
fn changes(string: &mut String) {
    string.push_str(" ruah");
}
// en of example 2
