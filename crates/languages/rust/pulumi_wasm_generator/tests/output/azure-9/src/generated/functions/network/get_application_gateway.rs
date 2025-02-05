pub mod get_application_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApplicationGatewayArgs {
        /// The name of this Application Gateway.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Application Gateway exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetApplicationGatewayArgs,
    ) -> GetApplicationGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetApplicationGatewayResult {
            authentication_certificates: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authenticationCertificates"),
            ),
            autoscale_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoscaleConfigurations"),
            ),
            backend_address_pools: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backendAddressPools"),
            ),
            backend_http_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backendHttpSettings"),
            ),
            custom_error_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customErrorConfigurations"),
            ),
            fips_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fipsEnabled"),
            ),
            firewall_policy_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firewallPolicyId"),
            ),
            force_firewall_policy_association: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forceFirewallPolicyAssociation"),
            ),
            frontend_ip_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("frontendIpConfigurations"),
            ),
            frontend_ports: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("frontendPorts"),
            ),
            gateway_ip_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("gatewayIpConfigurations"),
            ),
            globals: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("globals"),
            ),
            http2_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("http2Enabled"),
            ),
            http_listeners: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("httpListeners"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            private_endpoint_connections: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateEndpointConnections"),
            ),
            private_link_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateLinkConfigurations"),
            ),
            probes: pulumi_wasm_rust::__private::into_domain(o.extract_field("probes")),
            redirect_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("redirectConfigurations"),
            ),
            request_routing_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requestRoutingRules"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            rewrite_rule_sets: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rewriteRuleSets"),
            ),
            skus: pulumi_wasm_rust::__private::into_domain(o.extract_field("skus")),
            ssl_certificates: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sslCertificates"),
            ),
            ssl_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sslPolicies"),
            ),
            ssl_profiles: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sslProfiles"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            trusted_client_certificates: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trustedClientCertificates"),
            ),
            trusted_root_certificates: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trustedRootCertificates"),
            ),
            url_path_maps: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("urlPathMaps"),
            ),
            waf_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("wafConfigurations"),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(o.extract_field("zones")),
        }
    }
}
