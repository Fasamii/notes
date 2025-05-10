#![allow (unused)]

// simple module
mod nest_a_1 {
    // simple nested module
    pub mod nest_a_2 {
        // public enum in nested module a_2
        pub enum Wasaba {
            Old,
            New,
        }
    }
}

// simple module
mod nest_b_1 {
    // nested simple module
    pub mod nest_b_2 {

        // re-exporting of nest_a_2 module
        // you can use nest_a_1 even if it's private module because it is your grand parent and
        // rust allows for using pribate parents only childs are restricted
        pub use super::super::nest_a_1::nest_a_2;

        // public function that uses Wasaba from nest_a_2 module
        pub fn let_a() {
            let _a = nest_a_2::Wasaba::Old;
        }
    }
}

// simple module
mod nest_c_1 {
    // simple nested module
    pub mod nest_c_2 {

        // importing nested b module (in which re-export is)
        use super::super::nest_b_1::nest_b_2;

        // simple foo
        fn use_re_export() {

            // because of re-exporting in nested b module we can use the nest_a_2 as it was defined
            // in nested b module thanks to re-exporting and use it using that b module
            let _b = nest_b_2::nest_a_2::Wasaba::New;
        }

    }
}

fn main() {

}
