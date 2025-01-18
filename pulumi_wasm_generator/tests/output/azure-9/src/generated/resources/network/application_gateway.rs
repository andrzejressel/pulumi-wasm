/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-network
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       addressSpaces:
///         - 10.254.0.0/16
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.254.0.0/24
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: example-pip
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       allocationMethod: Static
///   network:
///     type: azure:network:ApplicationGateway
///     properties:
///       name: example-appgateway
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku:
///         name: Standard_v2
///         tier: Standard_v2
///         capacity: 2
///       gatewayIpConfigurations:
///         - name: my-gateway-ip-configuration
///           subnetId: ${exampleSubnet.id}
///       frontendPorts:
///         - name: ${frontendPortName}
///           port: 80
///       frontendIpConfigurations:
///         - name: ${frontendIpConfigurationName}
///           publicIpAddressId: ${examplePublicIp.id}
///       backendAddressPools:
///         - name: ${backendAddressPoolName}
///       backendHttpSettings:
///         - name: ${httpSettingName}
///           cookieBasedAffinity: Disabled
///           path: /path1/
///           port: 80
///           protocol: Http
///           requestTimeout: 60
///       httpListeners:
///         - name: ${listenerName}
///           frontendIpConfigurationName: ${frontendIpConfigurationName}
///           frontendPortName: ${frontendPortName}
///           protocol: Http
///       requestRoutingRules:
///         - name: ${requestRoutingRuleName}
///           priority: 9
///           ruleType: Basic
///           httpListenerName: ${listenerName}
///           backendAddressPoolName: ${backendAddressPoolName}
///           backendHttpSettingsName: ${httpSettingName}
/// variables:
///   backendAddressPoolName: ${exampleVirtualNetwork.name}-beap
///   frontendPortName: ${exampleVirtualNetwork.name}-feport
///   frontendIpConfigurationName: ${exampleVirtualNetwork.name}-feip
///   httpSettingName: ${exampleVirtualNetwork.name}-be-htst
///   listenerName: ${exampleVirtualNetwork.name}-httplstn
///   requestRoutingRuleName: ${exampleVirtualNetwork.name}-rqrt
///   redirectConfigurationName: ${exampleVirtualNetwork.name}-rdrcfg
/// ```
///
/// ## Import
///
/// Application Gateway's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/applicationGateway:ApplicationGateway example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/applicationGateways/myGateway1
/// ```
///
pub mod application_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationGatewayArgs {
        /// One or more `authentication_certificate` blocks as defined below.
        #[builder(into, default)]
        pub authentication_certificates: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayAuthenticationCertificate,
                >,
            >,
        >,
        /// An `autoscale_configuration` block as defined below.
        #[builder(into, default)]
        pub autoscale_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::network::ApplicationGatewayAutoscaleConfiguration,
            >,
        >,
        /// One or more `backend_address_pool` blocks as defined below.
        #[builder(into)]
        pub backend_address_pools: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayBackendAddressPool>,
        >,
        /// One or more `backend_http_settings` blocks as defined below.
        #[builder(into)]
        pub backend_http_settings: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayBackendHttpSetting>,
        >,
        /// One or more `custom_error_configuration` blocks as defined below.
        #[builder(into, default)]
        pub custom_error_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayCustomErrorConfiguration,
                >,
            >,
        >,
        /// Is HTTP2 enabled on the application gateway resource? Defaults to `false`.
        #[builder(into, default)]
        pub enable_http2: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is FIPS enabled on the Application Gateway?
        #[builder(into, default)]
        pub fips_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Web Application Firewall Policy.
        #[builder(into, default)]
        pub firewall_policy_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Is the Firewall Policy associated with the Application Gateway?
        #[builder(into, default)]
        pub force_firewall_policy_association: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `frontend_ip_configuration` blocks as defined below.
        #[builder(into)]
        pub frontend_ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayFrontendIpConfiguration>,
        >,
        /// One or more `frontend_port` blocks as defined below.
        #[builder(into)]
        pub frontend_ports: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayFrontendPort>,
        >,
        /// One or more `gateway_ip_configuration` blocks as defined below.
        #[builder(into)]
        pub gateway_ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayGatewayIpConfiguration>,
        >,
        /// A `global` block as defined below.
        #[builder(into, default)]
        pub global: pulumi_wasm_rust::Output<
            Option<super::super::types::network::ApplicationGatewayGlobal>,
        >,
        /// One or more `http_listener` blocks as defined below.
        #[builder(into)]
        pub http_listeners: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayHttpListener>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::network::ApplicationGatewayIdentity>,
        >,
        /// The Azure region where the Application Gateway should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Application Gateway. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `private_link_configuration` blocks as defined below.
        #[builder(into, default)]
        pub private_link_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayPrivateLinkConfiguration,
                >,
            >,
        >,
        /// One or more `probe` blocks as defined below.
        #[builder(into, default)]
        pub probes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::ApplicationGatewayProbe>>,
        >,
        /// One or more `redirect_configuration` blocks as defined below.
        #[builder(into, default)]
        pub redirect_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayRedirectConfiguration,
                >,
            >,
        >,
        /// One or more `request_routing_rule` blocks as defined below.
        #[builder(into)]
        pub request_routing_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayRequestRoutingRule>,
        >,
        /// The name of the resource group in which to the Application Gateway should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `rewrite_rule_set` blocks as defined below. Only valid for v2 WAF and Standard SKUs.
        #[builder(into, default)]
        pub rewrite_rule_sets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::ApplicationGatewayRewriteRuleSet>>,
        >,
        /// A `sku` block as defined below.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::Output<
            super::super::types::network::ApplicationGatewaySku,
        >,
        /// One or more `ssl_certificate` blocks as defined below.
        #[builder(into, default)]
        pub ssl_certificates: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::ApplicationGatewaySslCertificate>>,
        >,
        /// a `ssl_policy` block as defined below.
        #[builder(into, default)]
        pub ssl_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::network::ApplicationGatewaySslPolicy>,
        >,
        /// One or more `ssl_profile` blocks as defined below.
        #[builder(into, default)]
        pub ssl_profiles: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::ApplicationGatewaySslProfile>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `trusted_client_certificate` blocks as defined below.
        #[builder(into, default)]
        pub trusted_client_certificates: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayTrustedClientCertificate,
                >,
            >,
        >,
        /// One or more `trusted_root_certificate` blocks as defined below.
        #[builder(into, default)]
        pub trusted_root_certificates: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayTrustedRootCertificate,
                >,
            >,
        >,
        /// One or more `url_path_map` blocks as defined below.
        #[builder(into, default)]
        pub url_path_maps: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::ApplicationGatewayUrlPathMap>>,
        >,
        /// A `waf_configuration` block as defined below.
        #[builder(into, default)]
        pub waf_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::network::ApplicationGatewayWafConfiguration>,
        >,
        /// Specifies a list of Availability Zones in which this Application Gateway should be located. Changing this forces a new Application Gateway to be created.
        ///
        /// > **Please Note**: Availability Zones are not supported in all regions at this time, please check the [official documentation](https://docs.microsoft.com/azure/availability-zones/az-overview) for more information. They are also only supported for [v2 SKUs](https://docs.microsoft.com/azure/application-gateway/application-gateway-autoscaling-zone-redundant)
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ApplicationGatewayResult {
        /// One or more `authentication_certificate` blocks as defined below.
        pub authentication_certificates: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayAuthenticationCertificate,
                >,
            >,
        >,
        /// An `autoscale_configuration` block as defined below.
        pub autoscale_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::network::ApplicationGatewayAutoscaleConfiguration,
            >,
        >,
        /// One or more `backend_address_pool` blocks as defined below.
        pub backend_address_pools: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayBackendAddressPool>,
        >,
        /// One or more `backend_http_settings` blocks as defined below.
        pub backend_http_settings: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayBackendHttpSetting>,
        >,
        /// One or more `custom_error_configuration` blocks as defined below.
        pub custom_error_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayCustomErrorConfiguration,
                >,
            >,
        >,
        /// Is HTTP2 enabled on the application gateway resource? Defaults to `false`.
        pub enable_http2: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is FIPS enabled on the Application Gateway?
        pub fips_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Web Application Firewall Policy.
        pub firewall_policy_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Is the Firewall Policy associated with the Application Gateway?
        pub force_firewall_policy_association: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `frontend_ip_configuration` blocks as defined below.
        pub frontend_ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayFrontendIpConfiguration>,
        >,
        /// One or more `frontend_port` blocks as defined below.
        pub frontend_ports: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayFrontendPort>,
        >,
        /// One or more `gateway_ip_configuration` blocks as defined below.
        pub gateway_ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayGatewayIpConfiguration>,
        >,
        /// A `global` block as defined below.
        pub global: pulumi_wasm_rust::Output<
            Option<super::super::types::network::ApplicationGatewayGlobal>,
        >,
        /// One or more `http_listener` blocks as defined below.
        pub http_listeners: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayHttpListener>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::network::ApplicationGatewayIdentity>,
        >,
        /// The Azure region where the Application Gateway should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Application Gateway. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of `private_endpoint_connection` blocks as defined below.
        pub private_endpoint_connections: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::network::ApplicationGatewayPrivateEndpointConnection,
            >,
        >,
        /// One or more `private_link_configuration` blocks as defined below.
        pub private_link_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayPrivateLinkConfiguration,
                >,
            >,
        >,
        /// One or more `probe` blocks as defined below.
        pub probes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::ApplicationGatewayProbe>>,
        >,
        /// One or more `redirect_configuration` blocks as defined below.
        pub redirect_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayRedirectConfiguration,
                >,
            >,
        >,
        /// One or more `request_routing_rule` blocks as defined below.
        pub request_routing_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayRequestRoutingRule>,
        >,
        /// The name of the resource group in which to the Application Gateway should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `rewrite_rule_set` blocks as defined below. Only valid for v2 WAF and Standard SKUs.
        pub rewrite_rule_sets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::ApplicationGatewayRewriteRuleSet>>,
        >,
        /// A `sku` block as defined below.
        pub sku: pulumi_wasm_rust::Output<
            super::super::types::network::ApplicationGatewaySku,
        >,
        /// One or more `ssl_certificate` blocks as defined below.
        pub ssl_certificates: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::ApplicationGatewaySslCertificate>>,
        >,
        /// a `ssl_policy` block as defined below.
        pub ssl_policy: pulumi_wasm_rust::Output<
            super::super::types::network::ApplicationGatewaySslPolicy,
        >,
        /// One or more `ssl_profile` blocks as defined below.
        pub ssl_profiles: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::ApplicationGatewaySslProfile>>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `trusted_client_certificate` blocks as defined below.
        pub trusted_client_certificates: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayTrustedClientCertificate,
                >,
            >,
        >,
        /// One or more `trusted_root_certificate` blocks as defined below.
        pub trusted_root_certificates: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayTrustedRootCertificate,
                >,
            >,
        >,
        /// One or more `url_path_map` blocks as defined below.
        pub url_path_maps: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::ApplicationGatewayUrlPathMap>>,
        >,
        /// A `waf_configuration` block as defined below.
        pub waf_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::network::ApplicationGatewayWafConfiguration>,
        >,
        /// Specifies a list of Availability Zones in which this Application Gateway should be located. Changing this forces a new Application Gateway to be created.
        ///
        /// > **Please Note**: Availability Zones are not supported in all regions at this time, please check the [official documentation](https://docs.microsoft.com/azure/availability-zones/az-overview) for more information. They are also only supported for [v2 SKUs](https://docs.microsoft.com/azure/application-gateway/application-gateway-autoscaling-zone-redundant)
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApplicationGatewayArgs) -> ApplicationGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_certificates_binding = args
            .authentication_certificates
            .get_inner();
        let autoscale_configuration_binding = args.autoscale_configuration.get_inner();
        let backend_address_pools_binding = args.backend_address_pools.get_inner();
        let backend_http_settings_binding = args.backend_http_settings.get_inner();
        let custom_error_configurations_binding = args
            .custom_error_configurations
            .get_inner();
        let enable_http2_binding = args.enable_http2.get_inner();
        let fips_enabled_binding = args.fips_enabled.get_inner();
        let firewall_policy_id_binding = args.firewall_policy_id.get_inner();
        let force_firewall_policy_association_binding = args
            .force_firewall_policy_association
            .get_inner();
        let frontend_ip_configurations_binding = args
            .frontend_ip_configurations
            .get_inner();
        let frontend_ports_binding = args.frontend_ports.get_inner();
        let gateway_ip_configurations_binding = args
            .gateway_ip_configurations
            .get_inner();
        let global_binding = args.global.get_inner();
        let http_listeners_binding = args.http_listeners.get_inner();
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let private_link_configurations_binding = args
            .private_link_configurations
            .get_inner();
        let probes_binding = args.probes.get_inner();
        let redirect_configurations_binding = args.redirect_configurations.get_inner();
        let request_routing_rules_binding = args.request_routing_rules.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let rewrite_rule_sets_binding = args.rewrite_rule_sets.get_inner();
        let sku_binding = args.sku.get_inner();
        let ssl_certificates_binding = args.ssl_certificates.get_inner();
        let ssl_policy_binding = args.ssl_policy.get_inner();
        let ssl_profiles_binding = args.ssl_profiles.get_inner();
        let tags_binding = args.tags.get_inner();
        let trusted_client_certificates_binding = args
            .trusted_client_certificates
            .get_inner();
        let trusted_root_certificates_binding = args
            .trusted_root_certificates
            .get_inner();
        let url_path_maps_binding = args.url_path_maps.get_inner();
        let waf_configuration_binding = args.waf_configuration.get_inner();
        let zones_binding = args.zones.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/applicationGateway:ApplicationGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationCertificates".into(),
                    value: &authentication_certificates_binding,
                },
                register_interface::ObjectField {
                    name: "autoscaleConfiguration".into(),
                    value: &autoscale_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "backendAddressPools".into(),
                    value: &backend_address_pools_binding,
                },
                register_interface::ObjectField {
                    name: "backendHttpSettings".into(),
                    value: &backend_http_settings_binding,
                },
                register_interface::ObjectField {
                    name: "customErrorConfigurations".into(),
                    value: &custom_error_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "enableHttp2".into(),
                    value: &enable_http2_binding,
                },
                register_interface::ObjectField {
                    name: "fipsEnabled".into(),
                    value: &fips_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "firewallPolicyId".into(),
                    value: &firewall_policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "forceFirewallPolicyAssociation".into(),
                    value: &force_firewall_policy_association_binding,
                },
                register_interface::ObjectField {
                    name: "frontendIpConfigurations".into(),
                    value: &frontend_ip_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "frontendPorts".into(),
                    value: &frontend_ports_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayIpConfigurations".into(),
                    value: &gateway_ip_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "global".into(),
                    value: &global_binding,
                },
                register_interface::ObjectField {
                    name: "httpListeners".into(),
                    value: &http_listeners_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateLinkConfigurations".into(),
                    value: &private_link_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "probes".into(),
                    value: &probes_binding,
                },
                register_interface::ObjectField {
                    name: "redirectConfigurations".into(),
                    value: &redirect_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "requestRoutingRules".into(),
                    value: &request_routing_rules_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "rewriteRuleSets".into(),
                    value: &rewrite_rule_sets_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "sslCertificates".into(),
                    value: &ssl_certificates_binding,
                },
                register_interface::ObjectField {
                    name: "sslPolicy".into(),
                    value: &ssl_policy_binding,
                },
                register_interface::ObjectField {
                    name: "sslProfiles".into(),
                    value: &ssl_profiles_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "trustedClientCertificates".into(),
                    value: &trusted_client_certificates_binding,
                },
                register_interface::ObjectField {
                    name: "trustedRootCertificates".into(),
                    value: &trusted_root_certificates_binding,
                },
                register_interface::ObjectField {
                    name: "urlPathMaps".into(),
                    value: &url_path_maps_binding,
                },
                register_interface::ObjectField {
                    name: "wafConfiguration".into(),
                    value: &waf_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "zones".into(),
                    value: &zones_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authenticationCertificates".into(),
                },
                register_interface::ResultField {
                    name: "autoscaleConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "backendAddressPools".into(),
                },
                register_interface::ResultField {
                    name: "backendHttpSettings".into(),
                },
                register_interface::ResultField {
                    name: "customErrorConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "enableHttp2".into(),
                },
                register_interface::ResultField {
                    name: "fipsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "firewallPolicyId".into(),
                },
                register_interface::ResultField {
                    name: "forceFirewallPolicyAssociation".into(),
                },
                register_interface::ResultField {
                    name: "frontendIpConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "frontendPorts".into(),
                },
                register_interface::ResultField {
                    name: "gatewayIpConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "global".into(),
                },
                register_interface::ResultField {
                    name: "httpListeners".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateEndpointConnections".into(),
                },
                register_interface::ResultField {
                    name: "privateLinkConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "probes".into(),
                },
                register_interface::ResultField {
                    name: "redirectConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "requestRoutingRules".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "rewriteRuleSets".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "sslCertificates".into(),
                },
                register_interface::ResultField {
                    name: "sslPolicy".into(),
                },
                register_interface::ResultField {
                    name: "sslProfiles".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "trustedClientCertificates".into(),
                },
                register_interface::ResultField {
                    name: "trustedRootCertificates".into(),
                },
                register_interface::ResultField {
                    name: "urlPathMaps".into(),
                },
                register_interface::ResultField {
                    name: "wafConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationGatewayResult {
            authentication_certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationCertificates").unwrap(),
            ),
            autoscale_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscaleConfiguration").unwrap(),
            ),
            backend_address_pools: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendAddressPools").unwrap(),
            ),
            backend_http_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendHttpSettings").unwrap(),
            ),
            custom_error_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customErrorConfigurations").unwrap(),
            ),
            enable_http2: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableHttp2").unwrap(),
            ),
            fips_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fipsEnabled").unwrap(),
            ),
            firewall_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallPolicyId").unwrap(),
            ),
            force_firewall_policy_association: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceFirewallPolicyAssociation").unwrap(),
            ),
            frontend_ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frontendIpConfigurations").unwrap(),
            ),
            frontend_ports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frontendPorts").unwrap(),
            ),
            gateway_ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayIpConfigurations").unwrap(),
            ),
            global: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("global").unwrap(),
            ),
            http_listeners: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpListeners").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_endpoint_connections: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateEndpointConnections").unwrap(),
            ),
            private_link_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateLinkConfigurations").unwrap(),
            ),
            probes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("probes").unwrap(),
            ),
            redirect_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redirectConfigurations").unwrap(),
            ),
            request_routing_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestRoutingRules").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            rewrite_rule_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rewriteRuleSets").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            ssl_certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslCertificates").unwrap(),
            ),
            ssl_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslPolicy").unwrap(),
            ),
            ssl_profiles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslProfiles").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            trusted_client_certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustedClientCertificates").unwrap(),
            ),
            trusted_root_certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustedRootCertificates").unwrap(),
            ),
            url_path_maps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("urlPathMaps").unwrap(),
            ),
            waf_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("wafConfiguration").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}
