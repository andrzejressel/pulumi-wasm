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
            super::super::super::types::alb::GetLoadBalancerAccessLogs,
        >,
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub arn_suffix: pulumi_gestalt_rust::Output<String>,
        pub client_keep_alive: pulumi_gestalt_rust::Output<i32>,
        pub connection_logs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::alb::GetLoadBalancerConnectionLog>,
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
            Vec<super::super::super::types::alb::GetLoadBalancerSubnetMapping>,
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
        context: &pulumi_gestalt_rust::Context,
        args: GetLoadBalancerArgs,
    ) -> GetLoadBalancerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:alb/getLoadBalancer:getLoadBalancer".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLoadBalancerResult {
            access_logs: o.get_field("accessLogs"),
            arn: o.get_field("arn"),
            arn_suffix: o.get_field("arnSuffix"),
            client_keep_alive: o.get_field("clientKeepAlive"),
            connection_logs: o.get_field("connectionLogs"),
            customer_owned_ipv4_pool: o.get_field("customerOwnedIpv4Pool"),
            desync_mitigation_mode: o.get_field("desyncMitigationMode"),
            dns_name: o.get_field("dnsName"),
            dns_record_client_routing_policy: o
                .get_field("dnsRecordClientRoutingPolicy"),
            drop_invalid_header_fields: o.get_field("dropInvalidHeaderFields"),
            enable_cross_zone_load_balancing: o
                .get_field("enableCrossZoneLoadBalancing"),
            enable_deletion_protection: o.get_field("enableDeletionProtection"),
            enable_http2: o.get_field("enableHttp2"),
            enable_tls_version_and_cipher_suite_headers: o
                .get_field("enableTlsVersionAndCipherSuiteHeaders"),
            enable_waf_fail_open: o.get_field("enableWafFailOpen"),
            enable_xff_client_port: o.get_field("enableXffClientPort"),
            enable_zonal_shift: o.get_field("enableZonalShift"),
            enforce_security_group_inbound_rules_on_private_link_traffic: o
                .get_field("enforceSecurityGroupInboundRulesOnPrivateLinkTraffic"),
            id: o.get_field("id"),
            idle_timeout: o.get_field("idleTimeout"),
            internal: o.get_field("internal"),
            ip_address_type: o.get_field("ipAddressType"),
            load_balancer_type: o.get_field("loadBalancerType"),
            name: o.get_field("name"),
            preserve_host_header: o.get_field("preserveHostHeader"),
            security_groups: o.get_field("securityGroups"),
            subnet_mappings: o.get_field("subnetMappings"),
            subnets: o.get_field("subnets"),
            tags: o.get_field("tags"),
            vpc_id: o.get_field("vpcId"),
            xff_header_processing_mode: o.get_field("xffHeaderProcessingMode"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
