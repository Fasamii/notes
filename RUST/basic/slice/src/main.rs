fn main() {
    // to get the bytes of char use b'[char]' syntax e.g.:
    // b'~' will result in 126 in binary
    println!("{:?}", b'~');

    print!("\n");

    let str = String::from("woahslsl kmks sadfasdf s");
    let index_of_end_of_first_word = first_word(&str);
    println!("index of end of first word in \"{str}\" is {index_of_end_of_first_word}");

    print!("\n");

    // r_str is slice of String that contains chars from index 0 to index of end of first word
    // you can drop values in range syntax e.g.: (2.. , ..2 , ..) it will go
    // (2..) -> from the index of 2 to the end
    // (..2) -> from the start to index of 2
    // (..) -> from the start to the end
    // the .. takes slice of entire string
    // IMPORTANT: that types of slice doesnt work well with multibyte utf-8 chars
    // it will be discused in the 8 chapter of rust book
    // ASCII works just fine :3
    let r_str = &str[0..index_of_end_of_first_word];
    // the r_str variable is "fat pointer" because it not only stores pointer to the data on the
    // heap but also size of that data it's type is &str and contains pointer and lenght of str
    // it is result from slice operation, it must be a pointer because it contains data which size
    // is known only at runtime (part of the string of the String type)
    //
    // the slices are good because they are tied to underlying data and doesnt provide much of
    // overhead since esential data are arleady allocated and stored on the heap
    println!("\x1b[48;5;55m{r_str}\x1b[0m");

    // this operation cannot be performed an compiler will detect that because we will try to make
    // mutable borrow witch that .clear() foo and we have already borowed it creating a slice so modyfing it
    // will break the program and compiler knows it
    // str.clear();
    // println!("{r_str}");

    println!("{str}");
    println!("the 2 word is: {}", nth_word_slice(2, &str));
    println!("the 1 word is: {}", nth_word_slice(1, &str));
    println!("the 0 word is: {}", nth_word_slice(0, &str));

    print!("\n");

    let _s = "string literal";
    // the type of s is &str which points to the place in executable where "string literal" is
    // stored it's the same type as slice but slice points po part of the string on the heap
    // &str in immutable reference

    print!("\n");

    // you can also make slice of an array
    // or any general variable on wich slice is possible
    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[1..3];
    println!("{:?}", arr);
    println!("{:?}", arr_slice);
    // assert_eq!() verifies if two variables are equal to each other if not it will panic
    // since slice [1..4] is [2, 3] values from array assert_eq macro will pass
    assert_eq!(arr_slice, &[2, 3]);
}

fn first_word(str: &String) -> usize {
    // changes chars into bytes e.g.: "~" into 126
    let bytes = str.as_bytes();
    // makes iterator and adds enumerate to it wich returns a tuple witch index of each byte and
    // contents of that byte
    for (i, &item) in bytes.iter().enumerate() {
        // checks if returned content of byte is equal to byte equvalent of space
        if item == b' ' {
            // if is returns index of that space
            return i;
        }
    }
    // or if non of the chars in str is space it return length of string
    str.len()
}

fn _first_word_slice(str: &String) -> &str {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[..i];
        }
    }
    &str[..]
}

// if instead using &String as a parameter we use &str it will be more usful because it will work
// in foo calls with (string literals, slices)->&str and Strings will implicitly deref correct 
// itself which will make api more general without losing any functionality
//                                 ----
fn nth_word_slice(nth: usize, str: &str) -> &str {
    let mut count = 0;
    let mut start = 0;
    for (i, &item) in str.as_bytes().iter().enumerate() {
        // println!("loop {i} & {item}");
        if item == b' ' {
            // println!("is space");
            if count == nth{
                // println!("return of ({last_space_index})>({i})");
                return &str[start..i]
            } else {
                // println!("asign to last space index ({i})");
                start = i;
            }
            count += 1; 
        }
    }
    &str[start..]
}
