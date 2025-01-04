pub mod get_service_endpoint_connections {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceEndpointConnectionsArgs {
        /// The name of the resource group in which the private link service resides.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the private link service.
        #[builder(into)]
        pub service_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceEndpointConnectionsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub private_endpoint_connections: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::privatelink::GetServiceEndpointConnectionsPrivateEndpointConnection,
            >,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub service_id: pulumi_wasm_rust::Output<String>,
        /// The name of the private link service.
        pub service_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetServiceEndpointConnectionsArgs,
    ) -> GetServiceEndpointConnectionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let service_id_binding = args.service_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:privatelink/getServiceEndpointConnections:getServiceEndpointConnections"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceId".into(),
                    value: &service_id_binding,
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
                    name: "privateEndpointConnections".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serviceId".into(),
                },
                register_interface::ResultField {
                    name: "serviceName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServiceEndpointConnectionsResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            private_endpoint_connections: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateEndpointConnections").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceId").unwrap(),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceName").unwrap(),
            ),
        }
    }
}
