
#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ExampleServerArgs {
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub properties: pulumi_wasm_rust::Output<Option<inputproperties>>,
}

pub struct ExampleServerResult {
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ExampleServerArgs) -> ExampleServerResult {

    let result = crate::bindings::pulumi::example::example_server::invoke(name, &crate::bindings::pulumi::example::example_server::Args {
        properties: &args.properties.get_inner(),
    });

    ExampleServerResult {
        name: crate::into_domain(result.name),
    }
}
