pub mod get_target_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTargetGroupArgs {
        /// Full ARN of the target group.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub load_balancing_anomaly_mitigation: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Unique name of the target group.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Mapping of tags, each pair of which must exactly match a pair on the desired target group.
        ///
        /// > **NOTE:** When both `arn` and `name` are specified, `arn` takes precedence. `tags` has the lowest precedence.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetTargetGroupResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub arn_suffix: pulumi_wasm_rust::Output<String>,
        pub connection_termination: pulumi_wasm_rust::Output<bool>,
        pub deregistration_delay: pulumi_wasm_rust::Output<String>,
        pub health_check: pulumi_wasm_rust::Output<
            super::super::super::types::alb::GetTargetGroupHealthCheck,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub lambda_multi_value_headers_enabled: pulumi_wasm_rust::Output<bool>,
        pub load_balancer_arns: pulumi_wasm_rust::Output<Vec<String>>,
        pub load_balancing_algorithm_type: pulumi_wasm_rust::Output<String>,
        pub load_balancing_anomaly_mitigation: pulumi_wasm_rust::Output<String>,
        pub load_balancing_cross_zone_enabled: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub port: pulumi_wasm_rust::Output<i32>,
        pub preserve_client_ip: pulumi_wasm_rust::Output<String>,
        pub protocol: pulumi_wasm_rust::Output<String>,
        pub protocol_version: pulumi_wasm_rust::Output<String>,
        pub proxy_protocol_v2: pulumi_wasm_rust::Output<bool>,
        pub slow_start: pulumi_wasm_rust::Output<i32>,
        pub stickiness: pulumi_wasm_rust::Output<
            super::super::super::types::alb::GetTargetGroupStickiness,
        >,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub target_type: pulumi_wasm_rust::Output<String>,
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTargetGroupArgs,
    ) -> GetTargetGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let load_balancing_anomaly_mitigation_binding = args
            .load_balancing_anomaly_mitigation
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:alb/getTargetGroup:getTargetGroup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancingAnomalyMitigation".into(),
                    value: &load_balancing_anomaly_mitigation_binding,
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
        GetTargetGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            arn_suffix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("arnSuffix"),
            ),
            connection_termination: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionTermination"),
            ),
            deregistration_delay: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deregistrationDelay"),
            ),
            health_check: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("healthCheck"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            lambda_multi_value_headers_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lambdaMultiValueHeadersEnabled"),
            ),
            load_balancer_arns: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loadBalancerArns"),
            ),
            load_balancing_algorithm_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loadBalancingAlgorithmType"),
            ),
            load_balancing_anomaly_mitigation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loadBalancingAnomalyMitigation"),
            ),
            load_balancing_cross_zone_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loadBalancingCrossZoneEnabled"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            port: pulumi_wasm_rust::__private::into_domain(o.extract_field("port")),
            preserve_client_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("preserveClientIp"),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("protocol"),
            ),
            protocol_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("protocolVersion"),
            ),
            proxy_protocol_v2: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("proxyProtocolV2"),
            ),
            slow_start: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("slowStart"),
            ),
            stickiness: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stickiness"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            target_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetType"),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
