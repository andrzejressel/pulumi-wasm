pub mod get_traffic_manager_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTrafficManagerProfileArgs {
        /// Specifies the name of the Traffic Manager Profile.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the resource group the Traffic Manager Profile is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Indicates whether Traffic View is enabled for the Traffic Manager profile.
        #[builder(into, default)]
        pub traffic_view_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetTrafficManagerProfileResult {
        /// This block specifies the DNS configuration of the Profile.
        pub dns_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetTrafficManagerProfileDnsConfig>,
        >,
        /// The FQDN of the created Profile.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// This block specifies the Endpoint monitoring configuration for the Profile.
        pub monitor_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetTrafficManagerProfileMonitorConfig,
            >,
        >,
        /// The name of the custom header.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The status of the profile.
        pub profile_status: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the algorithm used to route traffic.
        pub traffic_routing_method: pulumi_wasm_rust::Output<String>,
        /// Indicates whether Traffic View is enabled for the Traffic Manager profile.
        pub traffic_view_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetTrafficManagerProfileArgs) -> GetTrafficManagerProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let traffic_view_enabled_binding = args.traffic_view_enabled.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getTrafficManagerProfile:getTrafficManagerProfile"
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
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "trafficViewEnabled".into(),
                    value: &traffic_view_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dnsConfigs".into(),
                },
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "monitorConfigs".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "profileStatus".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "trafficRoutingMethod".into(),
                },
                register_interface::ResultField {
                    name: "trafficViewEnabled".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTrafficManagerProfileResult {
            dns_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsConfigs").unwrap(),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            monitor_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitorConfigs").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            profile_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("profileStatus").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            traffic_routing_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficRoutingMethod").unwrap(),
            ),
            traffic_view_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficViewEnabled").unwrap(),
            ),
        }
    }
}
