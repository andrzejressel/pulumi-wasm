#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_target_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTargetGroupArgs {
        /// Full ARN of the target group.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub load_balancing_anomaly_mitigation: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Unique name of the target group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Mapping of tags, each pair of which must exactly match a pair on the desired target group.
        ///
        /// > **NOTE:** When both `arn` and `name` are specified, `arn` takes precedence. `tags` has the lowest precedence.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetTargetGroupResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub arn_suffix: pulumi_gestalt_rust::Output<String>,
        pub connection_termination: pulumi_gestalt_rust::Output<bool>,
        pub deregistration_delay: pulumi_gestalt_rust::Output<String>,
        pub health_check: pulumi_gestalt_rust::Output<
            super::super::super::types::lb::GetTargetGroupHealthCheck,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub lambda_multi_value_headers_enabled: pulumi_gestalt_rust::Output<bool>,
        pub load_balancer_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        pub load_balancing_algorithm_type: pulumi_gestalt_rust::Output<String>,
        pub load_balancing_anomaly_mitigation: pulumi_gestalt_rust::Output<String>,
        pub load_balancing_cross_zone_enabled: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub port: pulumi_gestalt_rust::Output<i32>,
        pub preserve_client_ip: pulumi_gestalt_rust::Output<String>,
        pub protocol: pulumi_gestalt_rust::Output<String>,
        pub protocol_version: pulumi_gestalt_rust::Output<String>,
        pub proxy_protocol_v2: pulumi_gestalt_rust::Output<bool>,
        pub slow_start: pulumi_gestalt_rust::Output<i32>,
        pub stickiness: pulumi_gestalt_rust::Output<
            super::super::super::types::lb::GetTargetGroupStickiness,
        >,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub target_type: pulumi_gestalt_rust::Output<String>,
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTargetGroupArgs,
    ) -> GetTargetGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let load_balancing_anomaly_mitigation_binding = args
            .load_balancing_anomaly_mitigation
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:lb/getTargetGroup:getTargetGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancingAnomalyMitigation".into(),
                    value: load_balancing_anomaly_mitigation_binding.get_id(),
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
        GetTargetGroupResult {
            arn: o.get_field("arn"),
            arn_suffix: o.get_field("arnSuffix"),
            connection_termination: o.get_field("connectionTermination"),
            deregistration_delay: o.get_field("deregistrationDelay"),
            health_check: o.get_field("healthCheck"),
            id: o.get_field("id"),
            lambda_multi_value_headers_enabled: o
                .get_field("lambdaMultiValueHeadersEnabled"),
            load_balancer_arns: o.get_field("loadBalancerArns"),
            load_balancing_algorithm_type: o.get_field("loadBalancingAlgorithmType"),
            load_balancing_anomaly_mitigation: o
                .get_field("loadBalancingAnomalyMitigation"),
            load_balancing_cross_zone_enabled: o
                .get_field("loadBalancingCrossZoneEnabled"),
            name: o.get_field("name"),
            port: o.get_field("port"),
            preserve_client_ip: o.get_field("preserveClientIp"),
            protocol: o.get_field("protocol"),
            protocol_version: o.get_field("protocolVersion"),
            proxy_protocol_v2: o.get_field("proxyProtocolV2"),
            slow_start: o.get_field("slowStart"),
            stickiness: o.get_field("stickiness"),
            tags: o.get_field("tags"),
            target_type: o.get_field("targetType"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
