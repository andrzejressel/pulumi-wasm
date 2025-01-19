pub mod get_application_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApplicationGatewayArgs {
        /// The name of this Application Gateway.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Application Gateway exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetApplicationGatewayResult {
        /// One or more `authentication_certificate` blocks as defined below.
        pub authentication_certificates: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayAuthenticationCertificate,
            >,
        >,
        /// An `autoscale_configuration` block as defined below.
        pub autoscale_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayAutoscaleConfiguration,
            >,
        >,
        /// One or more `backend_address_pool` blocks as defined below.
        pub backend_address_pools: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayBackendAddressPool,
            >,
        >,
        /// One or more `backend_http_settings` blocks as defined below.
        pub backend_http_settings: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayBackendHttpSetting,
            >,
        >,
        /// One or more `custom_error_configuration` blocks as defined below.
        pub custom_error_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayCustomErrorConfiguration,
            >,
        >,
        /// Is FIPS enabled on the Application Gateway?
        pub fips_enabled: pulumi_wasm_rust::Output<bool>,
        /// The ID of the Web Application Firewall Policy which is used as an HTTP Listener for this Path Rule.
        pub firewall_policy_id: pulumi_wasm_rust::Output<String>,
        /// Is the Firewall Policy associated with the Application Gateway?
        pub force_firewall_policy_association: pulumi_wasm_rust::Output<bool>,
        /// One or more `frontend_ip_configuration` blocks as defined below.
        pub frontend_ip_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayFrontendIpConfiguration,
            >,
        >,
        /// One or more `frontend_port` blocks as defined below.
        pub frontend_ports: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewayFrontendPort>,
        >,
        /// One or more `gateway_ip_configuration` blocks as defined below.
        pub gateway_ip_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayGatewayIpConfiguration,
            >,
        >,
        /// A `global` block as defined below.
        pub globals: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewayGlobal>,
        >,
        /// Is HTTP2 enabled on the application gateway resource?
        pub http2_enabled: pulumi_wasm_rust::Output<bool>,
        /// One or more `http_listener` blocks as defined below.
        pub http_listeners: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewayHttpListener>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewayIdentity>,
        >,
        /// The Azure region where the Application Gateway exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Unique name of the Rewrite Rule
        pub name: pulumi_wasm_rust::Output<String>,
        pub private_endpoint_connections: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayPrivateEndpointConnection,
            >,
        >,
        /// One or more `private_link_configuration` blocks as defined below.
        pub private_link_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayPrivateLinkConfiguration,
            >,
        >,
        /// One or more `probe` blocks as defined below.
        pub probes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewayProbe>,
        >,
        /// One or more `redirect_configuration` blocks as defined below.
        pub redirect_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayRedirectConfiguration,
            >,
        >,
        /// One or more `request_routing_rule` blocks as defined below.
        pub request_routing_rules: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayRequestRoutingRule,
            >,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `rewrite_rule_set` blocks as defined below.
        pub rewrite_rule_sets: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewayRewriteRuleSet>,
        >,
        /// A `sku` block as defined below.
        pub skus: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewaySkus>,
        >,
        /// One or more `ssl_certificate` blocks as defined below.
        pub ssl_certificates: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewaySslCertificate>,
        >,
        /// a `ssl_policy` block as defined below.
        pub ssl_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewaySslPolicy>,
        >,
        /// One or more `ssl_profile` blocks as defined below.
        pub ssl_profiles: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewaySslProfile>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// One or more `trusted_client_certificate` blocks as defined below.
        pub trusted_client_certificates: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayTrustedClientCertificate,
            >,
        >,
        /// One or more `trusted_root_certificate` blocks as defined below.
        pub trusted_root_certificates: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayTrustedRootCertificate,
            >,
        >,
        /// One or more `url_path_map` blocks as defined below.
        pub url_path_maps: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewayUrlPathMap>,
        >,
        /// A `waf_configuration` block as defined below.
        pub waf_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayWafConfiguration,
            >,
        >,
        /// The list of Availability Zones in which this Application Gateway can use.
        pub zones: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetApplicationGatewayArgs) -> GetApplicationGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getApplicationGateway:getApplicationGateway".into(),
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
                    name: "authenticationCertificates".into(),
                },
                register_interface::ResultField {
                    name: "autoscaleConfigurations".into(),
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
                    name: "globals".into(),
                },
                register_interface::ResultField {
                    name: "http2Enabled".into(),
                },
                register_interface::ResultField {
                    name: "httpListeners".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
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
                    name: "skus".into(),
                },
                register_interface::ResultField {
                    name: "sslCertificates".into(),
                },
                register_interface::ResultField {
                    name: "sslPolicies".into(),
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
                    name: "wafConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetApplicationGatewayResult {
            authentication_certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationCertificates").unwrap(),
            ),
            autoscale_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscaleConfigurations").unwrap(),
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
            globals: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globals").unwrap(),
            ),
            http2_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("http2Enabled").unwrap(),
            ),
            http_listeners: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpListeners").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
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
            skus: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skus").unwrap(),
            ),
            ssl_certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslCertificates").unwrap(),
            ),
            ssl_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslPolicies").unwrap(),
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
            waf_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("wafConfigurations").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}
