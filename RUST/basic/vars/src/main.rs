// consts can be declared in evry scope but cannot be changet later
// they are evaluated at compile time so 60 * 60 * 1 will be 3600 before runtime
// they need to be upper case
const ONE_HOUR_IN_SEC: u32 = 60 * 60 * 1;

fn main() {
    println!("One hour in seconds: {ONE_HOUR_IN_SEC}");

    let x: u32 = 5;
    println!("&X:{:p}", &x);
    // writing just prantacies without any keyworkd
    // will create scope for variable where if it will
    // schadow var from out of scope the schadowing will be only
    // in that scope so after scope ends schadowing aslo ends.
    //
    // schadowing creates separate space in memory for new var
    // so after scope ends old var can be accesed
    //
    // if running in loop the schadowing should use the same memory space for evry iteraion
    // so var1 => addr1 ; var2 => addr2 etc..
    {
        // even if x is not used after end of scope the new memory addres will be used for let
        // keyword if you want to reuse that addres use mut keyword
        let x = 6;
        println!("schadowed &X:{:p}", &x);
        println!("X:{x}");
    }
    println!("&x:{:p}", &x);

    print!("\n\n");

    // the isize is the size of int on sys architecture eg 64 bit system = 64 bit isize or usize
    // you can perform platform detection witch this e.g.: but it isnt the best aproach
    if isize::MAX == i64::MAX.try_into().unwrap() { println!("using 64 bit system"); };
    if isize::MAX == i32::MAX.try_into().unwrap() { println!("using 32 bit system"); };
    if isize::MAX == i16::MAX.try_into().unwrap() { println!("using 16 bit system"); };
    if isize::MAX == i8::MAX.try_into().unwrap() { println!("using 8 bit system"); };

    println!("using {} bit system", isize::BITS); // think outside the box fucker
                                                  // still not the best aproach

    print!("\n\n");

    // example overflow behaviour
    // debug: the thread will panic and exit
    // release: the numbers will wrap eg 8bit:MAX + 1 = 8bit:MIN
    println!("max:{}", u8::MAX);
    let mut small_num: u8 = u8::MIN;
    loop {
        print!("\x1b[38;5;2m{small_num}\x1b[0m,");
        // std::thread::sleep(std::time::Duration::from_millis(1));
        if small_num == u8::MAX {
            print!("\n");
            println!("\x1b[38;5;1mERROR\x1b[0m WILL OCCUR (u8 OVERFLOW)");
            println!("      MAX IS {} (AFTER ADD u8 will be {})", u8::MAX, "256");
            println!("\x1b[38;5;1mABORDING\x1b[0m TO PREVENT INF LOOP");
            println!("         if in debug thread will panic but in release it will");
            println!("         overflow from 255 to 0 because of complement wrapping");
            println!("         over the maximum value that bits can store causing inf loop");
            break;
        }
        small_num = small_num + 1;
    }

    print!("\n\n");

    // example of float (in rust only f32 and f64 exist)
    let floating_normal: f64 = 0.34;
    println!("fl_n: {}", floating_normal);
    // getting infinity
    let inf = f64::INFINITY;
    println!("INFINITY: {}", inf);

    print!("\n\n");

    // tuple doesnt have meta data like .lenght
    // they cannot be dynamicly accesed (compiler need to know what data
    // will be accesed) and because touples can contain diffrent types of datea
    // it wouldnt know
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tuple;
    // the let statment for bringing in touple values (descructing)
    // allocates new memory on the stack and copies data from the touple if they have
    // Copy trait if they dont data is moved into new binding so ownership is moved to new binding
    println!("&x let:{:p}\n&x tup:{:p}", &x, &tuple.0);
    println!("x:{x}, 0 from touple: {}", tuple.0);
    println!("y:{y}\nz:{z}");

    //you can also use tuple to return multiple values from the foo :)
    
    // you can also declarate empty tuple e.g.:
    let empty = ();
    println!("{:?}", empty);

    print!("\n\n");

    // arrays can be dynamicly accesed because
    // they contain only one type of data so compiler know 
    // what data type will be accesed even with dynamic acces
    let array: [i128;0] = [];
    println!("{:?}", array);
    let array: [i8;3] = [0, 2, 4];
    println!("{:?}", array);
    
    // the pointer to array and pointer to array[0] zero are the same because 0 is at beginning of
    // the array
    println!("array pointer -> {:p}", &array);
    println!("array index 0 pointer -> {:p}", &array[0]);
    // the let index_zero have other pointer because of the let keyword
    // it contains copy of array[0] data
    // it kinda behaves like schadowing other variables
    // and the reusing memory in loops stays the same
    let is_even = true;
    let mut iteration = 0;
    let mut index;
    loop {
        if is_even {
            index = 1;
        } else {
            index = 2;
        }
        let index_zero = array[index];
        iteration = iteration + 1;
        println!("array index 0 after let pointer -> {:p}", &index_zero); 
        if iteration >= 10 { break; }
    }
    // you can get the len of array by calling a foo on it
    println!("\narray size is {}", array.len());
    // you can still get out of bounds error while working with arrays
    
    print!("\n\n");

    // the ranges in rust are inplemented as a lightweight struct that holds the upper and lower
    // bounds the compiler translates range syntax into one of these structs
    //
    // you can use inclusive x..=y 
    // and exclusive range x..y
    let mut range = 1..4;
    println!("{:?}", range);
    // the range is used through that call on it
    // it increments the lower number by one until its
    // the equal to upper val or one smaller than upper val
    // depending on range type (inclusive/exclusive)
    range.next();
    println!("{:?}", range);
    // TODO: check what that foo does
    // let mut range = range.into_iter();
    // 
    // you cannot create ..2 (RangeTo) and use .next() on it
    // because it isnt containing any value to increment but is useful in creating String slices
    // and can be used without .next()
}
