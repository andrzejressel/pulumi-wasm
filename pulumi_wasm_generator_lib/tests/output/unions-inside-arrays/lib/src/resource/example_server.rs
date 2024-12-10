
#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ExampleServerArgs {
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub properties_collection: pulumi_wasm_rust::Output<Option<Vec<pulumi_wasm_provider_common::OneOf2<crate::types::ServerPropertiesForRestore, crate::types::ServerPropertiesForReplica>>>>,
}

pub struct ExampleServerResult {
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ExampleServerArgs) -> ExampleServerResult {

    let result = crate::bindings::pulumi::example::example_server::invoke(name, &crate::bindings::pulumi::example::example_server::Args {
        properties_collection: &args.properties_collection.get_inner(),
    });

    ExampleServerResult {
        name: crate::into_domain(result.name),
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]pub enum InputPropertiesCollection {    El0(Box<crate::types::ServerPropertiesForRestore>),    El1(Box<crate::types::ServerPropertiesForReplica>),}

