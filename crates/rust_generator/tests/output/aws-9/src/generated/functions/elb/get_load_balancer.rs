#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_load_balancer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLoadBalancerArgs {
        /// Unique name of the load balancer.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLoadBalancerResult {
        pub access_logs: pulumi_gestalt_rust::Output<
            super::super::super::types::elb::GetLoadBalancerAccessLogs,
        >,
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        pub connection_draining: pulumi_gestalt_rust::Output<bool>,
        pub connection_draining_timeout: pulumi_gestalt_rust::Output<i32>,
        pub cross_zone_load_balancing: pulumi_gestalt_rust::Output<bool>,
        pub desync_mitigation_mode: pulumi_gestalt_rust::Output<String>,
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        pub health_check: pulumi_gestalt_rust::Output<
            super::super::super::types::elb::GetLoadBalancerHealthCheck,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub idle_timeout: pulumi_gestalt_rust::Output<i32>,
        pub instances: pulumi_gestalt_rust::Output<Vec<String>>,
        pub internal: pulumi_gestalt_rust::Output<bool>,
        pub listeners: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elb::GetLoadBalancerListener>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        pub source_security_group: pulumi_gestalt_rust::Output<String>,
        pub source_security_group_id: pulumi_gestalt_rust::Output<String>,
        pub subnets: pulumi_gestalt_rust::Output<Vec<String>>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
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
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
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
            access_logs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessLogs"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            availability_zones: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZones"),
            ),
            connection_draining: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionDraining"),
            ),
            connection_draining_timeout: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionDrainingTimeout"),
            ),
            cross_zone_load_balancing: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("crossZoneLoadBalancing"),
            ),
            desync_mitigation_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("desyncMitigationMode"),
            ),
            dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            health_check: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("healthCheck"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            idle_timeout: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("idleTimeout"),
            ),
            instances: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instances"),
            ),
            internal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("internal"),
            ),
            listeners: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("listeners"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            security_groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroups"),
            ),
            source_security_group: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceSecurityGroup"),
            ),
            source_security_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceSecurityGroupId"),
            ),
            subnets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnets"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
