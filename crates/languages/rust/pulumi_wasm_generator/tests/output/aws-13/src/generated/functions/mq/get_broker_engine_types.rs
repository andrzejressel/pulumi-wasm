pub mod get_broker_engine_types {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBrokerEngineTypesArgs {
        /// The MQ engine type to return version details for.
        #[builder(into, default)]
        pub engine_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetBrokerEngineTypesArgs,
    ) -> GetBrokerEngineTypesResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let engine_type_binding = args.engine_type.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:mq/getBrokerEngineTypes:getBrokerEngineTypes".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "engineType".into(),
                    value: &engine_type_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBrokerEngineTypesResult {
            broker_engine_types: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("brokerEngineTypes"),
            ),
            engine_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("engineType"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
