pub mod get_endpoint_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEndpointConnectionArgs {
        /// Specifies the Name of the private endpoint.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the Name of the Resource Group within which the private endpoint exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetEndpointConnectionResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The supported Azure location where the resource exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the private endpoint.
        pub name: pulumi_wasm_rust::Output<String>,
        pub network_interfaces: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::privatelink::GetEndpointConnectionNetworkInterface,
            >,
        >,
        pub private_service_connections: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::privatelink::GetEndpointConnectionPrivateServiceConnection,
            >,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetEndpointConnectionArgs) -> GetEndpointConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:privatelink/getEndpointConnection:getEndpointConnection"
                .into(),
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
                    name: "networkInterfaces".into(),
                },
                register_interface::ResultField {
                    name: "privateServiceConnections".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetEndpointConnectionResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_interfaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaces").unwrap(),
            ),
            private_service_connections: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateServiceConnections").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}