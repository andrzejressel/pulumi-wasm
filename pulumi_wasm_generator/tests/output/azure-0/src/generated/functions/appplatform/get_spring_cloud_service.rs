pub mod get_spring_cloud_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSpringCloudServiceArgs {
        /// Specifies The name of the Spring Cloud Service resource.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Spring Cloud Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSpringCloudServiceResult {
        /// A `config_server_git_setting` block as defined below.
        pub config_server_git_settings: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::appplatform::GetSpringCloudServiceConfigServerGitSetting,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The location of Spring Cloud Service.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name to identify on the Git repository.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of the outbound Public IP Addresses used by this Spring Cloud Service.
        pub outbound_public_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of `required_network_traffic_rules` blocks as defined below.
        pub required_network_traffic_rules: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::appplatform::GetSpringCloudServiceRequiredNetworkTrafficRule,
            >,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to Spring Cloud Service.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSpringCloudServiceArgs) -> GetSpringCloudServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appplatform/getSpringCloudService:getSpringCloudService"
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
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configServerGitSettings".into(),
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
                    name: "outboundPublicIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "requiredNetworkTrafficRules".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSpringCloudServiceResult {
            config_server_git_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configServerGitSettings").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            outbound_public_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outboundPublicIpAddresses").unwrap(),
            ),
            required_network_traffic_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requiredNetworkTrafficRules").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
