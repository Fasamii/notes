use std::{collections::HashMap, str::FromStr};

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("alfa"), 8);
    scores.insert(String::from("beta"), -8);
    // overrides last value with that key
    scores.insert(String::from("beta"), 80);
    println!("{scores:?}");

    // if we want a owned value you can do that via calling .get() method which returns Optin<&T>
    // then call .copied() which converts Option<&T> into Option<T> if input to .copied() is None
    // it just propagates None and the .unwrap_or() method unwraps Option<T> into T or if None uses
    // provided value in method call e.g.: 90.
    // you can think why that order why not first call .unwrap_or() then .copied() to make
    // .copied() not handle None case these are two arguments why first .copied() then .unwrap_or()
    // 1. is that .copied() method is implemented on Option<T> so it will not work on not Option<T>
    // 2. if you will unwrap_or first you woyld have to provide refference as the or value and then
    //    unwrap that refference into owned value which is unnesesary work in .copied().unwrap_or()
    //    order you can provide owned value because Option<T> is already derefferenced and in cane
    //    of None no unnesesary conversions which can make code less readable exists
    // case with existing key
    let score = scores.get("alfa").copied().unwrap_or(90);
    println!("alfs key : {score}");
    // case with not existing key
    let score = scores.get("gamma").copied().unwrap_or(90);
    println!("gamma key : {score}");

    // you can iterate over each key-value pair in hashmap usin for loop
    // you have to use refference to scores in for loop is because without & it will consume
    // ownership of passed value and with & only consumes refference to that value making it usable
    // later in code
    for (key, value) in &scores {
        println!("scores for loop : {key} and {value}");
    };
    // possible because only refference is passed to for loop and scores variable isnt dropped yet
    println!("{scores:?}");

    // ownership in passing to hash map
    //
    // if type implements Copy trait it is copied into hask map if not e.g.: String values will be
    // moved into the hash map so
    let s1 = String::from("string");
    let i1 = 23;
    let mut map2 = HashMap::new();
    map2.insert(s1, i1);
    // the s1 is moved into the hash map because it doesnt implement Copy trait you can always call
    // .clone() method on that to make it doesnt move that value
    //
    // i1 is still valid event after .insert() method because it implements Copy trait and can be
    // eaisly copied int the hash map
    // println!("s1 : {s1}"); // <- isnt possible because .insert() takes ownership
    println!("i1 : {i1}"); // <- is possible because i32 implements Copy trait and can be eaisly
                           // copied into the hash map so it is coppied and still valid

    // if you want to change value in hash map only if key doesnt already exist you can use
    // .entry() method which takes key and and returns enum called Entry that represents if key
    // exists on not
    println!("{:?}", scores.entry("alfa".to_string()));
    println!("{:?}", scores.entry("sigma".to_string()));
    // you can use that enum to write code which will do diffrent things based on the enum variant
    // e.g.: use .ot_insert() method which will return mutable reffrence to provided value and
    // insert it to hash map if key doesnt exist or if exist it will return mutable refference to
    // value coresponding to existing key
    println!("entry \"gg\" : {:?}", scores.entry("gg".to_string()).or_insert(99));
    println!("entry \"alfa\" : {:?}", scores.entry("alfa".to_string()).or_insert(99));
    println!("scores : {scores:?}");

    // if you want to change value based on the old value you can do that using * thanks to hask
    // map returning mutable refferences e.g.:
    let mut map3 = HashMap::new();
    let text = "woah this is so good i really like rust programing language i mean rust is so great language rust rust";
    for word in text.split_ascii_whitespace() {
        let value = map3.entry(word).or_insert(0);
        *value += 1;
    }
    // shorter version of that for
    // for word in text.split_whitespace() { *map3.entry(word).or_insert(0) += 1 }

    println!("words count : {map3:?}");
    // and then print the most used word
    let mut max_word: (&str, i32) = ("", 0);
    if map3.is_empty() { println!("is empty"); }; // <- you can use that method to prevent errors
                                                  // on empty map
    for (&key, value) in &map3 {
        if *value > max_word.1{ // check if this isnt comparing refferences rather than vals
            max_word.1 = *value;
            max_word.0 = key;

        } 
    }
    println!("most used word : {} it apears {} times", max_word.0, max_word.1);
}
