#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FuncWithEmptyOutputsArgs {
    /// The Name of the FeatureGroup.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn invoke(args: FuncWithEmptyOutputsArgs) {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let name_binding = args.name.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "mypkg::funcWithEmptyOutputs".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
        ]),
        results: vec![],
    };
    register_interface::invoke(&request);
}
