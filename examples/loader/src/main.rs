#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use atrox::runtime::*;

    let runtime = Runtime::new(include_bytes!(
        "../../../target/wasm32-unknown-unknown/debug/loader.wasm"
    ));

    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap();
    let ser_fn = std::fs::read(&path).unwrap();
    let dyn_fn: DynFn<i32, i32> = atrox::bincode::deserialize(&ser_fn).unwrap();

    dbg!(runtime.call(&dyn_fn, &1337));
}
