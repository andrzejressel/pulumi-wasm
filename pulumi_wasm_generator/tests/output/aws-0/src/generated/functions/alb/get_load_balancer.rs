pub mod get_load_balancer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLoadBalancerArgs {
        /// Full ARN of the load balancer.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique name of the load balancer.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Mapping of tags, each pair of which must exactly match a pair on the desired load balancer.
        ///
        /// > **NOTE:** When both `arn` and `name` are specified, `arn` takes precedence. `tags` has lowest precedence.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLoadBalancerResult {
        pub access_logs: pulumi_wasm_rust::Output<
            super::super::super::types::alb::GetLoadBalancerAccessLogs,
        >,
        pub arn: pulumi_wasm_rust::Output<String>,
        pub arn_suffix: pulumi_wasm_rust::Output<String>,
        pub client_keep_alive: pulumi_wasm_rust::Output<i32>,
        pub connection_logs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::alb::GetLoadBalancerConnectionLog>,
        >,
        pub customer_owned_ipv4_pool: pulumi_wasm_rust::Output<String>,
        pub desync_mitigation_mode: pulumi_wasm_rust::Output<String>,
        pub dns_name: pulumi_wasm_rust::Output<String>,
        pub dns_record_client_routing_policy: pulumi_wasm_rust::Output<String>,
        pub drop_invalid_header_fields: pulumi_wasm_rust::Output<bool>,
        pub enable_cross_zone_load_balancing: pulumi_wasm_rust::Output<bool>,
        pub enable_deletion_protection: pulumi_wasm_rust::Output<bool>,
        pub enable_http2: pulumi_wasm_rust::Output<bool>,
        pub enable_tls_version_and_cipher_suite_headers: pulumi_wasm_rust::Output<bool>,
        pub enable_waf_fail_open: pulumi_wasm_rust::Output<bool>,
        pub enable_xff_client_port: pulumi_wasm_rust::Output<bool>,
        pub enable_zonal_shift: pulumi_wasm_rust::Output<bool>,
        pub enforce_security_group_inbound_rules_on_private_link_traffic: pulumi_wasm_rust::Output<
            String,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub idle_timeout: pulumi_wasm_rust::Output<i32>,
        pub internal: pulumi_wasm_rust::Output<bool>,
        pub ip_address_type: pulumi_wasm_rust::Output<String>,
        pub load_balancer_type: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub preserve_host_header: pulumi_wasm_rust::Output<bool>,
        pub security_groups: pulumi_wasm_rust::Output<Vec<String>>,
        pub subnet_mappings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::alb::GetLoadBalancerSubnetMapping>,
        >,
        pub subnets: pulumi_wasm_rust::Output<Vec<String>>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub vpc_id: pulumi_wasm_rust::Output<String>,
        pub xff_header_processing_mode: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetLoadBalancerArgs) -> GetLoadBalancerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:alb/getLoadBalancer:getLoadBalancer".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessLogs".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "arnSuffix".into(),
                },
                register_interface::ResultField {
                    name: "clientKeepAlive".into(),
                },
                register_interface::ResultField {
                    name: "connectionLogs".into(),
                },
                register_interface::ResultField {
                    name: "customerOwnedIpv4Pool".into(),
                },
                register_interface::ResultField {
                    name: "desyncMitigationMode".into(),
                },
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "dnsRecordClientRoutingPolicy".into(),
                },
                register_interface::ResultField {
                    name: "dropInvalidHeaderFields".into(),
                },
                register_interface::ResultField {
                    name: "enableCrossZoneLoadBalancing".into(),
                },
                register_interface::ResultField {
                    name: "enableDeletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "enableHttp2".into(),
                },
                register_interface::ResultField {
                    name: "enableTlsVersionAndCipherSuiteHeaders".into(),
                },
                register_interface::ResultField {
                    name: "enableWafFailOpen".into(),
                },
                register_interface::ResultField {
                    name: "enableXffClientPort".into(),
                },
                register_interface::ResultField {
                    name: "enableZonalShift".into(),
                },
                register_interface::ResultField {
                    name: "enforceSecurityGroupInboundRulesOnPrivateLinkTraffic".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "idleTimeout".into(),
                },
                register_interface::ResultField {
                    name: "internal".into(),
                },
                register_interface::ResultField {
                    name: "ipAddressType".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancerType".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "preserveHostHeader".into(),
                },
                register_interface::ResultField {
                    name: "securityGroups".into(),
                },
                register_interface::ResultField {
                    name: "subnetMappings".into(),
                },
                register_interface::ResultField {
                    name: "subnets".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
                register_interface::ResultField {
                    name: "xffHeaderProcessingMode".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLoadBalancerResult {
            access_logs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessLogs").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            arn_suffix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arnSuffix").unwrap(),
            ),
            client_keep_alive: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientKeepAlive").unwrap(),
            ),
            connection_logs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionLogs").unwrap(),
            ),
            customer_owned_ipv4_pool: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerOwnedIpv4Pool").unwrap(),
            ),
            desync_mitigation_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desyncMitigationMode").unwrap(),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            dns_record_client_routing_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsRecordClientRoutingPolicy").unwrap(),
            ),
            drop_invalid_header_fields: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dropInvalidHeaderFields").unwrap(),
            ),
            enable_cross_zone_load_balancing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableCrossZoneLoadBalancing").unwrap(),
            ),
            enable_deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableDeletionProtection").unwrap(),
            ),
            enable_http2: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableHttp2").unwrap(),
            ),
            enable_tls_version_and_cipher_suite_headers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableTlsVersionAndCipherSuiteHeaders").unwrap(),
            ),
            enable_waf_fail_open: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableWafFailOpen").unwrap(),
            ),
            enable_xff_client_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableXffClientPort").unwrap(),
            ),
            enable_zonal_shift: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableZonalShift").unwrap(),
            ),
            enforce_security_group_inbound_rules_on_private_link_traffic: pulumi_wasm_rust::__private::into_domain(
                hashmap
                    .remove("enforceSecurityGroupInboundRulesOnPrivateLinkTraffic")
                    .unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            idle_timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idleTimeout").unwrap(),
            ),
            internal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internal").unwrap(),
            ),
            ip_address_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddressType").unwrap(),
            ),
            load_balancer_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancerType").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            preserve_host_header: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preserveHostHeader").unwrap(),
            ),
            security_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroups").unwrap(),
            ),
            subnet_mappings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetMappings").unwrap(),
            ),
            subnets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnets").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
            xff_header_processing_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("xffHeaderProcessingMode").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
