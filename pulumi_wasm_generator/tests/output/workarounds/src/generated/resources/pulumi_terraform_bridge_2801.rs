pub mod pulumi_terraform_bridge_2801 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PulumiTerraformBridge2801Args {
        /// Soak time after each batch gets drained.
        ///
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PulumiTerraformBridge2801Args) {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "example:index:PulumiTerraformBridge2801".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([]),
        };
        register_interface::register(&request);
    }
}
