pub mod get_load_balancer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLoadBalancerArgs {
        /// Unique name of the load balancer.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetLoadBalancerArgs,
    ) -> GetLoadBalancerResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLoadBalancerResult {
            access_logs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accessLogs"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZones"),
            ),
            connection_draining: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionDraining"),
            ),
            connection_draining_timeout: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionDrainingTimeout"),
            ),
            cross_zone_load_balancing: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("crossZoneLoadBalancing"),
            ),
            desync_mitigation_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("desyncMitigationMode"),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            health_check: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("healthCheck"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            idle_timeout: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("idleTimeout"),
            ),
            instances: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instances"),
            ),
            internal: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("internal"),
            ),
            listeners: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("listeners"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            security_groups: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityGroups"),
            ),
            source_security_group: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceSecurityGroup"),
            ),
            source_security_group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceSecurityGroupId"),
            ),
            subnets: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnets"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            zone_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("zoneId")),
        }
    }
}
