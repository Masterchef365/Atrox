use atrox::runtime::DynFn;

#[cfg(not(target_arch = "wasm32"))]
const THIS_CRATES_WASM_EQUIV: &[u8] = include_bytes!("../../../target/wasm32-unknown-unknown/debug/basic.wasm");

fn main() {
    DynFn::from_fn(attorney_general_foo_barr);
}

#[atrox::generate_function]
fn attorney_general_foo_barr(a: i32) -> i32 {
    a * 2
}
