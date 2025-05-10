const ON_OFF: bool = true;

fn main() {
    // just if statment...
    // you cant pass number to if statment it only takes bool's so you
    // firstly need to use some logic to evaluate num to bool type.
    // e.g.: if number != 0 {...}
    if ON_OFF {
        println!("it was on");
    } else {
        println!("it was off");
    }
    // you can use the if to chose the output value of if because if is an expresion
    // note that the currly brackets in bellow expresion are part of if stantment and dont stand on
    // them own.
    let _x = if ON_OFF { 4 } else { 2 };
    // you can also use match keyword for that purpouse
    let y = match ON_OFF {
        true => 4,
        false => 23,
    };
    println!("y:{}", y);

    print!("\n\n");

    // you can use loop to get the value
    // if you provide value at the end of break keyword the loop {} will return that
    // value
    //
    // !!! note that we use break keyword to exit the loop with and return value if we would use
    // return the result will be exiting the foo because the return keyword always refers to the foo
    let mut counter = 0;
    let woah = loop {
        println!("woah counter: {}", counter);
        if counter > 3 {
            break counter;
        }
        counter += 1;
    };
    println!("woah is: {}", woah);

    print!("\n\n");

    // you can also brake the taged loop being in other loop inside that loop
    'layer_one: loop {
        counter += 1;
        println!("woah counter: {}", counter);
        if (counter % 2) == 0 {
            loop {
                if (counter % 5) == 0 {
                    counter += 1;
                    println!("woah counter: {}", counter);
                } else {
                    break 'layer_one;
                }
            }
        }
        if counter > 40 { break };
    }

    print!("\n\n");

    // the next are while loops the will loop if given condition is true
    let mut till = 3;
    while till != 0 {
        println!("{till}");
        till -= 1;
    }
    println!("yay");

    print!("\n\n");

    // if you want to loop through array dont (DONT) use while loop
    // it is error prone because you need a max arr size to have something to
    // check for end of array and COMPILER ADDS RUNTIME CODE TO CHECK IF YOU ARE ACCESSING
    // DATA FROM INSIDE ARRAY
    //
    // better alternative for that is using for loop
    let array: [&str;4] = [ "woah", "guh", "meow", "wihi" ];
    for element in array {
        println!("{:?}", element);
    }

    print!("\n\n");

    // you can also make countdown from while loop example with for using range as follows
    // note that rev() is for reversing the numbers from 1, 2, 3 -> 3, 2, 1
    //            (1..4) <- the not includeing range
    for number in (1..=3).rev() {
        println!("{number}");
    }
    println!("LIFTOF");
}
