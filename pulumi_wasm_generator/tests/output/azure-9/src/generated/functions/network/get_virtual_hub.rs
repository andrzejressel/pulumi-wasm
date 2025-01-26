pub mod get_virtual_hub {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualHubArgs {
        /// The name of the Virtual Hub.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the Virtual Hub exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualHubResult {
        /// The Address Prefix used for this Virtual Hub.
        pub address_prefix: pulumi_wasm_rust::Output<String>,
        /// The ID of the default Route Table in the Virtual Hub.
        pub default_route_table_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Virtual Hub exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Virtual Hub.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The Autonomous System Number of the Virtual Hub BGP router.
        pub virtual_router_asn: pulumi_wasm_rust::Output<i32>,
        /// The IP addresses of the Virtual Hub BGP router.
        pub virtual_router_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the Virtual WAN within which the Virtual Hub exists.
        pub virtual_wan_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetVirtualHubArgs,
    ) -> GetVirtualHubResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getVirtualHub:getVirtualHub".into(),
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
                    name: "addressPrefix".into(),
                },
                register_interface::ResultField {
                    name: "defaultRouteTableId".into(),
                },
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
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "virtualRouterAsn".into(),
                },
                register_interface::ResultField {
                    name: "virtualRouterIps".into(),
                },
                register_interface::ResultField {
                    name: "virtualWanId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVirtualHubResult {
            address_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressPrefix").unwrap(),
            ),
            default_route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultRouteTableId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            virtual_router_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualRouterAsn").unwrap(),
            ),
            virtual_router_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualRouterIps").unwrap(),
            ),
            virtual_wan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualWanId").unwrap(),
            ),
        }
    }
}
