struct Rectangle {
    width: u32,
    height: u32,
}

// impl keyword starts an implementation bloc for something inside implementation block we can
// define methods assosiated with type of for what we are implementing e.g.: in this case we are
// implementing for Ractangle type
impl Rectangle {
    // &self is short for self: &Self
    // in impl block self is an alias for the type that impl block is implementing for
    // e.g.: Rectangle
    // methods must have self as first parameter
    fn area(&self) -> u32 {
        // the foo returns new value (area wich can be asigned via let on method return)
        self.width * self.height
    }

    // the mutability of self stays the same as for any other variable
    // we can take ownership, take reference or take mutable reference to self
    fn set_zero(&mut self) {
        self.width = 0;
        self.height = 0;
    }

    fn set_dims(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    // you can name method the same as wariable name in struct compiler will know to which we
    // reffer to because after method we write pranticies "()" and that makes it diffrent from variable
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_fit_inside(&self, other: &Rectangle) -> bool {
        (self.width < other.width) && (self.height < other.height)
    }

    // you can also create assosiated function that doesnt take self as parameter
    // e.g.: for constructing rectangle
    //
    // instead of returning Rectangle you can just return Self because it is alias for type that we
    // are implementing for so returning Self and Rectangle is the same in that situation
    fn new(height: u32, width: u32) -> Self {
        Rectangle {
            height,
            width,
        }
    }
}

fn main() {
    // assosiated functions without self parameter are called with :: syntax instead of . syntax
    let mut rec = Rectangle::new(32, 2);

    println!("area of rec is {}", rec.area());
    rec.set_dims(10, 4);
    println!("area of rec is {}", rec.area());
    rec.set_zero();
    println!("area of rec is {}", rec.area());

    if rec.width() {
        println!("not zero");
    } else {
        println!("zero");
    }

    let bigger = Rectangle {
        width: 34,
        height: 40,
    };

    let smaller = Rectangle {
        width: 2,
        height: 3,
    };

    rec.set_dims(30, 30);
    println!("bigger {}", rec.can_fit_inside(&bigger));
    println!("smaller {}", rec.can_fit_inside(&smaller));

}
