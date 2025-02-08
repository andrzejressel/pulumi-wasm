#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_load_balancer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLoadBalancerArgs {
        /// Full ARN of the load balancer.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique name of the load balancer.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Mapping of tags, each pair of which must exactly match a pair on the desired load balancer.
        ///
        /// > **NOTE:** When both `arn` and `name` are specified, `arn` takes precedence. `tags` has lowest precedence.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLoadBalancerResult {
        pub access_logs: pulumi_gestalt_rust::Output<
            super::super::super::types::lb::GetLoadBalancerAccessLogs,
        >,
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub arn_suffix: pulumi_gestalt_rust::Output<String>,
        pub client_keep_alive: pulumi_gestalt_rust::Output<i32>,
        pub connection_logs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::lb::GetLoadBalancerConnectionLog>,
        >,
        pub customer_owned_ipv4_pool: pulumi_gestalt_rust::Output<String>,
        pub desync_mitigation_mode: pulumi_gestalt_rust::Output<String>,
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        pub dns_record_client_routing_policy: pulumi_gestalt_rust::Output<String>,
        pub drop_invalid_header_fields: pulumi_gestalt_rust::Output<bool>,
        pub enable_cross_zone_load_balancing: pulumi_gestalt_rust::Output<bool>,
        pub enable_deletion_protection: pulumi_gestalt_rust::Output<bool>,
        pub enable_http2: pulumi_gestalt_rust::Output<bool>,
        pub enable_tls_version_and_cipher_suite_headers: pulumi_gestalt_rust::Output<
            bool,
        >,
        pub enable_waf_fail_open: pulumi_gestalt_rust::Output<bool>,
        pub enable_xff_client_port: pulumi_gestalt_rust::Output<bool>,
        pub enable_zonal_shift: pulumi_gestalt_rust::Output<bool>,
        pub enforce_security_group_inbound_rules_on_private_link_traffic: pulumi_gestalt_rust::Output<
            String,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub idle_timeout: pulumi_gestalt_rust::Output<i32>,
        pub internal: pulumi_gestalt_rust::Output<bool>,
        pub ip_address_type: pulumi_gestalt_rust::Output<String>,
        pub load_balancer_type: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub preserve_host_header: pulumi_gestalt_rust::Output<bool>,
        pub security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        pub subnet_mappings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::lb::GetLoadBalancerSubnetMapping>,
        >,
        pub subnets: pulumi_gestalt_rust::Output<Vec<String>>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        pub xff_header_processing_mode: pulumi_gestalt_rust::Output<String>,
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLoadBalancerArgs,
    ) -> GetLoadBalancerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lb/getLoadBalancer:getLoadBalancer".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLoadBalancerResult {
            access_logs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessLogs"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            arn_suffix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("arnSuffix"),
            ),
            client_keep_alive: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientKeepAlive"),
            ),
            connection_logs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionLogs"),
            ),
            customer_owned_ipv4_pool: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customerOwnedIpv4Pool"),
            ),
            desync_mitigation_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("desyncMitigationMode"),
            ),
            dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            dns_record_client_routing_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsRecordClientRoutingPolicy"),
            ),
            drop_invalid_header_fields: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dropInvalidHeaderFields"),
            ),
            enable_cross_zone_load_balancing: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableCrossZoneLoadBalancing"),
            ),
            enable_deletion_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableDeletionProtection"),
            ),
            enable_http2: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableHttp2"),
            ),
            enable_tls_version_and_cipher_suite_headers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableTlsVersionAndCipherSuiteHeaders"),
            ),
            enable_waf_fail_open: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableWafFailOpen"),
            ),
            enable_xff_client_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableXffClientPort"),
            ),
            enable_zonal_shift: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableZonalShift"),
            ),
            enforce_security_group_inbound_rules_on_private_link_traffic: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enforceSecurityGroupInboundRulesOnPrivateLinkTraffic"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            idle_timeout: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("idleTimeout"),
            ),
            internal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("internal"),
            ),
            ip_address_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddressType"),
            ),
            load_balancer_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loadBalancerType"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            preserve_host_header: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preserveHostHeader"),
            ),
            security_groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroups"),
            ),
            subnet_mappings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetMappings"),
            ),
            subnets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnets"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcId"),
            ),
            xff_header_processing_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("xffHeaderProcessingMode"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
