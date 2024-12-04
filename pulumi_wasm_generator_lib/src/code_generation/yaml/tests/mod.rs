pub(crate) mod complex_yaml;
pub(crate) mod example_array;
pub(crate) mod example_empty_properties;
pub(crate) mod example_escape_string;
pub(crate) mod example_numbers;
pub(crate) mod example_interpolation;
pub(crate) mod example_variables;

fn reformat_code(code: &str) -> String {
    let syntax_tree = syn::parse_file(code).unwrap();
    prettyplease::unparse(&syntax_tree)
}
