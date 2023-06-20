#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use atrox::runtime::*;

    let runtime = Runtime::new(include_bytes!("../../../target/wasm32-unknown-unknown/debug/basic.wasm"));

    let f = runtime.new_fn(attorney_general_foo_barr);

    //DynFn::from_fn(attorney_general_foo_barr);
}

#[atrox::generate_function]
fn attorney_general_foo_barr(a: i32) -> i32 {
    a * 2
}
