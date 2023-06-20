use std::{collections::HashMap, marker::PhantomData};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use wasmtime::{Instance, Module, Store};

pub struct Runtime {
    own_bytecode: &'static [u8],
    wasm: wasmtime::Engine,
    own_module: wasmtime::Module,
}

#[derive(Serialize, Deserialize)]
pub struct DynFn<Input, Output> {
    bytecode: Vec<u8>,
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
            bytecode: self.own_bytecode.to_vec(),
            _phantomdata: Default::default(),
        }
    }

    pub fn call<Input: Serialize, Output: DeserializeOwned>(
        &self,
        f: &DynFn<Input, Output>,
        input: &Input,
    ) -> Output {
        // Create wasm module (TODO: Cache this!)
        let module = Module::new(&self.wasm, &f.bytecode).unwrap();
        let mut store = Store::new(&self.wasm, ());
        let instance = Instance::new(&mut store, &module, &[]).unwrap();

        // Find the corresponding symbol
        let func = instance
            .get_typed_func::<u32, u32>(&mut store, &f.symbol)
            .unwrap();

        // Serialize input
        let input_bytes = bincode::serialize(&input).unwrap();

        // Then reserve some space for the input
        let input_len = input_bytes.len() as u32;
        let mem_in_ptr = func.call(&mut store, input_len).unwrap();

        // Then write the serialized input into WASM
        let mem = instance.get_memory(&mut store, "memory").unwrap();
        mem.data_mut(&mut store)[mem_in_ptr as usize..][..input_len as usize]
            .copy_from_slice(&input_bytes);

        // Run the function (with the special u32::MAX for input size to tell it to call instead
        // of allocating)
        let mem_out_ptr = func.call(&mut store, u32::MAX).unwrap();

        // Read the header to know how long the data is
        let mut header_bytes = [0; 4];
        header_bytes.copy_from_slice(&mem.data(&mut store)[mem_out_ptr as usize..][..4]);
        let output_len = u32::from_le_bytes(header_bytes);

        // Read the rest of the content
        let output_bytes = &mem.data(&mut store)[mem_out_ptr as usize + 4..][..output_len as usize];
        bincode::deserialize(output_bytes).unwrap()
    }
}

//impl<Input, Output> DynFn<Input, Output> {}
