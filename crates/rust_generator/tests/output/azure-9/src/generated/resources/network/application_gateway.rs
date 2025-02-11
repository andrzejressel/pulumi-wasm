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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationGatewayArgs {
        /// One or more `authentication_certificate` blocks as defined below.
        #[builder(into, default)]
        pub authentication_certificates: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayAuthenticationCertificate,
                >,
            >,
        >,
        /// An `autoscale_configuration` block as defined below.
        #[builder(into, default)]
        pub autoscale_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::network::ApplicationGatewayAutoscaleConfiguration,
            >,
        >,
        /// One or more `backend_address_pool` blocks as defined below.
        #[builder(into)]
        pub backend_address_pools: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::network::ApplicationGatewayBackendAddressPool>,
        >,
        /// One or more `backend_http_settings` blocks as defined below.
        #[builder(into)]
        pub backend_http_settings: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::network::ApplicationGatewayBackendHttpSetting>,
        >,
        /// One or more `custom_error_configuration` blocks as defined below.
        #[builder(into, default)]
        pub custom_error_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayCustomErrorConfiguration,
                >,
            >,
        >,
        /// Is HTTP2 enabled on the application gateway resource? Defaults to `false`.
        #[builder(into, default)]
        pub enable_http2: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Is FIPS enabled on the Application Gateway?
        #[builder(into, default)]
        pub fips_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Web Application Firewall Policy.
        #[builder(into, default)]
        pub firewall_policy_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is the Firewall Policy associated with the Application Gateway?
        #[builder(into, default)]
        pub force_firewall_policy_association: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// One or more `frontend_ip_configuration` blocks as defined below.
        #[builder(into)]
        pub frontend_ip_configurations: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::network::ApplicationGatewayFrontendIpConfiguration>,
        >,
        /// One or more `frontend_port` blocks as defined below.
        #[builder(into)]
        pub frontend_ports: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::network::ApplicationGatewayFrontendPort>,
        >,
        /// One or more `gateway_ip_configuration` blocks as defined below.
        #[builder(into)]
        pub gateway_ip_configurations: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::network::ApplicationGatewayGatewayIpConfiguration>,
        >,
        /// A `global` block as defined below.
        #[builder(into, default)]
        pub global: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::ApplicationGatewayGlobal>,
        >,
        /// One or more `http_listener` blocks as defined below.
        #[builder(into)]
        pub http_listeners: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::network::ApplicationGatewayHttpListener>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::ApplicationGatewayIdentity>,
        >,
        /// The Azure region where the Application Gateway should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Application Gateway. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `private_link_configuration` blocks as defined below.
        #[builder(into, default)]
        pub private_link_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayPrivateLinkConfiguration,
                >,
            >,
        >,
        /// One or more `probe` blocks as defined below.
        #[builder(into, default)]
        pub probes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::network::ApplicationGatewayProbe>>,
        >,
        /// One or more `redirect_configuration` blocks as defined below.
        #[builder(into, default)]
        pub redirect_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayRedirectConfiguration,
                >,
            >,
        >,
        /// One or more `request_routing_rule` blocks as defined below.
        #[builder(into)]
        pub request_routing_rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::network::ApplicationGatewayRequestRoutingRule>,
        >,
        /// The name of the resource group in which to the Application Gateway should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `rewrite_rule_set` blocks as defined below. Only valid for v2 WAF and Standard SKUs.
        #[builder(into, default)]
        pub rewrite_rule_sets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::network::ApplicationGatewayRewriteRuleSet>>,
        >,
        /// A `sku` block as defined below.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::network::ApplicationGatewaySku,
        >,
        /// One or more `ssl_certificate` blocks as defined below.
        #[builder(into, default)]
        pub ssl_certificates: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::network::ApplicationGatewaySslCertificate>>,
        >,
        /// a `ssl_policy` block as defined below.
        #[builder(into, default)]
        pub ssl_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::ApplicationGatewaySslPolicy>,
        >,
        /// One or more `ssl_profile` blocks as defined below.
        #[builder(into, default)]
        pub ssl_profiles: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::network::ApplicationGatewaySslProfile>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `trusted_client_certificate` blocks as defined below.
        #[builder(into, default)]
        pub trusted_client_certificates: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayTrustedClientCertificate,
                >,
            >,
        >,
        /// One or more `trusted_root_certificate` blocks as defined below.
        #[builder(into, default)]
        pub trusted_root_certificates: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayTrustedRootCertificate,
                >,
            >,
        >,
        /// One or more `url_path_map` blocks as defined below.
        #[builder(into, default)]
        pub url_path_maps: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::network::ApplicationGatewayUrlPathMap>>,
        >,
        /// A `waf_configuration` block as defined below.
        #[builder(into, default)]
        pub waf_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::ApplicationGatewayWafConfiguration>,
        >,
        /// Specifies a list of Availability Zones in which this Application Gateway should be located. Changing this forces a new Application Gateway to be created.
        ///
        /// > **Please Note**: Availability Zones are not supported in all regions at this time, please check the [official documentation](https://docs.microsoft.com/azure/availability-zones/az-overview) for more information. They are also only supported for [v2 SKUs](https://docs.microsoft.com/azure/application-gateway/application-gateway-autoscaling-zone-redundant)
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ApplicationGatewayResult {
        /// One or more `authentication_certificate` blocks as defined below.
        pub authentication_certificates: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayAuthenticationCertificate,
                >,
            >,
        >,
        /// An `autoscale_configuration` block as defined below.
        pub autoscale_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::network::ApplicationGatewayAutoscaleConfiguration,
            >,
        >,
        /// One or more `backend_address_pool` blocks as defined below.
        pub backend_address_pools: pulumi_gestalt_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayBackendAddressPool>,
        >,
        /// One or more `backend_http_settings` blocks as defined below.
        pub backend_http_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayBackendHttpSetting>,
        >,
        /// One or more `custom_error_configuration` blocks as defined below.
        pub custom_error_configurations: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayCustomErrorConfiguration,
                >,
            >,
        >,
        /// Is HTTP2 enabled on the application gateway resource? Defaults to `false`.
        pub enable_http2: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Is FIPS enabled on the Application Gateway?
        pub fips_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Web Application Firewall Policy.
        pub firewall_policy_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Is the Firewall Policy associated with the Application Gateway?
        pub force_firewall_policy_association: pulumi_gestalt_rust::Output<Option<bool>>,
        /// One or more `frontend_ip_configuration` blocks as defined below.
        pub frontend_ip_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayFrontendIpConfiguration>,
        >,
        /// One or more `frontend_port` blocks as defined below.
        pub frontend_ports: pulumi_gestalt_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayFrontendPort>,
        >,
        /// One or more `gateway_ip_configuration` blocks as defined below.
        pub gateway_ip_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayGatewayIpConfiguration>,
        >,
        /// A `global` block as defined below.
        pub global: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::ApplicationGatewayGlobal>,
        >,
        /// One or more `http_listener` blocks as defined below.
        pub http_listeners: pulumi_gestalt_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayHttpListener>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::ApplicationGatewayIdentity>,
        >,
        /// The Azure region where the Application Gateway should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Application Gateway. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of `private_endpoint_connection` blocks as defined below.
        pub private_endpoint_connections: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::network::ApplicationGatewayPrivateEndpointConnection,
            >,
        >,
        /// One or more `private_link_configuration` blocks as defined below.
        pub private_link_configurations: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayPrivateLinkConfiguration,
                >,
            >,
        >,
        /// One or more `probe` blocks as defined below.
        pub probes: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::network::ApplicationGatewayProbe>>,
        >,
        /// One or more `redirect_configuration` blocks as defined below.
        pub redirect_configurations: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayRedirectConfiguration,
                >,
            >,
        >,
        /// One or more `request_routing_rule` blocks as defined below.
        pub request_routing_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::network::ApplicationGatewayRequestRoutingRule>,
        >,
        /// The name of the resource group in which to the Application Gateway should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `rewrite_rule_set` blocks as defined below. Only valid for v2 WAF and Standard SKUs.
        pub rewrite_rule_sets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::network::ApplicationGatewayRewriteRuleSet>>,
        >,
        /// A `sku` block as defined below.
        pub sku: pulumi_gestalt_rust::Output<
            super::super::types::network::ApplicationGatewaySku,
        >,
        /// One or more `ssl_certificate` blocks as defined below.
        pub ssl_certificates: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::network::ApplicationGatewaySslCertificate>>,
        >,
        /// a `ssl_policy` block as defined below.
        pub ssl_policy: pulumi_gestalt_rust::Output<
            super::super::types::network::ApplicationGatewaySslPolicy,
        >,
        /// One or more `ssl_profile` blocks as defined below.
        pub ssl_profiles: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::network::ApplicationGatewaySslProfile>>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `trusted_client_certificate` blocks as defined below.
        pub trusted_client_certificates: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayTrustedClientCertificate,
                >,
            >,
        >,
        /// One or more `trusted_root_certificate` blocks as defined below.
        pub trusted_root_certificates: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::network::ApplicationGatewayTrustedRootCertificate,
                >,
            >,
        >,
        /// One or more `url_path_map` blocks as defined below.
        pub url_path_maps: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::network::ApplicationGatewayUrlPathMap>>,
        >,
        /// A `waf_configuration` block as defined below.
        pub waf_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::ApplicationGatewayWafConfiguration>,
        >,
        /// Specifies a list of Availability Zones in which this Application Gateway should be located. Changing this forces a new Application Gateway to be created.
        ///
        /// > **Please Note**: Availability Zones are not supported in all regions at this time, please check the [official documentation](https://docs.microsoft.com/azure/availability-zones/az-overview) for more information. They are also only supported for [v2 SKUs](https://docs.microsoft.com/azure/application-gateway/application-gateway-autoscaling-zone-redundant)
        pub zones: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationGatewayArgs,
    ) -> ApplicationGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authentication_certificates_binding = args
            .authentication_certificates
            .get_output(context);
        let autoscale_configuration_binding = args
            .autoscale_configuration
            .get_output(context);
        let backend_address_pools_binding = args
            .backend_address_pools
            .get_output(context);
        let backend_http_settings_binding = args
            .backend_http_settings
            .get_output(context);
        let custom_error_configurations_binding = args
            .custom_error_configurations
            .get_output(context);
        let enable_http2_binding = args.enable_http2.get_output(context);
        let fips_enabled_binding = args.fips_enabled.get_output(context);
        let firewall_policy_id_binding = args.firewall_policy_id.get_output(context);
        let force_firewall_policy_association_binding = args
            .force_firewall_policy_association
            .get_output(context);
        let frontend_ip_configurations_binding = args
            .frontend_ip_configurations
            .get_output(context);
        let frontend_ports_binding = args.frontend_ports.get_output(context);
        let gateway_ip_configurations_binding = args
            .gateway_ip_configurations
            .get_output(context);
        let global_binding = args.global.get_output(context);
        let http_listeners_binding = args.http_listeners.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let private_link_configurations_binding = args
            .private_link_configurations
            .get_output(context);
        let probes_binding = args.probes.get_output(context);
        let redirect_configurations_binding = args
            .redirect_configurations
            .get_output(context);
        let request_routing_rules_binding = args
            .request_routing_rules
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let rewrite_rule_sets_binding = args.rewrite_rule_sets.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let ssl_certificates_binding = args.ssl_certificates.get_output(context);
        let ssl_policy_binding = args.ssl_policy.get_output(context);
        let ssl_profiles_binding = args.ssl_profiles.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let trusted_client_certificates_binding = args
            .trusted_client_certificates
            .get_output(context);
        let trusted_root_certificates_binding = args
            .trusted_root_certificates
            .get_output(context);
        let url_path_maps_binding = args.url_path_maps.get_output(context);
        let waf_configuration_binding = args.waf_configuration.get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/applicationGateway:ApplicationGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationCertificates".into(),
                    value: &authentication_certificates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscaleConfiguration".into(),
                    value: &autoscale_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backendAddressPools".into(),
                    value: &backend_address_pools_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backendHttpSettings".into(),
                    value: &backend_http_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customErrorConfigurations".into(),
                    value: &custom_error_configurations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableHttp2".into(),
                    value: &enable_http2_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fipsEnabled".into(),
                    value: &fips_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallPolicyId".into(),
                    value: &firewall_policy_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceFirewallPolicyAssociation".into(),
                    value: &force_firewall_policy_association_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frontendIpConfigurations".into(),
                    value: &frontend_ip_configurations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frontendPorts".into(),
                    value: &frontend_ports_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayIpConfigurations".into(),
                    value: &gateway_ip_configurations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "global".into(),
                    value: &global_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpListeners".into(),
                    value: &http_listeners_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateLinkConfigurations".into(),
                    value: &private_link_configurations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "probes".into(),
                    value: &probes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redirectConfigurations".into(),
                    value: &redirect_configurations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestRoutingRules".into(),
                    value: &request_routing_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rewriteRuleSets".into(),
                    value: &rewrite_rule_sets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sslCertificates".into(),
                    value: &ssl_certificates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sslPolicy".into(),
                    value: &ssl_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sslProfiles".into(),
                    value: &ssl_profiles_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trustedClientCertificates".into(),
                    value: &trusted_client_certificates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trustedRootCertificates".into(),
                    value: &trusted_root_certificates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "urlPathMaps".into(),
                    value: &url_path_maps_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "wafConfiguration".into(),
                    value: &waf_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: &zones_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationGatewayResult {
            authentication_certificates: o.get_field("authenticationCertificates"),
            autoscale_configuration: o.get_field("autoscaleConfiguration"),
            backend_address_pools: o.get_field("backendAddressPools"),
            backend_http_settings: o.get_field("backendHttpSettings"),
            custom_error_configurations: o.get_field("customErrorConfigurations"),
            enable_http2: o.get_field("enableHttp2"),
            fips_enabled: o.get_field("fipsEnabled"),
            firewall_policy_id: o.get_field("firewallPolicyId"),
            force_firewall_policy_association: o
                .get_field("forceFirewallPolicyAssociation"),
            frontend_ip_configurations: o.get_field("frontendIpConfigurations"),
            frontend_ports: o.get_field("frontendPorts"),
            gateway_ip_configurations: o.get_field("gatewayIpConfigurations"),
            global: o.get_field("global"),
            http_listeners: o.get_field("httpListeners"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            private_endpoint_connections: o.get_field("privateEndpointConnections"),
            private_link_configurations: o.get_field("privateLinkConfigurations"),
            probes: o.get_field("probes"),
            redirect_configurations: o.get_field("redirectConfigurations"),
            request_routing_rules: o.get_field("requestRoutingRules"),
            resource_group_name: o.get_field("resourceGroupName"),
            rewrite_rule_sets: o.get_field("rewriteRuleSets"),
            sku: o.get_field("sku"),
            ssl_certificates: o.get_field("sslCertificates"),
            ssl_policy: o.get_field("sslPolicy"),
            ssl_profiles: o.get_field("sslProfiles"),
            tags: o.get_field("tags"),
            trusted_client_certificates: o.get_field("trustedClientCertificates"),
            trusted_root_certificates: o.get_field("trustedRootCertificates"),
            url_path_maps: o.get_field("urlPathMaps"),
            waf_configuration: o.get_field("wafConfiguration"),
            zones: o.get_field("zones"),
        }
    }
}
