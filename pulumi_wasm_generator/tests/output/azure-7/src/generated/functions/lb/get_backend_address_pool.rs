pub mod get_backend_address_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBackendAddressPoolArgs {
        /// The ID of the Load Balancer in which the Backend Address Pool exists.
        #[builder(into)]
        pub loadbalancer_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Backend Address Pool.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetBackendAddressPoolResult {
        /// A list of `backend_address` block as defined below.
        pub backend_addresses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::lb::GetBackendAddressPoolBackendAddress>,
        >,
        /// A list of references to IP addresses defined in network interfaces.
        pub backend_ip_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::lb::GetBackendAddressPoolBackendIpConfiguration,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of the Load Balancing Inbound NAT Rules associated with this Backend Address Pool.
        pub inbound_nat_rules: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of the Load Balancing Rules associated with this Backend Address Pool.
        pub load_balancing_rules: pulumi_wasm_rust::Output<Vec<String>>,
        pub loadbalancer_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Backend Address.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of the Load Balancing Outbound Rules associated with this Backend Address Pool.
        pub outbound_rules: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetBackendAddressPoolArgs,
    ) -> GetBackendAddressPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let loadbalancer_id_binding = args
            .loadbalancer_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:lb/getBackendAddressPool:getBackendAddressPool".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "loadbalancerId".into(),
                    value: &loadbalancer_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backendAddresses".into(),
                },
                register_interface::ResultField {
                    name: "backendIpConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "inboundNatRules".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancingRules".into(),
                },
                register_interface::ResultField {
                    name: "loadbalancerId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outboundRules".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBackendAddressPoolResult {
            backend_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendAddresses").unwrap(),
            ),
            backend_ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendIpConfigurations").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            inbound_nat_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inboundNatRules").unwrap(),
            ),
            load_balancing_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancingRules").unwrap(),
            ),
            loadbalancer_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadbalancerId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            outbound_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outboundRules").unwrap(),
            ),
        }
    }
}
