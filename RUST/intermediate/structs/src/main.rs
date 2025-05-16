#![allow (dead_code)]
#![allow (unused)]

// struct [name] {...} //
// for println({:?}) auto formating (without it wont work)
#[derive (Debug)]
struct User { // <- struct declaration
    active: bool, 
    username: String,
    email: String,
    sign_in_count: u64,
}

// you can declare tuple like structs
#[derive (Debug)]
struct RGB (u8, u8, u8);

// unit like struct
// they dont have any fileds and doesn't contain any data at all
// they bahave similary to () <- empty tuple (these are better explained in chapter 10 of rust book) 
#[derive (Debug)]
struct AlwaysEqual;

fn main() {
    // you can use that struct by just binding its definition to let [val]
    // if you want elemement of the struct to be mutable you have to declare 
    // entire struct as mutable bariable
    let mut user0 = User { // <- struct definition
        active: true,
        sign_in_count: 1,
        username: String::from("egypt"),
        email: String::from("faraon@pyramid.eu"),
    };
    println!("{:?}", user0);
    user0.active = false;
    println!("{:?}", user0);

    // if you wnat to reuse values from other struct to create new one you can use .. syntax to
    // fill remaning fileds
    let user1 = User {
        email: String::from("woah@huh.rar"),
        ..user0 // <- here it will fill evrything up expect email because it was explicitly set
    };

    // [!] - because string types from user0 struct doesn implement copy traing the ownership of
    // them was moved to user1 so these cannot be accesed from user0 anymore but because we 
    // definied diffrent email string for user1 email still can be accesed from user0 :3
    // println!("{}", user0.username); // <- error user0 doesnt have ownership for username anymore
    println!("{}", user0.email); // <- valid because we wasnt moved email into user1 from user0

    // touple structs
    let color_rgb = RGB(12, 224, 123);
    println!("{:?}", color_rgb);
    // do deconstruct tuple structs
    let RGB(r, g, b) = color_rgb;

    // [!] - note that evry variable of diffrent struct will have it's own type 
    // (evry struct is it's own type)
    // it is because foo that takes 3d_point tuple struct as parameter cannot take rgb tuple struct
    // as parameter so it is better for them to have own type
    
    // to use intendation in structs debuging use {:#?} in println! macro
    println!("{user1:#?}");

}

fn builder(active: bool) -> User {
    User {
        active, // you can use short hand for asigning variables to rust if they names are the same
        sign_in_count: 1,
        username: String::new(),
        email: String::new(),
    }
}
