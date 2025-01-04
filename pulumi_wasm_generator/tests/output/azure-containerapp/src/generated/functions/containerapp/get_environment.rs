pub mod get_environment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEnvironmentArgs {
        /// The name of the Container Apps Managed Environment.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where this Container App Environment exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetEnvironmentResult {
        /// The ID of the Custom Domain Verification for this Container App Environment.
        pub custom_domain_verification_id: pulumi_wasm_rust::Output<String>,
        /// The default publicly resolvable name of this Container App Environment. This is generated at creation time to be globally unique.
        pub default_domain: pulumi_wasm_rust::Output<String>,
        /// The network addressing in which the Container Apps in this Container App Environment will reside in CIDR notation.
        pub docker_bridge_cidr: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Subnet in use by the Container Apps Control Plane.
        pub infrastructure_subnet_id: pulumi_wasm_rust::Output<String>,
        /// Does the Container App Environment operate in Internal Load Balancing Mode?
        pub internal_load_balancer_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Azure Location where this Container App Environment exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Log Analytics Workspace this Container Apps Managed Environment is linked to.
        pub log_analytics_workspace_name: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The IP range, in CIDR notation, that is reserved for environment infrastructure IP addresses.
        pub platform_reserved_cidr: pulumi_wasm_rust::Output<String>,
        /// The IP address from the IP range defined by `platform_reserved_cidr` that is reserved for the internal DNS server.
        pub platform_reserved_dns_ip_address: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Static IP address of the Environment.
        pub static_ip_address: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetEnvironmentArgs) -> GetEnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:containerapp/getEnvironment:getEnvironment".into(),
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
                    name: "customDomainVerificationId".into(),
                },
                register_interface::ResultField {
                    name: "defaultDomain".into(),
                },
                register_interface::ResultField {
                    name: "dockerBridgeCidr".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "infrastructureSubnetId".into(),
                },
                register_interface::ResultField {
                    name: "internalLoadBalancerEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "logAnalyticsWorkspaceName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "platformReservedCidr".into(),
                },
                register_interface::ResultField {
                    name: "platformReservedDnsIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "staticIpAddress".into(),
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
        GetEnvironmentResult {
            custom_domain_verification_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDomainVerificationId").unwrap(),
            ),
            default_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultDomain").unwrap(),
            ),
            docker_bridge_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dockerBridgeCidr").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            infrastructure_subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("infrastructureSubnetId").unwrap(),
            ),
            internal_load_balancer_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internalLoadBalancerEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            log_analytics_workspace_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logAnalyticsWorkspaceName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            platform_reserved_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformReservedCidr").unwrap(),
            ),
            platform_reserved_dns_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformReservedDnsIpAddress").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            static_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("staticIpAddress").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
