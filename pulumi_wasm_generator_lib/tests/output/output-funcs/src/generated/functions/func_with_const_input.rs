#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FuncWithConstInputArgs {
    #[builder(into, default)]
    pub plain_input: pulumi_wasm_rust::Output<
        Option<super::super::constants::ConstStringFixed>,
    >,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: FuncWithConstInputArgs) {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let plain_input_binding = args.plain_input.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "mypkg::funcWithConstInput".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "plainInput".into(),
                value: &plain_input_binding,
            },
        ]),
        results: vec![],
    };
    register_interface::invoke(&request);
}
