pub(crate) mod model;
#[cfg(test)]
pub(crate) mod tests;
pub(crate) mod yaml_model;

fn reformat_code(code: &str) -> String {
    let syntax_tree = syn::parse_file(code).unwrap();
    prettyplease::unparse(&syntax_tree)
}
