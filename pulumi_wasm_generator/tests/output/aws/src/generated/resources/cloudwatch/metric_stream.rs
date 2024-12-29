/// Provides a CloudWatch Metric Stream resource.
///
/// ## Example Usage
///
/// ### Filters
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let firehoseAssumeRole = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sts:AssumeRole",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["firehose.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let firehoseToS3 = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["s3:AbortMultipartUpload", "s3:GetBucketLocation",
///                     "s3:GetObject", "s3:ListBucket", "s3:ListBucketMultipartUploads",
///                     "s3:PutObject",]).effect("Allow").resources(vec!["${bucket.arn}",
///                     "${bucket.arn}/*",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let metricStreamToFirehose = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["firehose:PutRecord", "firehose:PutRecordBatch",])
///                     .effect("Allow").resources(vec!["${s3Stream.arn}",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let streamsAssumeRole = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sts:AssumeRole",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["streams.metrics.cloudwatch.amazonaws.com",]). type
///                     ("Service").build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let bucket = bucket_v_2::create(
///         "bucket",
///         BucketV2Args::builder().bucket("metric-stream-test-bucket").build_struct(),
///     );
///     let bucketAcl = bucket_acl_v_2::create(
///         "bucketAcl",
///         BucketAclV2Args::builder().acl("private").bucket("${bucket.id}").build_struct(),
///     );
///     let firehoseToS3Role = role::create(
///         "firehoseToS3Role",
///         RoleArgs::builder()
///             .assume_role_policy("${firehoseAssumeRole.json}")
///             .build_struct(),
///     );
///     let firehoseToS3RolePolicy = role_policy::create(
///         "firehoseToS3RolePolicy",
///         RolePolicyArgs::builder()
///             .name("default")
///             .policy("${firehoseToS3.json}")
///             .role("${firehoseToS3Role.id}")
///             .build_struct(),
///     );
///     let main = metric_stream::create(
///         "main",
///         MetricStreamArgs::builder()
///             .firehose_arn("${s3Stream.arn}")
///             .include_filters(
///                 vec![
///                     MetricStreamIncludeFilter::builder()
///                     .metricNames(vec!["CPUUtilization", "NetworkOut",])
///                     .namespace("AWS/EC2").build_struct(),
///                     MetricStreamIncludeFilter::builder().metricNames(vec![])
///                     .namespace("AWS/EBS").build_struct(),
///                 ],
///             )
///             .name("my-metric-stream")
///             .output_format("json")
///             .role_arn("${metricStreamToFirehoseRole.arn}")
///             .build_struct(),
///     );
///     let metricStreamToFirehoseRole = role::create(
///         "metricStreamToFirehoseRole",
///         RoleArgs::builder()
///             .assume_role_policy("${streamsAssumeRole.json}")
///             .name("metric_stream_to_firehose_role")
///             .build_struct(),
///     );
///     let metricStreamToFirehoseRolePolicy = role_policy::create(
///         "metricStreamToFirehoseRolePolicy",
///         RolePolicyArgs::builder()
///             .name("default")
///             .policy("${metricStreamToFirehose.json}")
///             .role("${metricStreamToFirehoseRole.id}")
///             .build_struct(),
///     );
///     let s3Stream = firehose_delivery_stream::create(
///         "s3Stream",
///         FirehoseDeliveryStreamArgs::builder()
///             .destination("extended_s3")
///             .extended_s_3_configuration(
///                 FirehoseDeliveryStreamExtendedS3Configuration::builder()
///                     .bucketArn("${bucket.arn}")
///                     .roleArn("${firehoseToS3Role.arn}")
///                     .build_struct(),
///             )
///             .name("metric-stream-test-stream")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Additional Statistics
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = metric_stream::create(
///         "main",
///         MetricStreamArgs::builder()
///             .firehose_arn("${s3Stream.arn}")
///             .name("my-metric-stream")
///             .output_format("json")
///             .role_arn("${metricStreamToFirehose.arn}")
///             .statistics_configurations(
///                 vec![
///                     MetricStreamStatisticsConfiguration::builder()
///                     .additionalStatistics(vec!["p1", "tm99",])
///                     .includeMetrics(vec![MetricStreamStatisticsConfigurationIncludeMetric::builder()
///                     .metricName("CPUUtilization").namespace("AWS/EC2").build_struct(),])
///                     .build_struct(), MetricStreamStatisticsConfiguration::builder()
///                     .additionalStatistics(vec!["TS(50.5:)",])
///                     .includeMetrics(vec![MetricStreamStatisticsConfigurationIncludeMetric::builder()
///                     .metricName("CPUUtilization").namespace("AWS/EC2").build_struct(),])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch metric streams using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/metricStream:MetricStream sample sample-stream-name
/// ```
pub mod metric_stream {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MetricStreamArgs {
        /// List of exclusive metric filters. If you specify this parameter, the stream sends metrics from all metric namespaces except for the namespaces and the conditional metric names that you specify here. If you don't specify metric names or provide empty metric names whole metric namespace is excluded. Conflicts with `include_filter`.
        #[builder(into, default)]
        pub exclude_filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudwatch::MetricStreamExcludeFilter>>,
        >,
        /// ARN of the Amazon Kinesis Firehose delivery stream to use for this metric stream.
        #[builder(into)]
        pub firehose_arn: pulumi_wasm_rust::Output<String>,
        /// List of inclusive metric filters. If you specify this parameter, the stream sends only the conditional metric names from the metric namespaces that you specify here. If you don't specify metric names or provide empty metric names whole metric namespace is included. Conflicts with `exclude_filter`.
        #[builder(into, default)]
        pub include_filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudwatch::MetricStreamIncludeFilter>>,
        >,
        /// If you are creating a metric stream in a monitoring account, specify true to include metrics from source accounts that are linked to this monitoring account, in the metric stream. The default is false. For more information about linking accounts, see [CloudWatch cross-account observability](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Unified-Cross-Account.html).
        #[builder(into, default)]
        pub include_linked_accounts_metrics: pulumi_wasm_rust::Output<Option<bool>>,
        /// Friendly name of the metric stream. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique friendly name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Output format for the stream. Possible values are `json`, `opentelemetry0.7`, and `opentelemetry1.0`. For more information about output formats, see [Metric streams output formats](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-formats.html).
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub output_format: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role that this metric stream will use to access Amazon Kinesis Firehose resources. For more information about role permissions, see [Trust between CloudWatch and Kinesis Data Firehose](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-trustpolicy.html).
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// For each entry in this array, you specify one or more metrics and the list of additional statistics to stream for those metrics. The additional statistics that you can stream depend on the stream's `output_format`. If the OutputFormat is `json`, you can stream any additional statistic that is supported by CloudWatch, listed in [CloudWatch statistics definitions](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Statistics-definitions.html.html). If the OutputFormat is `opentelemetry0.7` or `opentelemetry1.0`, you can stream percentile statistics (p99 etc.). See details below.
        #[builder(into, default)]
        pub statistics_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::cloudwatch::MetricStreamStatisticsConfiguration>,
            >,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MetricStreamResult {
        /// ARN of the metric stream.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the metric stream was created.
        pub creation_date: pulumi_wasm_rust::Output<String>,
        /// List of exclusive metric filters. If you specify this parameter, the stream sends metrics from all metric namespaces except for the namespaces and the conditional metric names that you specify here. If you don't specify metric names or provide empty metric names whole metric namespace is excluded. Conflicts with `include_filter`.
        pub exclude_filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudwatch::MetricStreamExcludeFilter>>,
        >,
        /// ARN of the Amazon Kinesis Firehose delivery stream to use for this metric stream.
        pub firehose_arn: pulumi_wasm_rust::Output<String>,
        /// List of inclusive metric filters. If you specify this parameter, the stream sends only the conditional metric names from the metric namespaces that you specify here. If you don't specify metric names or provide empty metric names whole metric namespace is included. Conflicts with `exclude_filter`.
        pub include_filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudwatch::MetricStreamIncludeFilter>>,
        >,
        /// If you are creating a metric stream in a monitoring account, specify true to include metrics from source accounts that are linked to this monitoring account, in the metric stream. The default is false. For more information about linking accounts, see [CloudWatch cross-account observability](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Unified-Cross-Account.html).
        pub include_linked_accounts_metrics: pulumi_wasm_rust::Output<Option<bool>>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the metric stream was last updated.
        pub last_update_date: pulumi_wasm_rust::Output<String>,
        /// Friendly name of the metric stream. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique friendly name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// Output format for the stream. Possible values are `json`, `opentelemetry0.7`, and `opentelemetry1.0`. For more information about output formats, see [Metric streams output formats](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-formats.html).
        ///
        /// The following arguments are optional:
        pub output_format: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role that this metric stream will use to access Amazon Kinesis Firehose resources. For more information about role permissions, see [Trust between CloudWatch and Kinesis Data Firehose](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-trustpolicy.html).
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// State of the metric stream. Possible values are `running` and `stopped`.
        pub state: pulumi_wasm_rust::Output<String>,
        /// For each entry in this array, you specify one or more metrics and the list of additional statistics to stream for those metrics. The additional statistics that you can stream depend on the stream's `output_format`. If the OutputFormat is `json`, you can stream any additional statistic that is supported by CloudWatch, listed in [CloudWatch statistics definitions](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Statistics-definitions.html.html). If the OutputFormat is `opentelemetry0.7` or `opentelemetry1.0`, you can stream percentile statistics (p99 etc.). See details below.
        pub statistics_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::cloudwatch::MetricStreamStatisticsConfiguration>,
            >,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MetricStreamArgs) -> MetricStreamResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let exclude_filters_binding = args.exclude_filters.get_inner();
        let firehose_arn_binding = args.firehose_arn.get_inner();
        let include_filters_binding = args.include_filters.get_inner();
        let include_linked_accounts_metrics_binding = args
            .include_linked_accounts_metrics
            .get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let output_format_binding = args.output_format.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let statistics_configurations_binding = args
            .statistics_configurations
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/metricStream:MetricStream".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "excludeFilters".into(),
                    value: &exclude_filters_binding,
                },
                register_interface::ObjectField {
                    name: "firehoseArn".into(),
                    value: &firehose_arn_binding,
                },
                register_interface::ObjectField {
                    name: "includeFilters".into(),
                    value: &include_filters_binding,
                },
                register_interface::ObjectField {
                    name: "includeLinkedAccountsMetrics".into(),
                    value: &include_linked_accounts_metrics_binding,
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
                    name: "outputFormat".into(),
                    value: &output_format_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "statisticsConfigurations".into(),
                    value: &statistics_configurations_binding,
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
                    name: "creationDate".into(),
                },
                register_interface::ResultField {
                    name: "excludeFilters".into(),
                },
                register_interface::ResultField {
                    name: "firehoseArn".into(),
                },
                register_interface::ResultField {
                    name: "includeFilters".into(),
                },
                register_interface::ResultField {
                    name: "includeLinkedAccountsMetrics".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdateDate".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "outputFormat".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "statisticsConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MetricStreamResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationDate").unwrap(),
            ),
            exclude_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("excludeFilters").unwrap(),
            ),
            firehose_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firehoseArn").unwrap(),
            ),
            include_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includeFilters").unwrap(),
            ),
            include_linked_accounts_metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includeLinkedAccountsMetrics").unwrap(),
            ),
            last_update_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdateDate").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            output_format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputFormat").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            statistics_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statisticsConfigurations").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
