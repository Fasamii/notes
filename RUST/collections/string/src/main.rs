fn main() {
    // String is type provided by rust std lib and is owned, mutable, grovalbe UTF-8 encoded string
    // string is implemented as a wrapper around Vec<T> type so some of the methods are common in
    // these two e.g.:
    //
    // .new() <- creates new instance of String
    //
    // .from(&str) <- creates new instance of string from string literal (&str)
    //
    // .push(char) <- used to push one element at the end of the heap structure (in String case char)
    //
    // .insert(usize, char) <- used to insert char to specified index
    //
    // .pop() <- removes the last item (char) from heap data structure
    //
    // .remove(usize) <- removes item (char) at specified index
    //
    // .crear() <- removes all content from heap data structure
    //
    // .len() <- returns length of the heap data structure from len filed of stack struct
    //
    // .capacity() <- returns capacity of heap data structure from cap filed of stack struct
    //
    // .reserve(usize) <- reserves space on the heap to ensure that there is enough space for later
    //
    // .repleace(&str, &str) -> <String> <- returns string with repelaced chars parameters are
    //                                      (from , to) can repleace single chars and full slices doesnt mutate orginal string returns
    //                                      new with repleaced char's
    //

    let s1 = String::from("woah");
    println!("{s1}");

    // String type exclusive methods (rather than common with Vec<T>)
    //
    // .push_str(&str) <- efficient way to append string slice to the String
    //
    // .insert_str(usize, &str) <- inserts string slice at specified index
    //
    
    // other methods releated with string
    //
    //  (any element that implements Display trait).to_string() -> String <- converts element into
    //                                                                       String type

    let str_temp = "woah this is so sick";
    // the to_stirng() method doesnt consume ownership of self
    // you can also use String::from() to do that both foo's are the same
    let to_str = str_temp.to_string();
    println!("str : {str_temp} \nstring : {to_str}");

    // because String is UTF-8 encoded you can use UTF-8 chars e.g.:
    // but careful with these one char can take more than one Byte of space in the heap structure
    // so can vary in size and that is error proune
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שלום");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    // you can also add String or &str to String using + operator e.g.:
    let s_bas = String::from("woah I'm base");
    let s_not_base = String::from("woah I'm not base");
    // the + operator refers to .add() method which signature looks like this:
    // fn add(self, s: &str) -> String
    // but you can se that add method takes &str not &String but code compiles and works fine this
    // is because compiler refference correction which throws away capacity filed of String type
    // struct which makes it identical to &str now
    let s_bas = s_bas + &s_not_base;
    println!("added stirngs : {s_bas}");

    // you cannot index into sitring like in most programing languages because Strings contain
    // UTF-8 encoded chars which can varry in size e.g.: one can be 1 Byte long other 2 Byte long
    // so indexing with Strings could result in returning half of char wihich calls panic!(); macro
    // to prevent that you can use various methods implemented on String type which can help you
    // with preventing error like these
    //
    // you can see that that as bytes array is 7 items long not 4 and that is because emoji is 4
    // Bytes long unicode character also note that 4 Byte long char is provided as 4 separate items
    // of the array and that is why it is error proune
    let str_exmp = String::from("123😊");
    println!();
    println!("raw string using .bytes(); : \n{:?}\n", str_exmp.bytes());
    println!("raw string using .as_bytes(); : \n{:?}\n", str_exmp.as_bytes());

    // the .char() method on String converts it to unicode scalar values converting the string
    // Bytes into chars (because char is 32 bit long unicode scalar value it can handle multibyte
    // unicode chars and store them as single char not separate Byte's) and that's why this for
    // loop prints String corectly (.chars() method returns iterator full of chars not Bytes)
    for ch in str_exmp.chars() {
        // each char is printed separatly as an unicode UTF-8 encoded char not Byte
        print!("{ch}");
    }; println!();

    // you can iterate over String one Byte for iteration (instead of chars) to do that use
    // .bytes() method
    for byte in str_exmp.bytes() {
        // be careful because now the byte variable may be half of the UTF-8 character
        print!("{byte}, ");
    } println!();

    //#////////////////////////#//
    // $> erm what the sigma <$ //
    //#////////////////////////#//
    use unicode_segmentation::UnicodeSegmentation;

    let s = "e\u{0301}"; // "e" followed by a combining accent
    let graphemes = s.graphemes(true).collect::<Vec<&str>>();
    println!("Grapheme clusters: {:?}", graphemes);

} 
