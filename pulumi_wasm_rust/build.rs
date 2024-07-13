use quote::{format_ident, quote};
use std::path::Path;
use std::{env, fs};
use syn::Ident;

fn main() {
    let letter_range: Vec<char> = ('a'..='z').collect();
    let mut items: Vec<syn::Item> = Vec::new();

    for i in 2..=letter_range.len() {
        let current_letters: Vec<&char> = letter_range.iter().take(i).collect();
        let current_letters_upper: Vec<char> = current_letters
            .iter()
            .map(|c| c.to_ascii_uppercase())
            .collect();
        let idents: Vec<Ident> = current_letters
            .iter()
            .map(|c| format_ident!("{}", c))
            .collect();
        let idents_generic: Vec<Ident> = current_letters_upper
            .iter()
            .map(|c| format_ident!("{}", c))
            .collect();

        let function_name = format_ident!("combine{}", i);

        let output = quote! {
            impl<#(#idents_generic),*> Output<(#(#idents_generic),*)> {
                pub fn #function_name(#(#idents: Output<#idents_generic>),*) -> Self {
                    let output_id = output_interface::combine(vec![#(#idents.future),*]);
                    Output {
                        phantom: PhantomData,
                        future: output_id,
                    }
                }
            }
        };

        let syntax_tree = syn::parse2::<syn::ItemImpl>(output).unwrap();
        items.push(syntax_tree.into());
    }

    let file = syn::File {
        attrs: vec![],
        items,
        shebang: None,
    };
    let formatted = prettyplease::unparse(&file);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("outputs.rs");
    fs::write(dest_path, formatted).unwrap();
}
