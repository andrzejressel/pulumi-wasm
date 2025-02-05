/// Provides an AutoScaling Scaling Policy resource.
///
/// > **NOTE:** You may want to omit `desired_capacity` attribute from attached `aws.autoscaling.Group`
/// when using autoscaling policies. It's good practice to pick either
/// [manual](https://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/as-manual-scaling.html)
/// or [dynamic](https://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/as-scale-based-on-demand.html)
/// (policy-based) scaling.
///
///
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bar = group::create(
///         "bar",
///         GroupArgs::builder()
///             .availability_zones(vec!["us-east-1a",])
///             .force_delete(true)
///             .health_check_grace_period(300)
///             .health_check_type("ELB")
///             .launch_configuration("${foo.name}")
///             .max_size(5)
///             .min_size(2)
///             .name("foobar3-test")
///             .build_struct(),
///     );
///     let bat = policy::create(
///         "bat",
///         PolicyArgs::builder()
///             .adjustment_type("ChangeInCapacity")
///             .autoscaling_group_name("${bar.name}")
///             .cooldown(300)
///             .name("foobar3-test")
///             .scaling_adjustment(4)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Create target tracking scaling policy using metric math
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = policy::create(
///         "example",
///         PolicyArgs::builder()
///             .autoscaling_group_name("my-test-asg")
///             .name("foo")
///             .policy_type("TargetTrackingScaling")
///             .target_tracking_configuration(
///                 PolicyTargetTrackingConfiguration::builder()
///                     .customizedMetricSpecification(
///                         PolicyTargetTrackingConfigurationCustomizedMetricSpecification::builder()
///                             .metrics(
///                                 vec![
///                                     PolicyTargetTrackingConfigurationCustomizedMetricSpecificationMetric::builder()
///                                     .id("m1")
///                                     .label("Get the queue size (the number of messages waiting to be processed)")
///                                     .metricStat(PolicyTargetTrackingConfigurationCustomizedMetricSpecificationMetricMetricStat::builder()
///                                     .metric(PolicyTargetTrackingConfigurationCustomizedMetricSpecificationMetricMetricStatMetric::builder()
///                                     .dimensions(vec![PolicyTargetTrackingConfigurationCustomizedMetricSpecificationMetricMetricStatMetricDimension::builder()
///                                     .name("QueueName").value("my-queue").build_struct(),])
///                                     .metricName("ApproximateNumberOfMessagesVisible")
///                                     .namespace("AWS/SQS").build_struct()).stat("Sum")
///                                     .build_struct()).returnData(false).build_struct(),
///                                     PolicyTargetTrackingConfigurationCustomizedMetricSpecificationMetric::builder()
///                                     .id("m2")
///                                     .label("Get the group size (the number of InService instances)")
///                                     .metricStat(PolicyTargetTrackingConfigurationCustomizedMetricSpecificationMetricMetricStat::builder()
///                                     .metric(PolicyTargetTrackingConfigurationCustomizedMetricSpecificationMetricMetricStatMetric::builder()
///                                     .dimensions(vec![PolicyTargetTrackingConfigurationCustomizedMetricSpecificationMetricMetricStatMetricDimension::builder()
///                                     .name("AutoScalingGroupName").value("my-asg")
///                                     .build_struct(),]).metricName("GroupInServiceInstances")
///                                     .namespace("AWS/AutoScaling").build_struct())
///                                     .stat("Average").build_struct()).returnData(false)
///                                     .build_struct(),
///                                     PolicyTargetTrackingConfigurationCustomizedMetricSpecificationMetric::builder()
///                                     .expression("m1 / m2").id("e1")
///                                     .label("Calculate the backlog per instance")
///                                     .returnData(true).build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .targetValue(100)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Create predictive scaling policy using customized metrics
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = policy::create(
///         "example",
///         PolicyArgs::builder()
///             .autoscaling_group_name("my-test-asg")
///             .name("foo")
///             .policy_type("PredictiveScaling")
///             .predictive_scaling_configuration(
///                 PolicyPredictiveScalingConfiguration::builder()
///                     .metricSpecification(
///                         PolicyPredictiveScalingConfigurationMetricSpecification::builder()
///                             .customizedCapacityMetricSpecification(
///                                 PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedCapacityMetricSpecification::builder()
///                                     .metricDataQueries(
///                                         vec![
///                                             PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedCapacityMetricSpecificationMetricDataQuery::builder()
///                                             .expression("SUM(SEARCH('{AWS/AutoScaling,AutoScalingGroupName} MetricName=\"GroupInServiceIntances\" my-test-asg', 'Average', 300))")
///                                             .id("capacity_sum").build_struct(),
///                                         ],
///                                     )
///                                     .build_struct(),
///                             )
///                             .customizedLoadMetricSpecification(
///                                 PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedLoadMetricSpecification::builder()
///                                     .metricDataQueries(
///                                         vec![
///                                             PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedLoadMetricSpecificationMetricDataQuery::builder()
///                                             .expression("SUM(SEARCH('{AWS/EC2,AutoScalingGroupName} MetricName=\"CPUUtilization\" my-test-asg', 'Sum', 3600))")
///                                             .id("load_sum").build_struct(),
///                                         ],
///                                     )
///                                     .build_struct(),
///                             )
///                             .customizedScalingMetricSpecification(
///                                 PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedScalingMetricSpecification::builder()
///                                     .metricDataQueries(
///                                         vec![
///                                             PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedScalingMetricSpecificationMetricDataQuery::builder()
///                                             .expression("SUM(SEARCH('{AWS/AutoScaling,AutoScalingGroupName} MetricName=\"GroupInServiceIntances\" my-test-asg', 'Average', 300))")
///                                             .id("capacity_sum").returnData(false).build_struct(),
///                                             PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedScalingMetricSpecificationMetricDataQuery::builder()
///                                             .expression("SUM(SEARCH('{AWS/EC2,AutoScalingGroupName} MetricName=\"CPUUtilization\" my-test-asg', 'Sum', 300))")
///                                             .id("load_sum").returnData(false).build_struct(),
///                                             PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedScalingMetricSpecificationMetricDataQuery::builder()
///                                             .expression("load_sum / (capacity_sum * PERIOD(capacity_sum) / 60)")
///                                             .id("weighted_average").build_struct(),
///                                         ],
///                                     )
///                                     .build_struct(),
///                             )
///                             .targetValue(10)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Create predictive scaling policy using customized scaling and predefined load metric
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = policy::create(
///         "example",
///         PolicyArgs::builder()
///             .autoscaling_group_name("my-test-asg")
///             .name("foo")
///             .policy_type("PredictiveScaling")
///             .predictive_scaling_configuration(
///                 PolicyPredictiveScalingConfiguration::builder()
///                     .metricSpecification(
///                         PolicyPredictiveScalingConfigurationMetricSpecification::builder()
///                             .customizedScalingMetricSpecification(
///                                 PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedScalingMetricSpecification::builder()
///                                     .metricDataQueries(
///                                         vec![
///                                             PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedScalingMetricSpecificationMetricDataQuery::builder()
///                                             .id("scaling")
///                                             .metricStat(PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedScalingMetricSpecificationMetricDataQueryMetricStat::builder()
///                                             .metric(PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedScalingMetricSpecificationMetricDataQueryMetricStatMetric::builder()
///                                             .dimensions(vec![PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedScalingMetricSpecificationMetricDataQueryMetricStatMetricDimension::builder()
///                                             .name("AutoScalingGroupName").value("my-test-asg")
///                                             .build_struct(),]).metricName("CPUUtilization")
///                                             .namespace("AWS/EC2").build_struct()).stat("Average")
///                                             .build_struct()).build_struct(),
///                                         ],
///                                     )
///                                     .build_struct(),
///                             )
///                             .predefinedLoadMetricSpecification(
///                                 PolicyPredictiveScalingConfigurationMetricSpecificationPredefinedLoadMetricSpecification::builder()
///                                     .predefinedMetricType("ASGTotalCPUUtilization")
///                                     .resourceLabel(
///                                         "app/my-alb/778d41231b141a0f/targetgroup/my-alb-target-group/943f017f100becff",
///                                     )
///                                     .build_struct(),
///                             )
///                             .targetValue(10)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AutoScaling scaling policy using the role autoscaling_group_name and name separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:autoscaling/policy:Policy test-policy asg-name/policy-name
/// ```
pub mod policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// Whether the adjustment is an absolute number or a percentage of the current capacity. Valid values are `ChangeInCapacity`, `ExactCapacity`, and `PercentChangeInCapacity`.
        #[builder(into, default)]
        pub adjustment_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the autoscaling group.
        #[builder(into)]
        pub autoscaling_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Amount of time, in seconds, after a scaling activity completes and before the next scaling activity can start.
        #[builder(into, default)]
        pub cooldown: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Whether the scaling policy is enabled or disabled. Default: `true`.
        ///
        /// The following argument is only available to "SimpleScaling" and "StepScaling" type policies:
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Estimated time, in seconds, until a newly launched instance will contribute CloudWatch metrics. Without a value, AWS will default to the group's specified cooldown period.
        #[builder(into, default)]
        pub estimated_instance_warmup: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Aggregation type for the policy's metrics. Valid values are "Minimum", "Maximum", and "Average". Without a value, AWS will treat the aggregation type as "Average".
        #[builder(into, default)]
        pub metric_aggregation_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Minimum value to scale by when `adjustment_type` is set to `PercentChangeInCapacity`.
        ///
        /// The following arguments are only available to "SimpleScaling" type policies:
        #[builder(into, default)]
        pub min_adjustment_magnitude: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Name of the policy.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Policy type, either "SimpleScaling", "StepScaling", "TargetTrackingScaling", or "PredictiveScaling". If this value isn't provided, AWS will default to "SimpleScaling."
        #[builder(into, default)]
        pub policy_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Predictive scaling policy configuration to use with Amazon EC2 Auto Scaling.
        #[builder(into, default)]
        pub predictive_scaling_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::autoscaling::PolicyPredictiveScalingConfiguration,
            >,
        >,
        /// Number of members by which to
        /// scale, when the adjustment bounds are breached. A positive value scales
        /// up. A negative value scales down.
        #[builder(into, default)]
        pub scaling_adjustment: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Set of adjustments that manage
        /// group scaling. These have the following structure:
        ///
        /// ```yaml
        /// resources:
        ///   example:
        ///     type: aws:autoscaling:Policy
        ///     properties:
        ///       stepAdjustments:
        ///         - scalingAdjustment: -1
        ///           metricIntervalLowerBound: 1
        ///           metricIntervalUpperBound: 2
        ///         - scalingAdjustment: 1
        ///           metricIntervalLowerBound: 2
        ///           metricIntervalUpperBound: 3
        /// ```
        ///
        /// The following fields are available in step adjustments:
        #[builder(into, default)]
        pub step_adjustments: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::autoscaling::PolicyStepAdjustment>>,
        >,
        /// Target tracking policy. These have the following structure:
        ///
        /// ```ignore
        /// use pulumi_wasm_rust::Output;
        /// use pulumi_wasm_rust::{add_export, pulumi_main};
        /// #[pulumi_main]
        /// fn test_main() -> Result<(), Error> {
        ///     let example = policy::create(
        ///         "example",
        ///         PolicyArgs::builder()
        ///             .target_tracking_configuration(
        ///                 PolicyTargetTrackingConfiguration::builder()
        ///                     .predefinedMetricSpecification(
        ///                         PolicyTargetTrackingConfigurationPredefinedMetricSpecification::builder()
        ///                             .predefinedMetricType("ASGAverageCPUUtilization")
        ///                             .build_struct(),
        ///                     )
        ///                     .targetValue(40)
        ///                     .build_struct(),
        ///             )
        ///             .build_struct(),
        ///     );
        /// }
        /// ```
        ///
        /// The following fields are available in target tracking configuration:
        #[builder(into, default)]
        pub target_tracking_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::autoscaling::PolicyTargetTrackingConfiguration>,
        >,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// Whether the adjustment is an absolute number or a percentage of the current capacity. Valid values are `ChangeInCapacity`, `ExactCapacity`, and `PercentChangeInCapacity`.
        pub adjustment_type: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN assigned by AWS to the scaling policy.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of the autoscaling group.
        pub autoscaling_group_name: pulumi_wasm_rust::Output<String>,
        /// Amount of time, in seconds, after a scaling activity completes and before the next scaling activity can start.
        pub cooldown: pulumi_wasm_rust::Output<Option<i32>>,
        /// Whether the scaling policy is enabled or disabled. Default: `true`.
        ///
        /// The following argument is only available to "SimpleScaling" and "StepScaling" type policies:
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Estimated time, in seconds, until a newly launched instance will contribute CloudWatch metrics. Without a value, AWS will default to the group's specified cooldown period.
        pub estimated_instance_warmup: pulumi_wasm_rust::Output<Option<i32>>,
        /// Aggregation type for the policy's metrics. Valid values are "Minimum", "Maximum", and "Average". Without a value, AWS will treat the aggregation type as "Average".
        pub metric_aggregation_type: pulumi_wasm_rust::Output<String>,
        /// Minimum value to scale by when `adjustment_type` is set to `PercentChangeInCapacity`.
        ///
        /// The following arguments are only available to "SimpleScaling" type policies:
        pub min_adjustment_magnitude: pulumi_wasm_rust::Output<Option<i32>>,
        /// Name of the policy.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Policy type, either "SimpleScaling", "StepScaling", "TargetTrackingScaling", or "PredictiveScaling". If this value isn't provided, AWS will default to "SimpleScaling."
        pub policy_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Predictive scaling policy configuration to use with Amazon EC2 Auto Scaling.
        pub predictive_scaling_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::autoscaling::PolicyPredictiveScalingConfiguration,
            >,
        >,
        /// Number of members by which to
        /// scale, when the adjustment bounds are breached. A positive value scales
        /// up. A negative value scales down.
        pub scaling_adjustment: pulumi_wasm_rust::Output<Option<i32>>,
        /// Set of adjustments that manage
        /// group scaling. These have the following structure:
        ///
        /// ```yaml
        /// resources:
        ///   example:
        ///     type: aws:autoscaling:Policy
        ///     properties:
        ///       stepAdjustments:
        ///         - scalingAdjustment: -1
        ///           metricIntervalLowerBound: 1
        ///           metricIntervalUpperBound: 2
        ///         - scalingAdjustment: 1
        ///           metricIntervalLowerBound: 2
        ///           metricIntervalUpperBound: 3
        /// ```
        ///
        /// The following fields are available in step adjustments:
        pub step_adjustments: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::autoscaling::PolicyStepAdjustment>>,
        >,
        /// Target tracking policy. These have the following structure:
        ///
        /// ```ignore
        /// use pulumi_wasm_rust::Output;
        /// use pulumi_wasm_rust::{add_export, pulumi_main};
        /// #[pulumi_main]
        /// fn test_main() -> Result<(), Error> {
        ///     let example = policy::create(
        ///         "example",
        ///         PolicyArgs::builder()
        ///             .target_tracking_configuration(
        ///                 PolicyTargetTrackingConfiguration::builder()
        ///                     .predefinedMetricSpecification(
        ///                         PolicyTargetTrackingConfigurationPredefinedMetricSpecification::builder()
        ///                             .predefinedMetricType("ASGAverageCPUUtilization")
        ///                             .build_struct(),
        ///                     )
        ///                     .targetValue(40)
        ///                     .build_struct(),
        ///             )
        ///             .build_struct(),
        ///     );
        /// }
        /// ```
        ///
        /// The following fields are available in target tracking configuration:
        pub target_tracking_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::autoscaling::PolicyTargetTrackingConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PolicyArgs,
    ) -> PolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let adjustment_type_binding = args
            .adjustment_type
            .get_output(context)
            .get_inner();
        let autoscaling_group_name_binding = args
            .autoscaling_group_name
            .get_output(context)
            .get_inner();
        let cooldown_binding = args.cooldown.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let estimated_instance_warmup_binding = args
            .estimated_instance_warmup
            .get_output(context)
            .get_inner();
        let metric_aggregation_type_binding = args
            .metric_aggregation_type
            .get_output(context)
            .get_inner();
        let min_adjustment_magnitude_binding = args
            .min_adjustment_magnitude
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let policy_type_binding = args.policy_type.get_output(context).get_inner();
        let predictive_scaling_configuration_binding = args
            .predictive_scaling_configuration
            .get_output(context)
            .get_inner();
        let scaling_adjustment_binding = args
            .scaling_adjustment
            .get_output(context)
            .get_inner();
        let step_adjustments_binding = args
            .step_adjustments
            .get_output(context)
            .get_inner();
        let target_tracking_configuration_binding = args
            .target_tracking_configuration
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:autoscaling/policy:Policy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "adjustmentType".into(),
                    value: &adjustment_type_binding,
                },
                register_interface::ObjectField {
                    name: "autoscalingGroupName".into(),
                    value: &autoscaling_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "cooldown".into(),
                    value: &cooldown_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "estimatedInstanceWarmup".into(),
                    value: &estimated_instance_warmup_binding,
                },
                register_interface::ObjectField {
                    name: "metricAggregationType".into(),
                    value: &metric_aggregation_type_binding,
                },
                register_interface::ObjectField {
                    name: "minAdjustmentMagnitude".into(),
                    value: &min_adjustment_magnitude_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policyType".into(),
                    value: &policy_type_binding,
                },
                register_interface::ObjectField {
                    name: "predictiveScalingConfiguration".into(),
                    value: &predictive_scaling_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "scalingAdjustment".into(),
                    value: &scaling_adjustment_binding,
                },
                register_interface::ObjectField {
                    name: "stepAdjustments".into(),
                    value: &step_adjustments_binding,
                },
                register_interface::ObjectField {
                    name: "targetTrackingConfiguration".into(),
                    value: &target_tracking_configuration_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PolicyResult {
            adjustment_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adjustmentType"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            autoscaling_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoscalingGroupName"),
            ),
            cooldown: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cooldown"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            estimated_instance_warmup: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("estimatedInstanceWarmup"),
            ),
            metric_aggregation_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metricAggregationType"),
            ),
            min_adjustment_magnitude: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("minAdjustmentMagnitude"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            policy_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyType"),
            ),
            predictive_scaling_configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("predictiveScalingConfiguration"),
            ),
            scaling_adjustment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scalingAdjustment"),
            ),
            step_adjustments: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stepAdjustments"),
            ),
            target_tracking_configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetTrackingConfiguration"),
            ),
        }
    }
}
