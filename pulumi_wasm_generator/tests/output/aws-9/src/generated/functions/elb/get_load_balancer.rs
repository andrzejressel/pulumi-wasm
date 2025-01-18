pub mod get_load_balancer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLoadBalancerArgs {
        /// Unique name of the load balancer.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLoadBalancerResult {
        pub access_logs: pulumi_wasm_rust::Output<
            super::super::super::types::elb::GetLoadBalancerAccessLogs,
        >,
        pub arn: pulumi_wasm_rust::Output<String>,
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        pub connection_draining: pulumi_wasm_rust::Output<bool>,
        pub connection_draining_timeout: pulumi_wasm_rust::Output<i32>,
        pub cross_zone_load_balancing: pulumi_wasm_rust::Output<bool>,
        pub desync_mitigation_mode: pulumi_wasm_rust::Output<String>,
        pub dns_name: pulumi_wasm_rust::Output<String>,
        pub health_check: pulumi_wasm_rust::Output<
            super::super::super::types::elb::GetLoadBalancerHealthCheck,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub idle_timeout: pulumi_wasm_rust::Output<i32>,
        pub instances: pulumi_wasm_rust::Output<Vec<String>>,
        pub internal: pulumi_wasm_rust::Output<bool>,
        pub listeners: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elb::GetLoadBalancerListener>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub security_groups: pulumi_wasm_rust::Output<Vec<String>>,
        pub source_security_group: pulumi_wasm_rust::Output<String>,
        pub source_security_group_id: pulumi_wasm_rust::Output<String>,
        pub subnets: pulumi_wasm_rust::Output<Vec<String>>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetLoadBalancerArgs) -> GetLoadBalancerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:elb/getLoadBalancer:getLoadBalancer".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
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
                    name: "availabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "connectionDraining".into(),
                },
                register_interface::ResultField {
                    name: "connectionDrainingTimeout".into(),
                },
                register_interface::ResultField {
                    name: "crossZoneLoadBalancing".into(),
                },
                register_interface::ResultField {
                    name: "desyncMitigationMode".into(),
                },
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "healthCheck".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "idleTimeout".into(),
                },
                register_interface::ResultField {
                    name: "instances".into(),
                },
                register_interface::ResultField {
                    name: "internal".into(),
                },
                register_interface::ResultField {
                    name: "listeners".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "securityGroups".into(),
                },
                register_interface::ResultField {
                    name: "sourceSecurityGroup".into(),
                },
                register_interface::ResultField {
                    name: "sourceSecurityGroupId".into(),
                },
                register_interface::ResultField {
                    name: "subnets".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
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
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZones").unwrap(),
            ),
            connection_draining: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionDraining").unwrap(),
            ),
            connection_draining_timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionDrainingTimeout").unwrap(),
            ),
            cross_zone_load_balancing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("crossZoneLoadBalancing").unwrap(),
            ),
            desync_mitigation_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desyncMitigationMode").unwrap(),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            health_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthCheck").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            idle_timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idleTimeout").unwrap(),
            ),
            instances: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instances").unwrap(),
            ),
            internal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internal").unwrap(),
            ),
            listeners: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listeners").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            security_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroups").unwrap(),
            ),
            source_security_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceSecurityGroup").unwrap(),
            ),
            source_security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceSecurityGroupId").unwrap(),
            ),
            subnets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnets").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
