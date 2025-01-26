/// Provides a CloudWatch Metric Alarm resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foobar = metric_alarm::create(
///         "foobar",
///         MetricAlarmArgs::builder()
///             .alarm_description("This metric monitors ec2 cpu utilization")
///             .comparison_operator("GreaterThanOrEqualToThreshold")
///             .evaluation_periods(2)
///             .insufficient_data_actions(vec![])
///             .metric_name("CPUUtilization")
///             .name("test-foobar5")
///             .namespace("AWS/EC2")
///             .period(120)
///             .statistic("Average")
///             .threshold(80)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Example in Conjunction with Scaling Policies
///
/// ```yaml
/// resources:
///   bat:
///     type: aws:autoscaling:Policy
///     properties:
///       name: foobar3-test
///       scalingAdjustment: 4
///       adjustmentType: ChangeInCapacity
///       cooldown: 300
///       autoscalingGroupName: ${bar.name}
///   batMetricAlarm:
///     type: aws:cloudwatch:MetricAlarm
///     name: bat
///     properties:
///       name: test-foobar5
///       comparisonOperator: GreaterThanOrEqualToThreshold
///       evaluationPeriods: 2
///       metricName: CPUUtilization
///       namespace: AWS/EC2
///       period: 120
///       statistic: Average
///       threshold: 80
///       dimensions:
///         AutoScalingGroupName: ${bar.name}
///       alarmDescription: This metric monitors ec2 cpu utilization
///       alarmActions:
///         - ${bat.arn}
/// ```
///
/// ## Example with an Expression
///
/// ```yaml
/// resources:
///   foobar:
///     type: aws:cloudwatch:MetricAlarm
///     properties:
///       name: test-foobar
///       comparisonOperator: GreaterThanOrEqualToThreshold
///       evaluationPeriods: 2
///       threshold: 10
///       alarmDescription: Request error rate has exceeded 10%
///       insufficientDataActions: []
///       metricQueries:
///         - id: e1
///           expression: m2/m1*100
///           label: Error Rate
///           returnData: 'true'
///         - id: m1
///           metric:
///             metricName: RequestCount
///             namespace: AWS/ApplicationELB
///             period: 120
///             stat: Sum
///             unit: Count
///             dimensions:
///               LoadBalancer: app/web
///         - id: m2
///           metric:
///             metricName: HTTPCode_ELB_5XX_Count
///             namespace: AWS/ApplicationELB
///             period: 120
///             stat: Sum
///             unit: Count
///             dimensions:
///               LoadBalancer: app/web
/// ```
///
/// ```yaml
/// resources:
///   xxAnomalyDetection:
///     type: aws:cloudwatch:MetricAlarm
///     name: xx_anomaly_detection
///     properties:
///       name: test-foobar
///       comparisonOperator: GreaterThanUpperThreshold
///       evaluationPeriods: 2
///       thresholdMetricId: e1
///       alarmDescription: This metric monitors ec2 cpu utilization
///       insufficientDataActions: []
///       metricQueries:
///         - id: e1
///           expression: ANOMALY_DETECTION_BAND(m1)
///           label: CPUUtilization (Expected)
///           returnData: 'true'
///         - id: m1
///           returnData: 'true'
///           metric:
///             metricName: CPUUtilization
///             namespace: AWS/EC2
///             period: 120
///             stat: Average
///             unit: Count
///             dimensions:
///               InstanceId: i-abc123
/// ```
///
/// ## Example of monitoring Healthy Hosts on NLB using Target Group and NLB
///
/// ```yaml
/// resources:
///   nlbHealthyhosts:
///     type: aws:cloudwatch:MetricAlarm
///     name: nlb_healthyhosts
///     properties:
///       name: alarmname
///       comparisonOperator: LessThanThreshold
///       evaluationPeriods: 1
///       metricName: HealthyHostCount
///       namespace: AWS/NetworkELB
///       period: 60
///       statistic: Average
///       threshold: ${logstashServersCount}
///       alarmDescription: Number of healthy nodes in Target Group
///       actionsEnabled: 'true'
///       alarmActions:
///         - ${sns.arn}
///       okActions:
///         - ${sns.arn}
///       dimensions:
///         TargetGroup: ${["lb-tg"].arnSuffix}
///         LoadBalancer: ${lb.arnSuffix}
/// ```
///
/// > **NOTE:**  You cannot create a metric alarm consisting of both `statistic` and `extended_statistic` parameters.
/// You must choose one or the other
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Metric Alarm using the `alarm_name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/metricAlarm:MetricAlarm test alarm-12345
/// ```
pub mod metric_alarm {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MetricAlarmArgs {
        /// Indicates whether or not actions should be executed during any changes to the alarm's state. Defaults to `true`.
        #[builder(into, default)]
        pub actions_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The list of actions to execute when this alarm transitions into an ALARM state from any other state. Each action is specified as an Amazon Resource Name (ARN).
        #[builder(into, default)]
        pub alarm_actions: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The description for the alarm.
        #[builder(into, default)]
        pub alarm_description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The arithmetic operation to use when comparing the specified Statistic and Threshold. The specified Statistic value is used as the first operand. Either of the following is supported: `GreaterThanOrEqualToThreshold`, `GreaterThanThreshold`, `LessThanThreshold`, `LessThanOrEqualToThreshold`. Additionally, the values  `LessThanLowerOrGreaterThanUpperThreshold`, `LessThanLowerThreshold`, and `GreaterThanUpperThreshold` are used only for alarms based on anomaly detection models.
        #[builder(into)]
        pub comparison_operator: pulumi_wasm_rust::InputOrOutput<String>,
        /// The number of datapoints that must be breaching to trigger the alarm.
        #[builder(into, default)]
        pub datapoints_to_alarm: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The dimensions for the alarm's associated metric.  For the list of available dimensions see the AWS documentation [here](http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/CW_Support_For_AWS.html).
        #[builder(into, default)]
        pub dimensions: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Used only for alarms based on percentiles.
        /// If you specify `ignore`, the alarm state will not change during periods with too few data points to be statistically significant.
        /// If you specify `evaluate` or omit this parameter, the alarm will always be evaluated and possibly change state no matter how many data points are available.
        /// The following values are supported: `ignore`, and `evaluate`.
        #[builder(into, default)]
        pub evaluate_low_sample_count_percentiles: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The number of periods over which data is compared to the specified threshold.
        #[builder(into)]
        pub evaluation_periods: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The percentile statistic for the metric associated with the alarm. Specify a value between p0.0 and p100.
        #[builder(into, default)]
        pub extended_statistic: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The list of actions to execute when this alarm transitions into an INSUFFICIENT_DATA state from any other state. Each action is specified as an Amazon Resource Name (ARN).
        #[builder(into, default)]
        pub insufficient_data_actions: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The name for the alarm's associated metric.
        /// See docs for [supported metrics](https://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/CW_Support_For_AWS.html).
        #[builder(into, default)]
        pub metric_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Enables you to create an alarm based on a metric math expression. You may specify at most 20.
        #[builder(into, default)]
        pub metric_queries: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::cloudwatch::MetricAlarmMetricQuery>>,
        >,
        /// The descriptive name for the alarm. This name must be unique within the user's AWS account
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The namespace for the alarm's associated metric. See docs for the [list of namespaces](https://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/aws-namespaces.html).
        /// See docs for [supported metrics](https://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/CW_Support_For_AWS.html).
        #[builder(into, default)]
        pub namespace: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The list of actions to execute when this alarm transitions into an OK state from any other state. Each action is specified as an Amazon Resource Name (ARN).
        #[builder(into, default)]
        pub ok_actions: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The period in seconds over which the specified `statistic` is applied.
        /// Valid values are `10`, `30`, or any multiple of `60`.
        #[builder(into, default)]
        pub period: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The statistic to apply to the alarm's associated metric.
        /// Either of the following is supported: `SampleCount`, `Average`, `Sum`, `Minimum`, `Maximum`
        #[builder(into, default)]
        pub statistic: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// > **NOTE:**  If you specify at least one `metric_query`, you may not specify a `metric_name`, `namespace`, `period` or `statistic`. If you do not specify a `metric_query`, you must specify each of these (although you may use `extended_statistic` instead of `statistic`).
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The value against which the specified statistic is compared. This parameter is required for alarms based on static thresholds, but should not be used for alarms based on anomaly detection models.
        #[builder(into, default)]
        pub threshold: pulumi_wasm_rust::InputOrOutput<Option<f64>>,
        /// If this is an alarm based on an anomaly detection model, make this value match the ID of the ANOMALY_DETECTION_BAND function.
        #[builder(into, default)]
        pub threshold_metric_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Sets how this alarm is to handle missing data points. The following values are supported: `missing`, `ignore`, `breaching` and `notBreaching`. Defaults to `missing`.
        #[builder(into, default)]
        pub treat_missing_data: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The unit for the alarm's associated metric.
        #[builder(into, default)]
        pub unit: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MetricAlarmResult {
        /// Indicates whether or not actions should be executed during any changes to the alarm's state. Defaults to `true`.
        pub actions_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The list of actions to execute when this alarm transitions into an ALARM state from any other state. Each action is specified as an Amazon Resource Name (ARN).
        pub alarm_actions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The description for the alarm.
        pub alarm_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the CloudWatch Metric Alarm.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The arithmetic operation to use when comparing the specified Statistic and Threshold. The specified Statistic value is used as the first operand. Either of the following is supported: `GreaterThanOrEqualToThreshold`, `GreaterThanThreshold`, `LessThanThreshold`, `LessThanOrEqualToThreshold`. Additionally, the values  `LessThanLowerOrGreaterThanUpperThreshold`, `LessThanLowerThreshold`, and `GreaterThanUpperThreshold` are used only for alarms based on anomaly detection models.
        pub comparison_operator: pulumi_wasm_rust::Output<String>,
        /// The number of datapoints that must be breaching to trigger the alarm.
        pub datapoints_to_alarm: pulumi_wasm_rust::Output<Option<i32>>,
        /// The dimensions for the alarm's associated metric.  For the list of available dimensions see the AWS documentation [here](http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/CW_Support_For_AWS.html).
        pub dimensions: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Used only for alarms based on percentiles.
        /// If you specify `ignore`, the alarm state will not change during periods with too few data points to be statistically significant.
        /// If you specify `evaluate` or omit this parameter, the alarm will always be evaluated and possibly change state no matter how many data points are available.
        /// The following values are supported: `ignore`, and `evaluate`.
        pub evaluate_low_sample_count_percentiles: pulumi_wasm_rust::Output<String>,
        /// The number of periods over which data is compared to the specified threshold.
        pub evaluation_periods: pulumi_wasm_rust::Output<i32>,
        /// The percentile statistic for the metric associated with the alarm. Specify a value between p0.0 and p100.
        pub extended_statistic: pulumi_wasm_rust::Output<Option<String>>,
        /// The list of actions to execute when this alarm transitions into an INSUFFICIENT_DATA state from any other state. Each action is specified as an Amazon Resource Name (ARN).
        pub insufficient_data_actions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name for the alarm's associated metric.
        /// See docs for [supported metrics](https://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/CW_Support_For_AWS.html).
        pub metric_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Enables you to create an alarm based on a metric math expression. You may specify at most 20.
        pub metric_queries: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudwatch::MetricAlarmMetricQuery>>,
        >,
        /// The descriptive name for the alarm. This name must be unique within the user's AWS account
        pub name: pulumi_wasm_rust::Output<String>,
        /// The namespace for the alarm's associated metric. See docs for the [list of namespaces](https://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/aws-namespaces.html).
        /// See docs for [supported metrics](https://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/CW_Support_For_AWS.html).
        pub namespace: pulumi_wasm_rust::Output<Option<String>>,
        /// The list of actions to execute when this alarm transitions into an OK state from any other state. Each action is specified as an Amazon Resource Name (ARN).
        pub ok_actions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The period in seconds over which the specified `statistic` is applied.
        /// Valid values are `10`, `30`, or any multiple of `60`.
        pub period: pulumi_wasm_rust::Output<Option<i32>>,
        /// The statistic to apply to the alarm's associated metric.
        /// Either of the following is supported: `SampleCount`, `Average`, `Sum`, `Minimum`, `Maximum`
        pub statistic: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// > **NOTE:**  If you specify at least one `metric_query`, you may not specify a `metric_name`, `namespace`, `period` or `statistic`. If you do not specify a `metric_query`, you must specify each of these (although you may use `extended_statistic` instead of `statistic`).
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The value against which the specified statistic is compared. This parameter is required for alarms based on static thresholds, but should not be used for alarms based on anomaly detection models.
        pub threshold: pulumi_wasm_rust::Output<Option<f64>>,
        /// If this is an alarm based on an anomaly detection model, make this value match the ID of the ANOMALY_DETECTION_BAND function.
        pub threshold_metric_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Sets how this alarm is to handle missing data points. The following values are supported: `missing`, `ignore`, `breaching` and `notBreaching`. Defaults to `missing`.
        pub treat_missing_data: pulumi_wasm_rust::Output<Option<String>>,
        /// The unit for the alarm's associated metric.
        pub unit: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MetricAlarmArgs,
    ) -> MetricAlarmResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let actions_enabled_binding = args
            .actions_enabled
            .get_output(context)
            .get_inner();
        let alarm_actions_binding = args.alarm_actions.get_output(context).get_inner();
        let alarm_description_binding = args
            .alarm_description
            .get_output(context)
            .get_inner();
        let comparison_operator_binding = args
            .comparison_operator
            .get_output(context)
            .get_inner();
        let datapoints_to_alarm_binding = args
            .datapoints_to_alarm
            .get_output(context)
            .get_inner();
        let dimensions_binding = args.dimensions.get_output(context).get_inner();
        let evaluate_low_sample_count_percentiles_binding = args
            .evaluate_low_sample_count_percentiles
            .get_output(context)
            .get_inner();
        let evaluation_periods_binding = args
            .evaluation_periods
            .get_output(context)
            .get_inner();
        let extended_statistic_binding = args
            .extended_statistic
            .get_output(context)
            .get_inner();
        let insufficient_data_actions_binding = args
            .insufficient_data_actions
            .get_output(context)
            .get_inner();
        let metric_name_binding = args.metric_name.get_output(context).get_inner();
        let metric_queries_binding = args.metric_queries.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let namespace_binding = args.namespace.get_output(context).get_inner();
        let ok_actions_binding = args.ok_actions.get_output(context).get_inner();
        let period_binding = args.period.get_output(context).get_inner();
        let statistic_binding = args.statistic.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let threshold_binding = args.threshold.get_output(context).get_inner();
        let threshold_metric_id_binding = args
            .threshold_metric_id
            .get_output(context)
            .get_inner();
        let treat_missing_data_binding = args
            .treat_missing_data
            .get_output(context)
            .get_inner();
        let unit_binding = args.unit.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/metricAlarm:MetricAlarm".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actionsEnabled".into(),
                    value: &actions_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "alarmActions".into(),
                    value: &alarm_actions_binding,
                },
                register_interface::ObjectField {
                    name: "alarmDescription".into(),
                    value: &alarm_description_binding,
                },
                register_interface::ObjectField {
                    name: "comparisonOperator".into(),
                    value: &comparison_operator_binding,
                },
                register_interface::ObjectField {
                    name: "datapointsToAlarm".into(),
                    value: &datapoints_to_alarm_binding,
                },
                register_interface::ObjectField {
                    name: "dimensions".into(),
                    value: &dimensions_binding,
                },
                register_interface::ObjectField {
                    name: "evaluateLowSampleCountPercentiles".into(),
                    value: &evaluate_low_sample_count_percentiles_binding,
                },
                register_interface::ObjectField {
                    name: "evaluationPeriods".into(),
                    value: &evaluation_periods_binding,
                },
                register_interface::ObjectField {
                    name: "extendedStatistic".into(),
                    value: &extended_statistic_binding,
                },
                register_interface::ObjectField {
                    name: "insufficientDataActions".into(),
                    value: &insufficient_data_actions_binding,
                },
                register_interface::ObjectField {
                    name: "metricName".into(),
                    value: &metric_name_binding,
                },
                register_interface::ObjectField {
                    name: "metricQueries".into(),
                    value: &metric_queries_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding,
                },
                register_interface::ObjectField {
                    name: "okActions".into(),
                    value: &ok_actions_binding,
                },
                register_interface::ObjectField {
                    name: "period".into(),
                    value: &period_binding,
                },
                register_interface::ObjectField {
                    name: "statistic".into(),
                    value: &statistic_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "threshold".into(),
                    value: &threshold_binding,
                },
                register_interface::ObjectField {
                    name: "thresholdMetricId".into(),
                    value: &threshold_metric_id_binding,
                },
                register_interface::ObjectField {
                    name: "treatMissingData".into(),
                    value: &treat_missing_data_binding,
                },
                register_interface::ObjectField {
                    name: "unit".into(),
                    value: &unit_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "actionsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "alarmActions".into(),
                },
                register_interface::ResultField {
                    name: "alarmDescription".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "comparisonOperator".into(),
                },
                register_interface::ResultField {
                    name: "datapointsToAlarm".into(),
                },
                register_interface::ResultField {
                    name: "dimensions".into(),
                },
                register_interface::ResultField {
                    name: "evaluateLowSampleCountPercentiles".into(),
                },
                register_interface::ResultField {
                    name: "evaluationPeriods".into(),
                },
                register_interface::ResultField {
                    name: "extendedStatistic".into(),
                },
                register_interface::ResultField {
                    name: "insufficientDataActions".into(),
                },
                register_interface::ResultField {
                    name: "metricName".into(),
                },
                register_interface::ResultField {
                    name: "metricQueries".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namespace".into(),
                },
                register_interface::ResultField {
                    name: "okActions".into(),
                },
                register_interface::ResultField {
                    name: "period".into(),
                },
                register_interface::ResultField {
                    name: "statistic".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "threshold".into(),
                },
                register_interface::ResultField {
                    name: "thresholdMetricId".into(),
                },
                register_interface::ResultField {
                    name: "treatMissingData".into(),
                },
                register_interface::ResultField {
                    name: "unit".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MetricAlarmResult {
            actions_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionsEnabled").unwrap(),
            ),
            alarm_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alarmActions").unwrap(),
            ),
            alarm_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alarmDescription").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            comparison_operator: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comparisonOperator").unwrap(),
            ),
            datapoints_to_alarm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("datapointsToAlarm").unwrap(),
            ),
            dimensions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dimensions").unwrap(),
            ),
            evaluate_low_sample_count_percentiles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("evaluateLowSampleCountPercentiles").unwrap(),
            ),
            evaluation_periods: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("evaluationPeriods").unwrap(),
            ),
            extended_statistic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extendedStatistic").unwrap(),
            ),
            insufficient_data_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("insufficientDataActions").unwrap(),
            ),
            metric_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metricName").unwrap(),
            ),
            metric_queries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metricQueries").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespace").unwrap(),
            ),
            ok_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("okActions").unwrap(),
            ),
            period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("period").unwrap(),
            ),
            statistic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statistic").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            threshold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("threshold").unwrap(),
            ),
            threshold_metric_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thresholdMetricId").unwrap(),
            ),
            treat_missing_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("treatMissingData").unwrap(),
            ),
            unit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("unit").unwrap(),
            ),
        }
    }
}
