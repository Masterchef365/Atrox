fn main() {
    dbg!(attorney_general_foo_barr_generated());
}

#[atrox::generate_function]
fn attorney_general_foo_barr(a: i32, b: i32) -> i32 {
    a - b
}
