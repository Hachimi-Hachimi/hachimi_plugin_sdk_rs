use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn hachimi_plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Get the name of the annotated function
    let fn_name = &input_fn.sig.ident;

    let expanded = quote! {
        // Keep the original function
        #input_fn

        // Generate the hachimi_init function
        #[no_mangle]
        pub unsafe extern "C" fn hachimi_init(
            vtable: *mut hachimi_plugin_sdk::sys::Vtable,
            version: i32
        ) -> hachimi_plugin_sdk::sys::InitResult {
            if version < hachimi_plugin_sdk::sys::VERSION {
                return hachimi_plugin_sdk::sys::InitResult::Error;
            }

            #fn_name(hachimi_plugin_sdk::api::HachimiApi::new(vtable, version))
        }
    };

    TokenStream::from(expanded)
}