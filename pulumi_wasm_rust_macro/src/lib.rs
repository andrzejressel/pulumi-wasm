extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn pulumi_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as syn::ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;
    let fn_output = &input_fn.sig.output;

    let expanded = quote! {

        fn #fn_name() #fn_output
            #fn_block

        #[export_name = "component:pulumi-wasm/pulumi-main@0.0.0#main"]
        unsafe extern "C" fn __exported() {
            pulumi_wasm_rust::runner::run(|| {
                #fn_name()
            }).unwrap();
        }
    };

    TokenStream::from(expanded)
}
