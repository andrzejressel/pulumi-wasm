
#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct typesystemServerArgs {
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub optional_string_array: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub optional_string_input: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub optional_union: pulumi_wasm_rust::Output<Option<pulumi_wasm_provider_common::OneOf2<crate::types::EnumCase1, crate::types::EnumCase2>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub properties_collection: pulumi_wasm_rust::Output<Option<Vec<pulumi_wasm_provider_common::OneOf2<crate::types::ServerPropertiesForReplica, crate::types::ServerPropertiesForRestore>>>>,
    #[builder(into)]
    pub required_string_array: pulumi_wasm_rust::Output<Vec<String>>,
    #[builder(into)]
    pub required_string_input: pulumi_wasm_rust::Output<String>,
    #[builder(into)]
    pub required_union: pulumi_wasm_rust::Output<pulumi_wasm_provider_common::OneOf2<crate::types::EnumCase1, crate::types::EnumCase2>>,
}

pub struct typesystemServerResult {
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: typesystemServerArgs) -> typesystemServerResult {

    let result = crate::bindings::pulumi::typesystem::typesystem_server::invoke(name, &crate::bindings::pulumi::typesystem::typesystem_server::Args {
        optional_string_array: &args.optional_string_array.get_inner(),
        optional_string_input: &args.optional_string_input.get_inner(),
        optional_union: &args.optional_union.get_inner(),
        properties_collection: &args.properties_collection.get_inner(),
        required_string_array: &args.required_string_array.get_inner(),
        required_string_input: &args.required_string_input.get_inner(),
        required_union: &args.required_union.get_inner(),
    });

    typesystemServerResult {
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]pub enum InputOptionalUnion {    El0(Box<crate::types::EnumCase1>),    El1(Box<crate::types::EnumCase2>),}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]pub enum InputPropertiesCollection {    El0(Box<crate::types::ServerPropertiesForReplica>),    El1(Box<crate::types::ServerPropertiesForRestore>),}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]pub enum InputRequiredUnion {    El0(Box<crate::types::EnumCase1>),    El1(Box<crate::types::EnumCase2>),}

