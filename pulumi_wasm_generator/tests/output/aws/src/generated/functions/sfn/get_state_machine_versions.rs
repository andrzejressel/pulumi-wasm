pub mod get_state_machine_versions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStateMachineVersionsArgs {
        /// ARN of the State Machine.
        #[builder(into)]
        pub statemachine_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetStateMachineVersionsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub statemachine_arn: pulumi_wasm_rust::Output<String>,
        /// ARN List identifying the statemachine versions.
        pub statemachine_versions: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetStateMachineVersionsArgs) -> GetStateMachineVersionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let statemachine_arn_binding = args.statemachine_arn.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:sfn/getStateMachineVersions:getStateMachineVersions".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "statemachineArn".into(),
                    value: &statemachine_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "statemachineArn".into(),
                },
                register_interface::ResultField {
                    name: "statemachineVersions".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetStateMachineVersionsResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            statemachine_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statemachineArn").unwrap(),
            ),
            statemachine_versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statemachineVersions").unwrap(),
            ),
        }
    }
}