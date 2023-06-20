use std::{marker::PhantomData, collections::HashMap};

use serde::{Serialize, Deserialize};

struct Runtime {
    own_bytecode: &'static [u8],
    wasm: wasmtime::Engine,
    own_module: wasmtime::Module,
    own_fns: HashMap<String, >
}

impl Runtime {
    /// Should be supplied the wasm32 equivalent of the crate using this library.
    /// So compile your own lib as wasm32, and then compile for the native platform.
    /// This is so your program can send its source to other programs!
    pub fn new(own_bytecode: &'static [u8]) -> Self {
        let wasm = wasmtime::Engine::new(&Default::default()).unwrap();
        let own_module = wasmtime::Module::new(&wasm, own_bytecode).unwrap();
        for  own_module.exports()

        Self {
            wasm,
            //own_source: own_source.to_vec().into_boxed_slice().into()
            own_module,
            own_bytecode: own_bytecode.into(),
        }
    }

    pub fn new_fn<Input, Output, F: Fn(Input) -> Output>(&self, f: F) -> DynFn<Input, Output> {
        // Bad name bodge lol
        let name = std::any::type_name::<F>();

        // Get the absolute name of this function
        let name = name.split("::").last().unwrap();
        let symbol = format!("__atrox_{}", name);

        // Get the equivalent symbol
        DynFn { _phantondata: Default::default(), symbol, source: self.own_bytecode.to_vec()  }
    }

    pub fn exec<Input, Output>(&self, f: &DynFn<Input, Output>, input: &Input) -> Result<Output> {
        let input_bytes = bincode::serialize(&input) ;


    }
}

#[derive(Serialize, Deserialize)]
pub struct DynFn<Input, Output> {
    source: Vec<u8>,
    symbol: String,
    _phantondata: PhantomData<(Input, Output)>,
}

impl<Input, Output> DynFn<Input, Output> {
}
