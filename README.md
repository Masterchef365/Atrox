# Atrox
Portable functions as data in Rust (WIP)

First you compile your executable as WASM, then it is included in your native (x86, arm) binary. You can then serialize your functions as e.g. json or bincode, and then send them to another application. The application can then run those functions!

For example, here we write a function to a file:
```rust
/// This is the function we are exporting
#[atrox::generate_function]
fn attorney_general_foo_barr(a: i32) -> i32 {
    a * a
}

// We can pick an individual (annotated) function out of our binary
let f = runtime.new_fn(attorney_general_foo_barr);

// And send it elsewhere!
let ser_fn = bincode::serialize(&f).unwrap();
//let ser_fn = serde_json::to_vec(&f).unwrap();

std::fs::write("out.fn", &ser_fn).unwrap();
```

And then we can use it from any other Rust program (on any platform supported by wasmtime!)
```rust
let ser_fn = std::fs::read("out.fn").unwrap();
let dyn_fn: DynFn<i32, i32> = bincode::deserialize(&ser_fn).unwrap();
//let dyn_fn: DynFn<i32, i32> = serde_json::from_slice(&ser_fn).unwrap();

dbg!(runtime.call(&dyn_fn, &1337));
```

This should output `1787569`.

# Limitations
* You must be able to compile your application to WASM
* Atrox relies on serde - your function arguments must be serializable and deserializable.
