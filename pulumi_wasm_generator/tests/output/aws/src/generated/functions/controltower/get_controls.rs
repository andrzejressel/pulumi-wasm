pub mod get_controls {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetControlsArgs {
        /// The ARN of the organizational unit.
        #[builder(into)]
        pub target_identifier: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetControlsResult {
        /// List of all the ARNs for the controls applied to the `target_identifier`.
        pub enabled_controls: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub target_identifier: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetControlsArgs) -> GetControlsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let target_identifier_binding = args.target_identifier.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:controltower/getControls:getControls".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "targetIdentifier".into(),
                    value: &target_identifier_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enabledControls".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "targetIdentifier".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetControlsResult {
            enabled_controls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabledControls").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            target_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetIdentifier").unwrap(),
            ),
        }
    }
}
