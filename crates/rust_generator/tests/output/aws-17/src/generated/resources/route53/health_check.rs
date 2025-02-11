/// Provides a Route53 health check.
///
/// ## Example Usage
///
/// ### Connectivity and HTTP Status Code Check
///
/// ```yaml
/// resources:
///   example:
///     type: aws:route53:HealthCheck
///     properties:
///       fqdn: example.com
///       port: 80
///       type: HTTP
///       resourcePath: /
///       failureThreshold: '5'
///       requestInterval: '30'
///       tags:
///         Name: tf-test-health-check
/// ```
///
/// ### Connectivity and String Matching Check
///
/// ```yaml
/// resources:
///   example:
///     type: aws:route53:HealthCheck
///     properties:
///       failureThreshold: '5'
///       fqdn: example.com
///       port: 443
///       requestInterval: '30'
///       resourcePath: /
///       searchString: example
///       type: HTTPS_STR_MATCH
/// ```
///
/// ### Aggregate Check
///
/// ```yaml
/// resources:
///   parent:
///     type: aws:route53:HealthCheck
///     properties:
///       type: CALCULATED
///       childHealthThreshold: 1
///       childHealthchecks:
///         - ${child.id}
///       tags:
///         Name: tf-test-calculated-health-check
/// ```
///
/// ### CloudWatch Alarm Check
///
/// ```yaml
/// resources:
///   foobar:
///     type: aws:cloudwatch:MetricAlarm
///     properties:
///       name: test-foobar5
///       comparisonOperator: GreaterThanOrEqualToThreshold
///       evaluationPeriods: '2'
///       metricName: CPUUtilization
///       namespace: AWS/EC2
///       period: '120'
///       statistic: Average
///       threshold: '80'
///       alarmDescription: This metric monitors ec2 cpu utilization
///   foo:
///     type: aws:route53:HealthCheck
///     properties:
///       type: CLOUDWATCH_METRIC
///       cloudwatchAlarmName: ${foobar.name}
///       cloudwatchAlarmRegion: us-west-2
///       insufficientDataHealthStatus: Healthy
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Health Checks using the health check `id`. For example:
///
/// ```sh
/// $ pulumi import aws:route53/healthCheck:HealthCheck http_check abcdef11-2222-3333-4444-555555fedcba
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod health_check {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HealthCheckArgs {
        /// The minimum number of child health checks that must be healthy for Route 53 to consider the parent health check to be healthy. Valid values are integers between 0 and 256, inclusive
        #[builder(into, default)]
        pub child_health_threshold: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// For a specified parent health check, a list of HealthCheckId values for the associated child health checks.
        #[builder(into, default)]
        pub child_healthchecks: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the CloudWatch alarm.
        #[builder(into, default)]
        pub cloudwatch_alarm_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The CloudWatchRegion that the CloudWatch alarm was created in.
        #[builder(into, default)]
        pub cloudwatch_alarm_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A boolean value that stops Route 53 from performing health checks. When set to true, Route 53 will do the following depending on the type of health check:
        /// * For health checks that check the health of endpoints, Route5 53 stops submitting requests to your application, server, or other resource.
        /// * For calculated health checks, Route 53 stops aggregating the status of the referenced health checks.
        /// * For health checks that monitor CloudWatch alarms, Route 53 stops monitoring the corresponding CloudWatch metrics.
        ///
        /// > **Note:** After you disable a health check, Route 53 considers the status of the health check to always be healthy. If you configured DNS failover, Route 53 continues to route traffic to the corresponding resources. If you want to stop routing traffic to a resource, change the value of `invert_healthcheck`.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A boolean value that indicates whether Route53 should send the `fqdn` to the endpoint when performing the health check. This defaults to AWS' defaults: when the `type` is "HTTPS" `enable_sni` defaults to `true`, when `type` is anything else `enable_sni` defaults to `false`.
        #[builder(into, default)]
        pub enable_sni: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The number of consecutive health checks that an endpoint must pass or fail.
        #[builder(into, default)]
        pub failure_threshold: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The fully qualified domain name of the endpoint to be checked. If a value is set for `ip_address`, the value set for `fqdn` will be passed in the `Host` header.
        #[builder(into, default)]
        pub fqdn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The status of the health check when CloudWatch has insufficient data about the state of associated alarm. Valid values are `Healthy` , `Unhealthy` and `LastKnownStatus`.
        #[builder(into, default)]
        pub insufficient_data_health_status: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A boolean value that indicates whether the status of health check should be inverted. For example, if a health check is healthy but Inverted is True , then Route 53 considers the health check to be unhealthy.
        #[builder(into, default)]
        pub invert_healthcheck: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The IP address of the endpoint to be checked.
        #[builder(into, default)]
        pub ip_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A Boolean value that indicates whether you want Route 53 to measure the latency between health checkers in multiple AWS regions and your endpoint and to display CloudWatch latency graphs in the Route 53 console.
        #[builder(into, default)]
        pub measure_latency: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The port of the endpoint to be checked.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// This is a reference name used in Caller Reference
        /// (helpful for identifying single health_check set amongst others)
        #[builder(into, default)]
        pub reference_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of AWS regions that you want Amazon Route 53 health checkers to check the specified endpoint from.
        #[builder(into, default)]
        pub regions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The number of seconds between the time that Amazon Route 53 gets a response from your endpoint and the time that it sends the next health-check request.
        #[builder(into, default)]
        pub request_interval: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The path that you want Amazon Route 53 to request when performing health checks.
        #[builder(into, default)]
        pub resource_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) for the Route 53 Application Recovery Controller routing control. This is used when health check type is `RECOVERY_CONTROL`
        #[builder(into, default)]
        pub routing_control_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// String searched in the first 5120 bytes of the response body for check to be considered healthy. Only valid with `HTTP_STR_MATCH` and `HTTPS_STR_MATCH`.
        #[builder(into, default)]
        pub search_string: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the health check. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The protocol to use when performing health checks. Valid values are `HTTP`, `HTTPS`, `HTTP_STR_MATCH`, `HTTPS_STR_MATCH`, `TCP`, `CALCULATED`, `CLOUDWATCH_METRIC` and `RECOVERY_CONTROL`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct HealthCheckResult {
        /// The Amazon Resource Name (ARN) of the Health Check.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The minimum number of child health checks that must be healthy for Route 53 to consider the parent health check to be healthy. Valid values are integers between 0 and 256, inclusive
        pub child_health_threshold: pulumi_gestalt_rust::Output<Option<i32>>,
        /// For a specified parent health check, a list of HealthCheckId values for the associated child health checks.
        pub child_healthchecks: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name of the CloudWatch alarm.
        pub cloudwatch_alarm_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The CloudWatchRegion that the CloudWatch alarm was created in.
        pub cloudwatch_alarm_region: pulumi_gestalt_rust::Output<Option<String>>,
        /// A boolean value that stops Route 53 from performing health checks. When set to true, Route 53 will do the following depending on the type of health check:
        /// * For health checks that check the health of endpoints, Route5 53 stops submitting requests to your application, server, or other resource.
        /// * For calculated health checks, Route 53 stops aggregating the status of the referenced health checks.
        /// * For health checks that monitor CloudWatch alarms, Route 53 stops monitoring the corresponding CloudWatch metrics.
        ///
        /// > **Note:** After you disable a health check, Route 53 considers the status of the health check to always be healthy. If you configured DNS failover, Route 53 continues to route traffic to the corresponding resources. If you want to stop routing traffic to a resource, change the value of `invert_healthcheck`.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A boolean value that indicates whether Route53 should send the `fqdn` to the endpoint when performing the health check. This defaults to AWS' defaults: when the `type` is "HTTPS" `enable_sni` defaults to `true`, when `type` is anything else `enable_sni` defaults to `false`.
        pub enable_sni: pulumi_gestalt_rust::Output<bool>,
        /// The number of consecutive health checks that an endpoint must pass or fail.
        pub failure_threshold: pulumi_gestalt_rust::Output<i32>,
        /// The fully qualified domain name of the endpoint to be checked. If a value is set for `ip_address`, the value set for `fqdn` will be passed in the `Host` header.
        pub fqdn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The status of the health check when CloudWatch has insufficient data about the state of associated alarm. Valid values are `Healthy` , `Unhealthy` and `LastKnownStatus`.
        pub insufficient_data_health_status: pulumi_gestalt_rust::Output<Option<String>>,
        /// A boolean value that indicates whether the status of health check should be inverted. For example, if a health check is healthy but Inverted is True , then Route 53 considers the health check to be unhealthy.
        pub invert_healthcheck: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The IP address of the endpoint to be checked.
        pub ip_address: pulumi_gestalt_rust::Output<Option<String>>,
        /// A Boolean value that indicates whether you want Route 53 to measure the latency between health checkers in multiple AWS regions and your endpoint and to display CloudWatch latency graphs in the Route 53 console.
        pub measure_latency: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The port of the endpoint to be checked.
        pub port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// This is a reference name used in Caller Reference
        /// (helpful for identifying single health_check set amongst others)
        pub reference_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of AWS regions that you want Amazon Route 53 health checkers to check the specified endpoint from.
        pub regions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The number of seconds between the time that Amazon Route 53 gets a response from your endpoint and the time that it sends the next health-check request.
        pub request_interval: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The path that you want Amazon Route 53 to request when performing health checks.
        pub resource_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) for the Route 53 Application Recovery Controller routing control. This is used when health check type is `RECOVERY_CONTROL`
        pub routing_control_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// String searched in the first 5120 bytes of the response body for check to be considered healthy. Only valid with `HTTP_STR_MATCH` and `HTTPS_STR_MATCH`.
        pub search_string: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to assign to the health check. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The protocol to use when performing health checks. Valid values are `HTTP`, `HTTPS`, `HTTP_STR_MATCH`, `HTTPS_STR_MATCH`, `TCP`, `CALCULATED`, `CLOUDWATCH_METRIC` and `RECOVERY_CONTROL`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HealthCheckArgs,
    ) -> HealthCheckResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let child_health_threshold_binding = args
            .child_health_threshold
            .get_output(context);
        let child_healthchecks_binding = args.child_healthchecks.get_output(context);
        let cloudwatch_alarm_name_binding = args
            .cloudwatch_alarm_name
            .get_output(context);
        let cloudwatch_alarm_region_binding = args
            .cloudwatch_alarm_region
            .get_output(context);
        let disabled_binding = args.disabled.get_output(context);
        let enable_sni_binding = args.enable_sni.get_output(context);
        let failure_threshold_binding = args.failure_threshold.get_output(context);
        let fqdn_binding = args.fqdn.get_output(context);
        let insufficient_data_health_status_binding = args
            .insufficient_data_health_status
            .get_output(context);
        let invert_healthcheck_binding = args.invert_healthcheck.get_output(context);
        let ip_address_binding = args.ip_address.get_output(context);
        let measure_latency_binding = args.measure_latency.get_output(context);
        let port_binding = args.port.get_output(context);
        let reference_name_binding = args.reference_name.get_output(context);
        let regions_binding = args.regions.get_output(context);
        let request_interval_binding = args.request_interval.get_output(context);
        let resource_path_binding = args.resource_path.get_output(context);
        let routing_control_arn_binding = args.routing_control_arn.get_output(context);
        let search_string_binding = args.search_string.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/healthCheck:HealthCheck".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "childHealthThreshold".into(),
                    value: &child_health_threshold_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "childHealthchecks".into(),
                    value: &child_healthchecks_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudwatchAlarmName".into(),
                    value: &cloudwatch_alarm_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudwatchAlarmRegion".into(),
                    value: &cloudwatch_alarm_region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableSni".into(),
                    value: &enable_sni_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "failureThreshold".into(),
                    value: &failure_threshold_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fqdn".into(),
                    value: &fqdn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "insufficientDataHealthStatus".into(),
                    value: &insufficient_data_health_status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "invertHealthcheck".into(),
                    value: &invert_healthcheck_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAddress".into(),
                    value: &ip_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "measureLatency".into(),
                    value: &measure_latency_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: &port_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "referenceName".into(),
                    value: &reference_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regions".into(),
                    value: &regions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestInterval".into(),
                    value: &request_interval_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourcePath".into(),
                    value: &resource_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routingControlArn".into(),
                    value: &routing_control_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "searchString".into(),
                    value: &search_string_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        HealthCheckResult {
            arn: o.get_field("arn"),
            child_health_threshold: o.get_field("childHealthThreshold"),
            child_healthchecks: o.get_field("childHealthchecks"),
            cloudwatch_alarm_name: o.get_field("cloudwatchAlarmName"),
            cloudwatch_alarm_region: o.get_field("cloudwatchAlarmRegion"),
            disabled: o.get_field("disabled"),
            enable_sni: o.get_field("enableSni"),
            failure_threshold: o.get_field("failureThreshold"),
            fqdn: o.get_field("fqdn"),
            insufficient_data_health_status: o.get_field("insufficientDataHealthStatus"),
            invert_healthcheck: o.get_field("invertHealthcheck"),
            ip_address: o.get_field("ipAddress"),
            measure_latency: o.get_field("measureLatency"),
            port: o.get_field("port"),
            reference_name: o.get_field("referenceName"),
            regions: o.get_field("regions"),
            request_interval: o.get_field("requestInterval"),
            resource_path: o.get_field("resourcePath"),
            routing_control_arn: o.get_field("routingControlArn"),
            search_string: o.get_field("searchString"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
        }
    }
}
