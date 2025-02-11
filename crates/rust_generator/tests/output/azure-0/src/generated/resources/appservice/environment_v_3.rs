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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment_v_3 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentV3Args {
        /// Should new Private Endpoint Connections be allowed. Defaults to `true`.
        #[builder(into, default)]
        pub allow_new_private_endpoint_connections: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Zero or more `cluster_setting` blocks as defined below.
        #[builder(into, default)]
        pub cluster_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appservice::EnvironmentV3ClusterSetting>>,
        >,
        /// This ASEv3 should use dedicated Hosts. Possible values are `2`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub dedicated_host_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies which endpoints to serve internally in the Virtual Network for the App Service Environment. Possible values are `None` (for an External VIP Type), and `"Web, Publishing"` (for an Internal VIP Type). Defaults to `None`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub internal_load_balancing_mode: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the App Service Environment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to enable remote debug. Defaults to `false`.
        #[builder(into, default)]
        pub remote_debugging_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Resource Group where the App Service Environment exists. Defaults to the Resource Group of the Subnet (specified by `subnet_id`). Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Subnet which the App Service Environment should be connected to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** a /24 or larger CIDR is required. Once associated with an ASE, this size cannot be changed.
        ///
        /// > **NOTE:** This Subnet requires a delegation to `Microsoft.Web/hostingEnvironments` as detailed in the example above.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Set to `true` to deploy the ASEv3 with availability zones supported. Zonal ASEs can be deployed in some regions, you can refer to [Availability Zone support for App Service Environments](https://docs.microsoft.com/azure/app-service/environment/zone-redundancy). You can only set either `dedicated_host_count` or `zone_redundant` but not both. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Setting this value will provision 2 Physical Hosts for your App Service Environment V3, this is done at additional cost, please be aware of the pricing commitment in the [General Availability Notes](https://techcommunity.microsoft.com/t5/apps-on-azure/announcing-app-service-environment-v3-ga/ba-p/2517990)
        #[builder(into, default)]
        pub zone_redundant: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentV3Result {
        /// Should new Private Endpoint Connections be allowed. Defaults to `true`.
        pub allow_new_private_endpoint_connections: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Zero or more `cluster_setting` blocks as defined below.
        pub cluster_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appservice::EnvironmentV3ClusterSetting>,
        >,
        /// This ASEv3 should use dedicated Hosts. Possible values are `2`. Changing this forces a new resource to be created.
        pub dedicated_host_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// the DNS suffix for this App Service Environment V3.
        pub dns_suffix: pulumi_gestalt_rust::Output<String>,
        /// The external inbound IP addresses of the App Service Environment V3.
        pub external_inbound_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// An `inbound_network_dependencies` block as defined below.
        pub inbound_network_dependencies: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appservice::EnvironmentV3InboundNetworkDependency>,
        >,
        /// The internal inbound IP addresses of the App Service Environment V3.
        pub internal_inbound_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies which endpoints to serve internally in the Virtual Network for the App Service Environment. Possible values are `None` (for an External VIP Type), and `"Web, Publishing"` (for an Internal VIP Type). Defaults to `None`. Changing this forces a new resource to be created.
        pub internal_load_balancing_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The number of IP SSL addresses reserved for the App Service Environment V3.
        pub ip_ssl_address_count: pulumi_gestalt_rust::Output<i32>,
        /// Outbound addresses of Linux based Apps in this App Service Environment V3
        pub linux_outbound_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The location where the App Service Environment exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the App Service Environment. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Pricing tier for the front end instances.
        pub pricing_tier: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable remote debug. Defaults to `false`.
        pub remote_debugging_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Resource Group where the App Service Environment exists. Defaults to the Resource Group of the Subnet (specified by `subnet_id`). Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subnet which the App Service Environment should be connected to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** a /24 or larger CIDR is required. Once associated with an ASE, this size cannot be changed.
        ///
        /// > **NOTE:** This Subnet requires a delegation to `Microsoft.Web/hostingEnvironments` as detailed in the example above.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Outbound addresses of Windows based Apps in this App Service Environment V3.
        pub windows_outbound_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set to `true` to deploy the ASEv3 with availability zones supported. Zonal ASEs can be deployed in some regions, you can refer to [Availability Zone support for App Service Environments](https://docs.microsoft.com/azure/app-service/environment/zone-redundancy). You can only set either `dedicated_host_count` or `zone_redundant` but not both. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Setting this value will provision 2 Physical Hosts for your App Service Environment V3, this is done at additional cost, please be aware of the pricing commitment in the [General Availability Notes](https://techcommunity.microsoft.com/t5/apps-on-azure/announcing-app-service-environment-v3-ga/ba-p/2517990)
        pub zone_redundant: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvironmentV3Args,
    ) -> EnvironmentV3Result {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_new_private_endpoint_connections_binding = args
            .allow_new_private_endpoint_connections
            .get_output(context);
        let cluster_settings_binding = args.cluster_settings.get_output(context);
        let dedicated_host_count_binding = args.dedicated_host_count.get_output(context);
        let internal_load_balancing_mode_binding = args
            .internal_load_balancing_mode
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let remote_debugging_enabled_binding = args
            .remote_debugging_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let zone_redundant_binding = args.zone_redundant.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/environmentV3:EnvironmentV3".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowNewPrivateEndpointConnections".into(),
                    value: &allow_new_private_endpoint_connections_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterSettings".into(),
                    value: &cluster_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dedicatedHostCount".into(),
                    value: &dedicated_host_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "internalLoadBalancingMode".into(),
                    value: &internal_load_balancing_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remoteDebuggingEnabled".into(),
                    value: &remote_debugging_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneRedundant".into(),
                    value: &zone_redundant_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvironmentV3Result {
            allow_new_private_endpoint_connections: o
                .get_field("allowNewPrivateEndpointConnections"),
            cluster_settings: o.get_field("clusterSettings"),
            dedicated_host_count: o.get_field("dedicatedHostCount"),
            dns_suffix: o.get_field("dnsSuffix"),
            external_inbound_ip_addresses: o.get_field("externalInboundIpAddresses"),
            inbound_network_dependencies: o.get_field("inboundNetworkDependencies"),
            internal_inbound_ip_addresses: o.get_field("internalInboundIpAddresses"),
            internal_load_balancing_mode: o.get_field("internalLoadBalancingMode"),
            ip_ssl_address_count: o.get_field("ipSslAddressCount"),
            linux_outbound_ip_addresses: o.get_field("linuxOutboundIpAddresses"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            pricing_tier: o.get_field("pricingTier"),
            remote_debugging_enabled: o.get_field("remoteDebuggingEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            windows_outbound_ip_addresses: o.get_field("windowsOutboundIpAddresses"),
            zone_redundant: o.get_field("zoneRedundant"),
        }
    }
}
