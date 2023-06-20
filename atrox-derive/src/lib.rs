use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn generate_function(_: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input token stream into a function item
    let item_fn = parse_macro_input!(item as ItemFn);

    // Extract the function signature and name
    let fn_signature = &item_fn.sig;
    let fn_name = &fn_signature.ident;

    // Generate the new function name by appending "_generated" to the original name
    let generated_fn_name = syn::Ident::new(&format!("__atrox_{}", fn_name), fn_name.span());

    // Generate the new function item with the generated name and body
    let generated_fn_item = quote! {
        #item_fn

        #[cfg(target_arch = "wasm32")]
        #[no_mangle]
        fn #generated_fn_name(size: u32) -> *mut u8 {
            let mut mem = atrox::__RESERVED_MEMORY.lock().unwrap();

            if size == u32::MAX {
                // Call function underneath
                let input_val = atrox::bincode::deserialize(mem.as_slice())
                    .expect("Failed to decode function args");

                let output_val = #fn_name(input_val);

                *mem = atrox::bincode::serialize(&output_val)
                    .expect("Failed to encode function result");
            } else {
                // Allocate and return without calling the wrapped function
                *mem = vec![0; size as usize];
            }

            mem.as_mut_ptr()
        }
    };

    // Return the generated function as a TokenStream
    generated_fn_item.into()
}
