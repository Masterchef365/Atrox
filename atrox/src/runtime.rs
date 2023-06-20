/*
struct Runtime {

}

impl Runtime {
    pub fn new() {
    }
}
*/

use std::marker::PhantomData;

pub struct DynFn<Input, Output> {
    _phantondata: PhantomData<(Input, Output)>,
}

impl<Input, Output> DynFn<Input, Output> {
    pub fn from_fn<F: Fn(Input) -> Output>(f: F) -> Self {
        let name = std::any::type_name::<F>();
        dbg!(name);
        Self { _phantondata: Default::default() }
    }
}
