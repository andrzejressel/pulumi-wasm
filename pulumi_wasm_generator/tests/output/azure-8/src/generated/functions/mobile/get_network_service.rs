pub mod get_network_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkServiceArgs {
        /// Specifies the ID of the Mobile Network Service.
        #[builder(into)]
        pub mobile_network_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name which should be used for this Mobile Network Service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkServiceResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Mobile Network Service should exist.
        pub location: pulumi_wasm_rust::Output<String>,
        pub mobile_network_id: pulumi_wasm_rust::Output<String>,
        /// The name of the data flow template. This must be unique within the parent data flow policy rule.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `pcc_rule` block as defined below. The set of PCC Rules that make up this service.
        pub pcc_rules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mobile::GetNetworkServicePccRule>,
        >,
        /// A precedence value that is used to decide between services when identifying the QoS values to use for a particular SIM. A lower value means a higher priority.
        pub service_precedence: pulumi_wasm_rust::Output<i32>,
        /// A `service_qos_policy` block as defined below. The QoS policy to use for packets matching this service.
        pub service_qos_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mobile::GetNetworkServiceServiceQosPolicy>,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Service.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetNetworkServiceArgs,
    ) -> GetNetworkServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let mobile_network_id_binding = args
            .mobile_network_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:mobile/getNetworkService:getNetworkService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mobileNetworkId".into(),
                    value: &mobile_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mobileNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pccRules".into(),
                },
                register_interface::ResultField {
                    name: "servicePrecedence".into(),
                },
                register_interface::ResultField {
                    name: "serviceQosPolicies".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNetworkServiceResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            mobile_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mobileNetworkId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pcc_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pccRules").unwrap(),
            ),
            service_precedence: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicePrecedence").unwrap(),
            ),
            service_qos_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceQosPolicies").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
