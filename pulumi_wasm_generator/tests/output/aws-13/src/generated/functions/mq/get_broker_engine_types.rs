pub mod get_broker_engine_types {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBrokerEngineTypesArgs {
        /// The MQ engine type to return version details for.
        #[builder(into, default)]
        pub engine_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBrokerEngineTypesResult {
        /// A list of available engine types and versions. See Engine Types.
        pub broker_engine_types: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mq::GetBrokerEngineTypesBrokerEngineType>,
        >,
        /// The broker's engine type.
        pub engine_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetBrokerEngineTypesArgs) -> GetBrokerEngineTypesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let engine_type_binding = args.engine_type.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:mq/getBrokerEngineTypes:getBrokerEngineTypes".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "engineType".into(),
                    value: &engine_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "brokerEngineTypes".into(),
                },
                register_interface::ResultField {
                    name: "engineType".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBrokerEngineTypesResult {
            broker_engine_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("brokerEngineTypes").unwrap(),
            ),
            engine_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineType").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}
