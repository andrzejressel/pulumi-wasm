pub mod get_express_route_circuit {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetExpressRouteCircuitArgs {
        /// The name of the ExpressRoute circuit.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the ExpressRoute circuit exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetExpressRouteCircuitResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure location where the ExpressRoute circuit exists
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `peerings` block for the ExpressRoute circuit as documented below
        pub peerings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetExpressRouteCircuitPeering>,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The string needed by the service provider to provision the ExpressRoute circuit.
        pub service_key: pulumi_wasm_rust::Output<String>,
        /// A `service_provider_properties` block for the ExpressRoute circuit as documented below
        pub service_provider_properties: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetExpressRouteCircuitServiceProviderProperty,
            >,
        >,
        /// The ExpressRoute circuit provisioning state from your chosen service provider. Possible values are `NotProvisioned`, `Provisioning`, `Provisioned`, and `Deprovisioning`.
        pub service_provider_provisioning_state: pulumi_wasm_rust::Output<String>,
        /// A `sku` block for the ExpressRoute circuit as documented below.
        pub sku: pulumi_wasm_rust::Output<
            super::super::super::types::network::GetExpressRouteCircuitSku,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetExpressRouteCircuitArgs,
    ) -> GetExpressRouteCircuitResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getExpressRouteCircuit:getExpressRouteCircuit".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
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
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "peerings".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serviceKey".into(),
                },
                register_interface::ResultField {
                    name: "serviceProviderProperties".into(),
                },
                register_interface::ResultField {
                    name: "serviceProviderProvisioningState".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetExpressRouteCircuitResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            peerings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerings").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            service_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceKey").unwrap(),
            ),
            service_provider_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceProviderProperties").unwrap(),
            ),
            service_provider_provisioning_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceProviderProvisioningState").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(hashmap.remove("sku").unwrap()),
        }
    }
}
