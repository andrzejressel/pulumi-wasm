/// Provides a Target Group resource for use with Load Balancer resources.
///
/// > **Note:** `aws.alb.TargetGroup` is known as `aws.lb.TargetGroup`. The functionality is identical.
///
/// ## Example Usage
///
/// ### Instance Target Group
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = vpc::create(
///         "main",
///         VpcArgs::builder().cidr_block("10.0.0.0/16").build_struct(),
///     );
///     let test = target_group::create(
///         "test",
///         TargetGroupArgs::builder()
///             .name("tf-example-lb-tg")
///             .port(80)
///             .protocol("HTTP")
///             .vpc_id("${main.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### IP Target Group
///
/// ```yaml
/// resources:
///   ip-example:
///     type: aws:lb:TargetGroup
///     properties:
///       name: tf-example-lb-tg
///       port: 80
///       protocol: HTTP
///       targetType: ip
///       vpcId: ${main.id}
///   main:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.0.0.0/16
/// ```
///
/// ### Lambda Target Group
///
/// ```yaml
/// resources:
///   lambda-example:
///     type: aws:lb:TargetGroup
///     properties:
///       name: tf-example-lb-tg
///       targetType: lambda
/// ```
///
/// ### ALB Target Group
///
/// ```yaml
/// resources:
///   alb-example:
///     type: aws:lb:TargetGroup
///     properties:
///       name: tf-example-lb-alb-tg
///       targetType: alb
///       port: 80
///       protocol: TCP
///       vpcId: ${main.id}
/// ```
///
/// ### Target group with unhealthy connection termination disabled
///
/// ```yaml
/// resources:
///   tcp-example:
///     type: aws:lb:TargetGroup
///     properties:
///       name: tf-example-lb-nlb-tg
///       port: 25
///       protocol: TCP
///       vpcId: ${main.id}
///       targetHealthStates:
///         - enableUnhealthyConnectionTermination: false
/// ```
///
/// ### Target group with health requirements
///
/// ```yaml
/// resources:
///   tcp-example:
///     type: aws:lb:TargetGroup
///     properties:
///       name: tf-example-lb-nlb-tg
///       port: 80
///       protocol: TCP
///       vpcId: ${main.id}
///       targetGroupHealth:
///         dnsFailover:
///           minimumHealthyTargetsCount: '1'
///           minimumHealthyTargetsPercentage: off
///         unhealthyStateRouting:
///           minimumHealthyTargetsCount: '1'
///           minimumHealthyTargetsPercentage: off
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Target Groups using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:alb/targetGroup:TargetGroup app_front_end arn:aws:elasticloadbalancing:us-west-2:187416307283:targetgroup/app-front-end/20cfe21448b66314
/// ```
pub mod target_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetGroupArgs {
        /// Whether to terminate connections at the end of the deregistration timeout on Network Load Balancers. See [doc](https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-target-groups.html#deregistration-delay) for more information. Default is `false`.
        #[builder(into, default)]
        pub connection_termination: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Amount time for Elastic Load Balancing to wait before changing the state of a deregistering target from draining to unused. The range is 0-3600 seconds. The default value is 300 seconds.
        #[builder(into, default)]
        pub deregistration_delay: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Health Check configuration block. Detailed below.
        #[builder(into, default)]
        pub health_check: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::alb::TargetGroupHealthCheck>,
        >,
        /// The type of IP addresses used by the target group, only supported when target type is set to `ip`. Possible values are `ipv4` or `ipv6`.
        #[builder(into, default)]
        pub ip_address_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether the request and response headers exchanged between the load balancer and the Lambda function include arrays of values or strings. Only applies when `target_type` is `lambda`. Default is `false`.
        #[builder(into, default)]
        pub lambda_multi_value_headers_enabled: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Determines how the load balancer selects targets when routing requests. Only applicable for Application Load Balancer Target Groups. The value is `round_robin`, `least_outstanding_requests`, or `weighted_random`. The default is `round_robin`.
        #[builder(into, default)]
        pub load_balancing_algorithm_type: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Determines whether to enable target anomaly mitigation.  Target anomaly mitigation is only supported by the `weighted_random` load balancing algorithm type.  See [doc](https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-target-groups.html#automatic-target-weights) for more information.  The value is `"on"` or `"off"`. The default is `"off"`.
        #[builder(into, default)]
        pub load_balancing_anomaly_mitigation: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Indicates whether cross zone load balancing is enabled. The value is `"true"`, `"false"` or `"use_load_balancer_configuration"`. The default is `"use_load_balancer_configuration"`.
        #[builder(into, default)]
        pub load_balancing_cross_zone_enabled: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Name of the target group. If omitted, this provider will assign a random, unique name. This name must be unique per region per account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`. Cannot be longer than 6 characters.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Port on which targets receive traffic, unless overridden when registering a specific target. Required when `target_type` is `instance`, `ip` or `alb`. Does not apply when `target_type` is `lambda`.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Whether client IP preservation is enabled. See [doc](https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-target-groups.html#client-ip-preservation) for more information.
        #[builder(into, default)]
        pub preserve_client_ip: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Protocol to use for routing traffic to the targets.
        /// Should be one of `GENEVE`, `HTTP`, `HTTPS`, `TCP`, `TCP_UDP`, `TLS`, or `UDP`.
        /// Required when `target_type` is `instance`, `ip`, or `alb`.
        /// Does not apply when `target_type` is `lambda`.
        #[builder(into, default)]
        pub protocol: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Only applicable when `protocol` is `HTTP` or `HTTPS`. The protocol version. Specify `GRPC` to send requests to targets using gRPC. Specify `HTTP2` to send requests to targets using HTTP/2. The default is `HTTP1`, which sends requests to targets using HTTP/1.1
        #[builder(into, default)]
        pub protocol_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to enable support for proxy protocol v2 on Network Load Balancers. See [doc](https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-target-groups.html#proxy-protocol) for more information. Default is `false`.
        #[builder(into, default)]
        pub proxy_protocol_v2: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Amount time for targets to warm up before the load balancer sends them a full share of requests. The range is 30-900 seconds or 0 to disable. The default value is 0 seconds.
        #[builder(into, default)]
        pub slow_start: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Stickiness configuration block. Detailed below.
        #[builder(into, default)]
        pub stickiness: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::alb::TargetGroupStickiness>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Target failover block. Only applicable for Gateway Load Balancer target groups. See target_failover for more information.
        #[builder(into, default)]
        pub target_failovers: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::alb::TargetGroupTargetFailover>>,
        >,
        /// Target health requirements block. See target_group_health for more information.
        #[builder(into, default)]
        pub target_group_health: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::alb::TargetGroupTargetGroupHealth>,
        >,
        /// Target health state block. Only applicable for Network Load Balancer target groups when `protocol` is `TCP` or `TLS`. See target_health_state for more information.
        #[builder(into, default)]
        pub target_health_states: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::alb::TargetGroupTargetHealthState>>,
        >,
        /// Type of target that you must specify when registering targets with this target group.
        /// See [doc](https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_CreateTargetGroup.html) for supported values.
        /// The default is `instance`.
        ///
        /// Note that you can't specify targets for a target group using both instance IDs and IP addresses.
        ///
        /// If the target type is `ip`, specify IP addresses from the subnets of the virtual private cloud (VPC) for the target group, the RFC 1918 range (10.0.0.0/8, 172.16.0.0/12, and 192.168.0.0/16), and the RFC 6598 range (100.64.0.0/10). You can't specify publicly routable IP addresses.
        ///
        /// Network Load Balancers do not support the `lambda` target type.
        ///
        /// Application Load Balancers do not support the `alb` target type.
        #[builder(into, default)]
        pub target_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Identifier of the VPC in which to create the target group. Required when `target_type` is `instance`, `ip` or `alb`. Does not apply when `target_type` is `lambda`.
        #[builder(into, default)]
        pub vpc_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TargetGroupResult {
        /// ARN of the Target Group (matches `id`).
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN suffix for use with CloudWatch Metrics.
        pub arn_suffix: pulumi_wasm_rust::Output<String>,
        /// Whether to terminate connections at the end of the deregistration timeout on Network Load Balancers. See [doc](https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-target-groups.html#deregistration-delay) for more information. Default is `false`.
        pub connection_termination: pulumi_wasm_rust::Output<bool>,
        /// Amount time for Elastic Load Balancing to wait before changing the state of a deregistering target from draining to unused. The range is 0-3600 seconds. The default value is 300 seconds.
        pub deregistration_delay: pulumi_wasm_rust::Output<Option<i32>>,
        /// Health Check configuration block. Detailed below.
        pub health_check: pulumi_wasm_rust::Output<
            super::super::types::alb::TargetGroupHealthCheck,
        >,
        /// The type of IP addresses used by the target group, only supported when target type is set to `ip`. Possible values are `ipv4` or `ipv6`.
        pub ip_address_type: pulumi_wasm_rust::Output<String>,
        /// Whether the request and response headers exchanged between the load balancer and the Lambda function include arrays of values or strings. Only applies when `target_type` is `lambda`. Default is `false`.
        pub lambda_multi_value_headers_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// ARNs of the Load Balancers associated with the Target Group.
        pub load_balancer_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// Determines how the load balancer selects targets when routing requests. Only applicable for Application Load Balancer Target Groups. The value is `round_robin`, `least_outstanding_requests`, or `weighted_random`. The default is `round_robin`.
        pub load_balancing_algorithm_type: pulumi_wasm_rust::Output<String>,
        /// Determines whether to enable target anomaly mitigation.  Target anomaly mitigation is only supported by the `weighted_random` load balancing algorithm type.  See [doc](https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-target-groups.html#automatic-target-weights) for more information.  The value is `"on"` or `"off"`. The default is `"off"`.
        pub load_balancing_anomaly_mitigation: pulumi_wasm_rust::Output<String>,
        /// Indicates whether cross zone load balancing is enabled. The value is `"true"`, `"false"` or `"use_load_balancer_configuration"`. The default is `"use_load_balancer_configuration"`.
        pub load_balancing_cross_zone_enabled: pulumi_wasm_rust::Output<String>,
        /// Name of the target group. If omitted, this provider will assign a random, unique name. This name must be unique per region per account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`. Cannot be longer than 6 characters.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// Port on which targets receive traffic, unless overridden when registering a specific target. Required when `target_type` is `instance`, `ip` or `alb`. Does not apply when `target_type` is `lambda`.
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// Whether client IP preservation is enabled. See [doc](https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-target-groups.html#client-ip-preservation) for more information.
        pub preserve_client_ip: pulumi_wasm_rust::Output<String>,
        /// Protocol to use for routing traffic to the targets.
        /// Should be one of `GENEVE`, `HTTP`, `HTTPS`, `TCP`, `TCP_UDP`, `TLS`, or `UDP`.
        /// Required when `target_type` is `instance`, `ip`, or `alb`.
        /// Does not apply when `target_type` is `lambda`.
        pub protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// Only applicable when `protocol` is `HTTP` or `HTTPS`. The protocol version. Specify `GRPC` to send requests to targets using gRPC. Specify `HTTP2` to send requests to targets using HTTP/2. The default is `HTTP1`, which sends requests to targets using HTTP/1.1
        pub protocol_version: pulumi_wasm_rust::Output<String>,
        /// Whether to enable support for proxy protocol v2 on Network Load Balancers. See [doc](https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-target-groups.html#proxy-protocol) for more information. Default is `false`.
        pub proxy_protocol_v2: pulumi_wasm_rust::Output<Option<bool>>,
        /// Amount time for targets to warm up before the load balancer sends them a full share of requests. The range is 30-900 seconds or 0 to disable. The default value is 0 seconds.
        pub slow_start: pulumi_wasm_rust::Output<Option<i32>>,
        /// Stickiness configuration block. Detailed below.
        pub stickiness: pulumi_wasm_rust::Output<
            super::super::types::alb::TargetGroupStickiness,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Target failover block. Only applicable for Gateway Load Balancer target groups. See target_failover for more information.
        pub target_failovers: pulumi_wasm_rust::Output<
            Vec<super::super::types::alb::TargetGroupTargetFailover>,
        >,
        /// Target health requirements block. See target_group_health for more information.
        pub target_group_health: pulumi_wasm_rust::Output<
            super::super::types::alb::TargetGroupTargetGroupHealth,
        >,
        /// Target health state block. Only applicable for Network Load Balancer target groups when `protocol` is `TCP` or `TLS`. See target_health_state for more information.
        pub target_health_states: pulumi_wasm_rust::Output<
            Vec<super::super::types::alb::TargetGroupTargetHealthState>,
        >,
        /// Type of target that you must specify when registering targets with this target group.
        /// See [doc](https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_CreateTargetGroup.html) for supported values.
        /// The default is `instance`.
        ///
        /// Note that you can't specify targets for a target group using both instance IDs and IP addresses.
        ///
        /// If the target type is `ip`, specify IP addresses from the subnets of the virtual private cloud (VPC) for the target group, the RFC 1918 range (10.0.0.0/8, 172.16.0.0/12, and 192.168.0.0/16), and the RFC 6598 range (100.64.0.0/10). You can't specify publicly routable IP addresses.
        ///
        /// Network Load Balancers do not support the `lambda` target type.
        ///
        /// Application Load Balancers do not support the `alb` target type.
        pub target_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of the VPC in which to create the target group. Required when `target_type` is `instance`, `ip` or `alb`. Does not apply when `target_type` is `lambda`.
        pub vpc_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TargetGroupArgs,
    ) -> TargetGroupResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connection_termination_binding = args
            .connection_termination
            .get_output(context)
            .get_inner();
        let deregistration_delay_binding = args
            .deregistration_delay
            .get_output(context)
            .get_inner();
        let health_check_binding = args.health_check.get_output(context).get_inner();
        let ip_address_type_binding = args
            .ip_address_type
            .get_output(context)
            .get_inner();
        let lambda_multi_value_headers_enabled_binding = args
            .lambda_multi_value_headers_enabled
            .get_output(context)
            .get_inner();
        let load_balancing_algorithm_type_binding = args
            .load_balancing_algorithm_type
            .get_output(context)
            .get_inner();
        let load_balancing_anomaly_mitigation_binding = args
            .load_balancing_anomaly_mitigation
            .get_output(context)
            .get_inner();
        let load_balancing_cross_zone_enabled_binding = args
            .load_balancing_cross_zone_enabled
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let name_prefix_binding = args.name_prefix.get_output(context).get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let preserve_client_ip_binding = args
            .preserve_client_ip
            .get_output(context)
            .get_inner();
        let protocol_binding = args.protocol.get_output(context).get_inner();
        let protocol_version_binding = args
            .protocol_version
            .get_output(context)
            .get_inner();
        let proxy_protocol_v2_binding = args
            .proxy_protocol_v2
            .get_output(context)
            .get_inner();
        let slow_start_binding = args.slow_start.get_output(context).get_inner();
        let stickiness_binding = args.stickiness.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_failovers_binding = args
            .target_failovers
            .get_output(context)
            .get_inner();
        let target_group_health_binding = args
            .target_group_health
            .get_output(context)
            .get_inner();
        let target_health_states_binding = args
            .target_health_states
            .get_output(context)
            .get_inner();
        let target_type_binding = args.target_type.get_output(context).get_inner();
        let vpc_id_binding = args.vpc_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:alb/targetGroup:TargetGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionTermination".into(),
                    value: &connection_termination_binding,
                },
                register_interface::ObjectField {
                    name: "deregistrationDelay".into(),
                    value: &deregistration_delay_binding,
                },
                register_interface::ObjectField {
                    name: "healthCheck".into(),
                    value: &health_check_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddressType".into(),
                    value: &ip_address_type_binding,
                },
                register_interface::ObjectField {
                    name: "lambdaMultiValueHeadersEnabled".into(),
                    value: &lambda_multi_value_headers_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancingAlgorithmType".into(),
                    value: &load_balancing_algorithm_type_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancingAnomalyMitigation".into(),
                    value: &load_balancing_anomaly_mitigation_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancingCrossZoneEnabled".into(),
                    value: &load_balancing_cross_zone_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "preserveClientIp".into(),
                    value: &preserve_client_ip_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "protocolVersion".into(),
                    value: &protocol_version_binding,
                },
                register_interface::ObjectField {
                    name: "proxyProtocolV2".into(),
                    value: &proxy_protocol_v2_binding,
                },
                register_interface::ObjectField {
                    name: "slowStart".into(),
                    value: &slow_start_binding,
                },
                register_interface::ObjectField {
                    name: "stickiness".into(),
                    value: &stickiness_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetFailovers".into(),
                    value: &target_failovers_binding,
                },
                register_interface::ObjectField {
                    name: "targetGroupHealth".into(),
                    value: &target_group_health_binding,
                },
                register_interface::ObjectField {
                    name: "targetHealthStates".into(),
                    value: &target_health_states_binding,
                },
                register_interface::ObjectField {
                    name: "targetType".into(),
                    value: &target_type_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TargetGroupResult {
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
            ip_address_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipAddressType"),
            ),
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
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namePrefix"),
            ),
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
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            target_failovers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetFailovers"),
            ),
            target_group_health: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetGroupHealth"),
            ),
            target_health_states: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetHealthStates"),
            ),
            target_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetType"),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
