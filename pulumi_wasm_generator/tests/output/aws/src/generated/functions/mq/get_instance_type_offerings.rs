pub mod get_instance_type_offerings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceTypeOfferingsArgs {
        /// Filter response by engine type.
        #[builder(into, default)]
        pub engine_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Filter response by host instance type.
        #[builder(into, default)]
        pub host_instance_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Filter response by storage type.
        #[builder(into, default)]
        pub storage_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceTypeOfferingsResult {
        /// Option for host instance type. See Broker Instance Options below.
        pub broker_instance_options: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::mq::GetInstanceTypeOfferingsBrokerInstanceOption,
            >,
        >,
        /// Broker's engine type.
        pub engine_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Broker's instance type.
        pub host_instance_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Broker's storage type.
        pub storage_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetInstanceTypeOfferingsArgs) -> GetInstanceTypeOfferingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let engine_type_binding = args.engine_type.get_inner();
        let host_instance_type_binding = args.host_instance_type.get_inner();
        let storage_type_binding = args.storage_type.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:mq/getInstanceTypeOfferings:getInstanceTypeOfferings".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "engineType".into(),
                    value: &engine_type_binding,
                },
                register_interface::ObjectField {
                    name: "hostInstanceType".into(),
                    value: &host_instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "storageType".into(),
                    value: &storage_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "brokerInstanceOptions".into(),
                },
                register_interface::ResultField {
                    name: "engineType".into(),
                },
                register_interface::ResultField {
                    name: "hostInstanceType".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "storageType".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetInstanceTypeOfferingsResult {
            broker_instance_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("brokerInstanceOptions").unwrap(),
            ),
            engine_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineType").unwrap(),
            ),
            host_instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostInstanceType").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            storage_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageType").unwrap(),
            ),
        }
    }
}