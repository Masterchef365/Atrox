use std::{collections::HashMap, marker::PhantomData};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub struct Runtime {
    own_bytecode: &'static [u8],
    wasm: wasmtime::Engine,
    own_module: wasmtime::Module,
}

#[derive(Serialize, Deserialize)]
pub struct DynFn<Input, Output> {
    source: Vec<u8>,
    symbol: String,
    _phantomdata: PhantomData<(Input, Output)>,
}

impl Runtime {
    /// Should be supplied the wasm32 equivalent of the crate using this library.
    /// So compile your own lib as wasm32, and then compile for the native platform.
    /// This is so your program can send its source to other programs!
    pub fn new(own_bytecode: &'static [u8]) -> Self {
        let wasm = wasmtime::Engine::new(&Default::default()).unwrap();
        let own_module = wasmtime::Module::new(&wasm, own_bytecode).unwrap();

        Self {
            wasm,
            //own_source: own_source.to_vec().into_boxed_slice().into()
            own_module,
            own_bytecode: own_bytecode.into(),
        }
    }

    /// Find the corresponding function in our own WASM bytecode
    pub fn new_fn<Input, Output, F: Fn(Input) -> Output>(&self, f: F) -> DynFn<Input, Output> {
        // Bad name bodge lol
        let name = std::any::type_name::<F>();

        // Get the absolute name of this function
        let name = name.split("::").last().unwrap();
        let symbol = format!("__atrox_{}", name);

        // Check that this is part of our own source
        assert!(
            self.own_module
                .exports()
                .any(|export| export.name() == symbol && export.ty().func().is_some()),
            "Symbol {} not found in WASM binary. Did you build it yet?",
            symbol
        );

        // Get the equivalent symbol
        DynFn {
            symbol,
            source: self.own_bytecode.to_vec(),
            _phantomdata: Default::default(),
        }
    }

    pub fn call<Input: Serialize, Output: DeserializeOwned>(
        &self,
        f: &DynFn<Input, Output>,
        input: &Input,
    ) -> Output {
        let input_bytes = bincode::serialize(&input);
        todo!()
    }
}

//impl<Input, Output> DynFn<Input, Output> {}
