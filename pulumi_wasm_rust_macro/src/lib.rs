extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// Marks function to be executed by Pulumi Wasm Runner - basically makes it special `fn main()`
/// ```rust,no_run
/// use anyhow::{Context, Error, Result};
/// use pulumi_wasm_rust::pulumi_main;
/// use pulumi_wasm_providers_random::random_string::RandomStringArgs;
/// use pulumi_wasm_providers_random::random_string;
///
/// #[pulumi_main]
/// fn main() -> Result<()> {
///   let random_string = random_string::create(
///     "test",
///     RandomStringArgs::builder().length(32).build_struct()
///   );
///   Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn pulumi_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as syn::ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;
    let fn_output = &input_fn.sig.output;

    let expanded = quote! {

        fn #fn_name() #fn_output
            #fn_block

        #[export_name = "component:pulumi-wasm-external/pulumi-main@0.0.0-STABLE-DEV#main"]
        unsafe extern "C" fn __exported() {
            pulumi_wasm_rust::__private::runner::run(|| {
                #fn_name()
            }).unwrap();
        }
    };

    TokenStream::from(expanded)
}
