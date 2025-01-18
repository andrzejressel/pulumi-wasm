pub mod get_virtual_hub_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualHubConnectionArgs {
        /// The name of the Connection which should be retrieved.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group where the Virtual Hub Connection exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Virtual Hub where this Connection exists.
        #[builder(into)]
        pub virtual_hub_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualHubConnectionResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Whether Internet Security is enabled to secure internet traffic on this connection
        pub internet_security_enabled: pulumi_wasm_rust::Output<bool>,
        /// The name which is used for this Static Route.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Network which the Virtual Hub is connected
        pub remote_virtual_network_id: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `routing` block as defined below.
        pub routings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetVirtualHubConnectionRouting>,
        >,
        /// The ID of the Virtual Hub within which this connection is created
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
        pub virtual_hub_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetVirtualHubConnectionArgs) -> GetVirtualHubConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let virtual_hub_name_binding = args.virtual_hub_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getVirtualHubConnection:getVirtualHubConnection"
                .into(),
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
                register_interface::ObjectField {
                    name: "virtualHubName".into(),
                    value: &virtual_hub_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "internetSecurityEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "remoteVirtualNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "routings".into(),
                },
                register_interface::ResultField {
                    name: "virtualHubId".into(),
                },
                register_interface::ResultField {
                    name: "virtualHubName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVirtualHubConnectionResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            internet_security_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internetSecurityEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            remote_virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteVirtualNetworkId").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            routings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routings").unwrap(),
            ),
            virtual_hub_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualHubId").unwrap(),
            ),
            virtual_hub_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualHubName").unwrap(),
            ),
        }
    }
}
