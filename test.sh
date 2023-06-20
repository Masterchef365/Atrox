cargo build --target=wasm32-unknown-unknown\
    && cargo run --bin basic\
    && cargo run --bin loader -- out.fn
    
