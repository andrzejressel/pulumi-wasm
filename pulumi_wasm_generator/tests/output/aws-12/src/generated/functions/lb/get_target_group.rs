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
            super::super::super::types::lb::GetTargetGroupHealthCheck,
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
            super::super::super::types::lb::GetTargetGroupStickiness,
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
            token: "aws:lb/getTargetGroup:getTargetGroup".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "arnSuffix".into(),
                },
                register_interface::ResultField {
                    name: "connectionTermination".into(),
                },
                register_interface::ResultField {
                    name: "deregistrationDelay".into(),
                },
                register_interface::ResultField {
                    name: "healthCheck".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lambdaMultiValueHeadersEnabled".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancerArns".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancingAlgorithmType".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancingAnomalyMitigation".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancingCrossZoneEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "preserveClientIp".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "protocolVersion".into(),
                },
                register_interface::ResultField {
                    name: "proxyProtocolV2".into(),
                },
                register_interface::ResultField {
                    name: "slowStart".into(),
                },
                register_interface::ResultField {
                    name: "stickiness".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "targetType".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTargetGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            arn_suffix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arnSuffix").unwrap(),
            ),
            connection_termination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionTermination").unwrap(),
            ),
            deregistration_delay: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deregistrationDelay").unwrap(),
            ),
            health_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthCheck").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            lambda_multi_value_headers_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lambdaMultiValueHeadersEnabled").unwrap(),
            ),
            load_balancer_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancerArns").unwrap(),
            ),
            load_balancing_algorithm_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancingAlgorithmType").unwrap(),
            ),
            load_balancing_anomaly_mitigation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancingAnomalyMitigation").unwrap(),
            ),
            load_balancing_cross_zone_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancingCrossZoneEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            preserve_client_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preserveClientIp").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            protocol_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocolVersion").unwrap(),
            ),
            proxy_protocol_v2: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyProtocolV2").unwrap(),
            ),
            slow_start: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("slowStart").unwrap(),
            ),
            stickiness: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stickiness").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            target_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetType").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
