
mod nest_a_1;
mod nest_b_1;
mod nest_c_1;

use nest_c_1::nest_c_2;

fn main() {
    nest_c_2::use_re_exporting();
}
