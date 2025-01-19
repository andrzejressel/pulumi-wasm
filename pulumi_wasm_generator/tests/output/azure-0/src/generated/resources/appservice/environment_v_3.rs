/// Manages a 3rd Generation (v3) App Service Environment.
///
/// ## Example Usage
///
/// This example provisions an App Service Environment V3. Additional examples of how to use the `azure.appservice.EnvironmentV3` resource can be found in the `./examples/app-service-environment-v3` directory within the GitHub Repository.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: exampleRG1
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       addressSpaces:
///         - 10.0.0.0/16
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///       delegations:
///         - name: Microsoft.Web.hostingEnvironments
///           serviceDelegation:
///             name: Microsoft.Web/hostingEnvironments
///             actions:
///               - Microsoft.Network/virtualNetworks/subnets/action
///   exampleEnvironmentV3:
///     type: azure:appservice:EnvironmentV3
///     name: example
///     properties:
///       name: example-asev3
///       resourceGroupName: ${example.name}
///       subnetId: ${exampleSubnet.id}
///       internalLoadBalancingMode: Web, Publishing
///       clusterSettings:
///         - name: DisableTls1.0
///           value: '1'
///         - name: InternalEncryption
///           value: 'true'
///         - name: FrontEndSSLCipherSuiteOrder
///           value: TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256
///       tags:
///         env: production
///         terraformed: 'true'
///   exampleServicePlan:
///     type: azure:appservice:ServicePlan
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       osType: Linux
///       skuName: I1v2
///       appServiceEnvironmentId: ${exampleEnvironmentV3.id}
/// ```
///
/// ## Import
///
/// A 3rd Generation (v3) App Service Environment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/environmentV3:EnvironmentV3 myAppServiceEnv /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myResourceGroup/providers/Microsoft.Web/hostingEnvironments/myAppServiceEnv
/// ```
///
pub mod environment_v_3 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentV3Args {
        /// Should new Private Endpoint Connections be allowed. Defaults to `true`.
        #[builder(into, default)]
        pub allow_new_private_endpoint_connections: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Zero or more `cluster_setting` blocks as defined below.
        #[builder(into, default)]
        pub cluster_settings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appservice::EnvironmentV3ClusterSetting>>,
        >,
        /// This ASEv3 should use dedicated Hosts. Possible values are `2`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub dedicated_host_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies which endpoints to serve internally in the Virtual Network for the App Service Environment. Possible values are `None` (for an External VIP Type), and `"Web, Publishing"` (for an Internal VIP Type). Defaults to `None`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub internal_load_balancing_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the App Service Environment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to enable remote debug. Defaults to `false`.
        #[builder(into, default)]
        pub remote_debugging_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Resource Group where the App Service Environment exists. Defaults to the Resource Group of the Subnet (specified by `subnet_id`). Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Subnet which the App Service Environment should be connected to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** a /24 or larger CIDR is required. Once associated with an ASE, this size cannot be changed.
        ///
        /// > **NOTE:** This Subnet requires a delegation to `Microsoft.Web/hostingEnvironments` as detailed in the example above.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Set to `true` to deploy the ASEv3 with availability zones supported. Zonal ASEs can be deployed in some regions, you can refer to [Availability Zone support for App Service Environments](https://docs.microsoft.com/azure/app-service/environment/zone-redundancy). You can only set either `dedicated_host_count` or `zone_redundant` but not both. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Setting this value will provision 2 Physical Hosts for your App Service Environment V3, this is done at additional cost, please be aware of the pricing commitment in the [General Availability Notes](https://techcommunity.microsoft.com/t5/apps-on-azure/announcing-app-service-environment-v3-ga/ba-p/2517990)
        #[builder(into, default)]
        pub zone_redundant: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentV3Result {
        /// Should new Private Endpoint Connections be allowed. Defaults to `true`.
        pub allow_new_private_endpoint_connections: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Zero or more `cluster_setting` blocks as defined below.
        pub cluster_settings: pulumi_wasm_rust::Output<
            Vec<super::super::types::appservice::EnvironmentV3ClusterSetting>,
        >,
        /// This ASEv3 should use dedicated Hosts. Possible values are `2`. Changing this forces a new resource to be created.
        pub dedicated_host_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// the DNS suffix for this App Service Environment V3.
        pub dns_suffix: pulumi_wasm_rust::Output<String>,
        /// The external inbound IP addresses of the App Service Environment V3.
        pub external_inbound_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// An `inbound_network_dependencies` block as defined below.
        pub inbound_network_dependencies: pulumi_wasm_rust::Output<
            Vec<super::super::types::appservice::EnvironmentV3InboundNetworkDependency>,
        >,
        /// The internal inbound IP addresses of the App Service Environment V3.
        pub internal_inbound_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies which endpoints to serve internally in the Virtual Network for the App Service Environment. Possible values are `None` (for an External VIP Type), and `"Web, Publishing"` (for an Internal VIP Type). Defaults to `None`. Changing this forces a new resource to be created.
        pub internal_load_balancing_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of IP SSL addresses reserved for the App Service Environment V3.
        pub ip_ssl_address_count: pulumi_wasm_rust::Output<i32>,
        /// Outbound addresses of Linux based Apps in this App Service Environment V3
        pub linux_outbound_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The location where the App Service Environment exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the App Service Environment. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Pricing tier for the front end instances.
        pub pricing_tier: pulumi_wasm_rust::Output<String>,
        /// Whether to enable remote debug. Defaults to `false`.
        pub remote_debugging_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Resource Group where the App Service Environment exists. Defaults to the Resource Group of the Subnet (specified by `subnet_id`). Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Subnet which the App Service Environment should be connected to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** a /24 or larger CIDR is required. Once associated with an ASE, this size cannot be changed.
        ///
        /// > **NOTE:** This Subnet requires a delegation to `Microsoft.Web/hostingEnvironments` as detailed in the example above.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Outbound addresses of Windows based Apps in this App Service Environment V3.
        pub windows_outbound_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Set to `true` to deploy the ASEv3 with availability zones supported. Zonal ASEs can be deployed in some regions, you can refer to [Availability Zone support for App Service Environments](https://docs.microsoft.com/azure/app-service/environment/zone-redundancy). You can only set either `dedicated_host_count` or `zone_redundant` but not both. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Setting this value will provision 2 Physical Hosts for your App Service Environment V3, this is done at additional cost, please be aware of the pricing commitment in the [General Availability Notes](https://techcommunity.microsoft.com/t5/apps-on-azure/announcing-app-service-environment-v3-ga/ba-p/2517990)
        pub zone_redundant: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EnvironmentV3Args) -> EnvironmentV3Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_new_private_endpoint_connections_binding = args
            .allow_new_private_endpoint_connections
            .get_inner();
        let cluster_settings_binding = args.cluster_settings.get_inner();
        let dedicated_host_count_binding = args.dedicated_host_count.get_inner();
        let internal_load_balancing_mode_binding = args
            .internal_load_balancing_mode
            .get_inner();
        let name_binding = args.name.get_inner();
        let remote_debugging_enabled_binding = args.remote_debugging_enabled.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let zone_redundant_binding = args.zone_redundant.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/environmentV3:EnvironmentV3".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowNewPrivateEndpointConnections".into(),
                    value: &allow_new_private_endpoint_connections_binding,
                },
                register_interface::ObjectField {
                    name: "clusterSettings".into(),
                    value: &cluster_settings_binding,
                },
                register_interface::ObjectField {
                    name: "dedicatedHostCount".into(),
                    value: &dedicated_host_count_binding,
                },
                register_interface::ObjectField {
                    name: "internalLoadBalancingMode".into(),
                    value: &internal_load_balancing_mode_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "remoteDebuggingEnabled".into(),
                    value: &remote_debugging_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "zoneRedundant".into(),
                    value: &zone_redundant_binding,
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EnvironmentV3Result {
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
