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
        context: &pulumi_gestalt_rust::Context,
        args: GetLoadBalancerArgs,
    ) -> GetLoadBalancerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:elb/getLoadBalancer:getLoadBalancer".into(),
            version: super::super::super::get_version(),
            object: &[
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
            availability_zones: o.get_field("availabilityZones"),
            connection_draining: o.get_field("connectionDraining"),
            connection_draining_timeout: o.get_field("connectionDrainingTimeout"),
            cross_zone_load_balancing: o.get_field("crossZoneLoadBalancing"),
            desync_mitigation_mode: o.get_field("desyncMitigationMode"),
            dns_name: o.get_field("dnsName"),
            health_check: o.get_field("healthCheck"),
            id: o.get_field("id"),
            idle_timeout: o.get_field("idleTimeout"),
            instances: o.get_field("instances"),
            internal: o.get_field("internal"),
            listeners: o.get_field("listeners"),
            name: o.get_field("name"),
            security_groups: o.get_field("securityGroups"),
            source_security_group: o.get_field("sourceSecurityGroup"),
            source_security_group_id: o.get_field("sourceSecurityGroupId"),
            subnets: o.get_field("subnets"),
            tags: o.get_field("tags"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
