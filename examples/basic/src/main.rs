fn main() {
    dbg!(attorney_general_foo_barr(99));
}

#[atrox::generate_function]
fn attorney_general_foo_barr(a: i32) -> i32 {
    a * 2
}
