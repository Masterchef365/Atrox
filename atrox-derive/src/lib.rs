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
    let generated_fn_name = syn::Ident::new(&format!("{}_generated", fn_name), fn_name.span());

    // Generate the function body
    let generated_fn_body = quote! {
        // Add your custom generated code here
        // This is just an example that prints a message
        println!("Generated function called!");

        // Call the original function
        #fn_name()
    };

    // Generate the new function item with the generated name and body
    let generated_fn_item = quote! {
        #[allow(non_snake_case)]
        #item_fn

        fn #generated_fn_name() {
            #generated_fn_body
        }
    };

    // Return the generated function as a TokenStream
    generated_fn_item.into()
}
