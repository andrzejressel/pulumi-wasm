pub mod get_environment_v_3 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEnvironmentV3Args {
        /// The name of this v3 App Service Environment.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the v3 App Service Environment exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetEnvironmentV3Result {
        /// Are new Private Endpoint Connections allowed.
        pub allow_new_private_endpoint_connections: pulumi_wasm_rust::Output<bool>,
        /// A `cluster_setting` block as defined below.
        pub cluster_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetEnvironmentV3ClusterSetting>,
        >,
        /// The number of Dedicated Hosts used by this ASEv3.
        pub dedicated_host_count: pulumi_wasm_rust::Output<i32>,
        /// the DNS suffix for this App Service Environment V3.
        pub dns_suffix: pulumi_wasm_rust::Output<String>,
        /// The external inbound IP addresses of the App Service Environment V3.
        pub external_inbound_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An Inbound Network Dependencies block as defined below.
        pub inbound_network_dependencies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::appservice::GetEnvironmentV3InboundNetworkDependency,
            >,
        >,
        /// The internal inbound IP addresses of the App Service Environment V3.
        pub internal_inbound_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Internal Load Balancing Mode of this ASEv3.
        pub internal_load_balancing_mode: pulumi_wasm_rust::Output<String>,
        /// The number of IP SSL addresses reserved for the App Service Environment V3.
        pub ip_ssl_address_count: pulumi_wasm_rust::Output<i32>,
        /// The list of Outbound IP Addresses of Linux based Apps in this App Service Environment V3.
        pub linux_outbound_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The location where the App Service Environment exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Cluster Setting.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Pricing tier for the front end instances.
        pub pricing_tier: pulumi_wasm_rust::Output<String>,
        pub remote_debugging_enabled: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the v3 App Service Environment Subnet.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the v3 App Service Environment.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Outbound addresses of Windows based Apps in this App Service Environment V3.
        pub windows_outbound_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        pub zone_redundant: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetEnvironmentV3Args,
    ) -> GetEnvironmentV3Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appservice/getEnvironmentV3:getEnvironmentV3".into(),
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
                    name: "allowNewPrivateEndpointConnections".into(),
                },
                register_interface::ResultField {
                    name: "clusterSettings".into(),
                },
                register_interface::ResultField {
                    name: "dedicatedHostCount".into(),
                },
                register_interface::ResultField {
                    name: "dnsSuffix".into(),
                },
                register_interface::ResultField {
                    name: "externalInboundIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "inboundNetworkDependencies".into(),
                },
                register_interface::ResultField {
                    name: "internalInboundIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "internalLoadBalancingMode".into(),
                },
                register_interface::ResultField {
                    name: "ipSslAddressCount".into(),
                },
                register_interface::ResultField {
                    name: "linuxOutboundIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pricingTier".into(),
                },
                register_interface::ResultField {
                    name: "remoteDebuggingEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "windowsOutboundIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "zoneRedundant".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetEnvironmentV3Result {
            allow_new_private_endpoint_connections: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowNewPrivateEndpointConnections").unwrap(),
            ),
            cluster_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterSettings").unwrap(),
            ),
            dedicated_host_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dedicatedHostCount").unwrap(),
            ),
            dns_suffix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsSuffix").unwrap(),
            ),
            external_inbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalInboundIpAddresses").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            inbound_network_dependencies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inboundNetworkDependencies").unwrap(),
            ),
            internal_inbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internalInboundIpAddresses").unwrap(),
            ),
            internal_load_balancing_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internalLoadBalancingMode").unwrap(),
            ),
            ip_ssl_address_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipSslAddressCount").unwrap(),
            ),
            linux_outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linuxOutboundIpAddresses").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pricing_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pricingTier").unwrap(),
            ),
            remote_debugging_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteDebuggingEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            windows_outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("windowsOutboundIpAddresses").unwrap(),
            ),
            zone_redundant: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneRedundant").unwrap(),
            ),
        }
    }
}
