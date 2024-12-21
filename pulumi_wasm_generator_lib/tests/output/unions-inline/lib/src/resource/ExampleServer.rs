
#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ExampleServerArgs {
    #[builder(into, default)]
    pub properties: pulumi_wasm_rust::Output<Option<pulumi_wasm_provider_common::OneOf2<crate::types::ServerPropertiesForReplica, crate::types::ServerPropertiesForRestore>>>,
}

pub struct ExampleServerResult {
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ExampleServerArgs) -> ExampleServerResult {

    let result = crate::bindings::pulumi::example::ExampleServer::invoke(name, &crate::bindings::pulumi::example::ExampleServer::Args {
        properties: &args.properties.get_inner(),
    });

    ExampleServerResult {
        name: crate::into_domain(result.name),
    }
}
