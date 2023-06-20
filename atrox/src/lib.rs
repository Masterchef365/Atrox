pub use atrox_derive::generate_function;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static __RESERVED_MEMORY: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![]));
pub use bincode;
