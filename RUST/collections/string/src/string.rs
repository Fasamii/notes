// String is type provided by rust std lib
// it is owned *(The variable is responsible for its value (is owner of that string)),
// mutable, growable UTF-8 encoded string.
// String is implemented as a wrapper around Vec<u8> *(each UTF-8 char is based on 8-bit
// code units 1 char can vary in size from 1 to 4 Bytes that why Vec uses unsigned 8-bit).
// String is provided by std library and isn't implemented in core of rust language.
fn main() {

    // there is several methods to create an String.
    // 1st one is by using String::new() foo.
    let s = String::new();
    // When calling String::new() foo computer performs these operations:
    // 1. The String::new() foo is called.
    // 2. The String struct is created with a Vec<u8> field.
    // 3. The Vec::<u8>new() is initialized with RawVec buffer (using RawVec::new()) with len 0
    // 4. RawVec::new() initializes with a dangling pointer from Unique::new() cap set to 0 and
    //    global allocator
    // 5. Creation of non-null pointer (typically set to The alignment value of u8, which is 1)
    //    that is never meant to be dereferenced
    // This involves following CPU level operations:
    // - Register allocations
    // - Stack frame creation
    // - Setting The dangling pointer value to 1
    // - Setting capacity and length fields to 0
    // - constructing The nested data structures
    // - Returning The final String struct
}
