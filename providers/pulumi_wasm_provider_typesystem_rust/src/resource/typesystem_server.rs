
#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TypesystemServerArgs {
    #[builder(into, default)]
    pub optional_string_array: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    #[builder(into, default)]
    pub optional_string_input: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default)]
    pub optional_union: pulumi_wasm_rust::Output<Option<pulumi_wasm_provider_common::OneOf2<crate::types::UnionCase1, crate::types::UnionCase2>>>,
    #[builder(into, default)]
    pub properties_collection: pulumi_wasm_rust::Output<Option<Vec<pulumi_wasm_provider_common::OneOf2<crate::types::UnionCase1, crate::types::UnionCase2>>>>,
    #[builder(into)]
    pub required_string_array: pulumi_wasm_rust::Output<Vec<String>>,
    #[builder(into)]
    pub required_string_input: pulumi_wasm_rust::Output<String>,
    #[builder(into)]
    pub required_union: pulumi_wasm_rust::Output<pulumi_wasm_provider_common::OneOf2<crate::types::UnionCase1, crate::types::UnionCase2>>,
}

pub struct TypesystemServerResult {
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: TypesystemServerArgs
) -> TypesystemServerResult {

    let result = crate::bindings::pulumi::typesystem::typesystem_server::invoke(
        name,
        &crate::bindings::pulumi::typesystem::typesystem_server::Args {
                optional_string_array: &args.optional_string_array.get_inner(),
                optional_string_input: &args.optional_string_input.get_inner(),
                optional_union: &args.optional_union.get_inner(),
                properties_collection: &args.properties_collection.get_inner(),
                required_string_array: &args.required_string_array.get_inner(),
                required_string_input: &args.required_string_input.get_inner(),
                required_union: &args.required_union.get_inner(),
        }
    );

    TypesystemServerResult {
    }
}
