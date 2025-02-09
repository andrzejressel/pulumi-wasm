/// Provides a CloudWatch Metric Stream resource.
///
/// ## Example Usage
///
/// ### Filters
///
/// ```yaml
/// resources:
///   main:
///     type: aws:cloudwatch:MetricStream
///     properties:
///       name: my-metric-stream
///       roleArn: ${metricStreamToFirehoseRole.arn}
///       firehoseArn: ${s3Stream.arn}
///       outputFormat: json
///       includeFilters:
///         - namespace: AWS/EC2
///           metricNames:
///             - CPUUtilization
///             - NetworkOut
///         - namespace: AWS/EBS
///           metricNames: []
///   metricStreamToFirehoseRole:
///     type: aws:iam:Role
///     name: metric_stream_to_firehose
///     properties:
///       name: metric_stream_to_firehose_role
///       assumeRolePolicy: ${streamsAssumeRole.json}
///   metricStreamToFirehoseRolePolicy:
///     type: aws:iam:RolePolicy
///     name: metric_stream_to_firehose
///     properties:
///       name: default
///       role: ${metricStreamToFirehoseRole.id}
///       policy: ${metricStreamToFirehose.json}
///   bucket:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: metric-stream-test-bucket
///   bucketAcl:
///     type: aws:s3:BucketAclV2
///     name: bucket_acl
///     properties:
///       bucket: ${bucket.id}
///       acl: private
///   firehoseToS3Role:
///     type: aws:iam:Role
///     name: firehose_to_s3
///     properties:
///       assumeRolePolicy: ${firehoseAssumeRole.json}
///   firehoseToS3RolePolicy:
///     type: aws:iam:RolePolicy
///     name: firehose_to_s3
///     properties:
///       name: default
///       role: ${firehoseToS3Role.id}
///       policy: ${firehoseToS3.json}
///   s3Stream:
///     type: aws:kinesis:FirehoseDeliveryStream
///     name: s3_stream
///     properties:
///       name: metric-stream-test-stream
///       destination: extended_s3
///       extendedS3Configuration:
///         roleArn: ${firehoseToS3Role.arn}
///         bucketArn: ${bucket.arn}
/// variables:
///   # https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-trustpolicy.html
///   streamsAssumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - streams.metrics.cloudwatch.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   # https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-trustpolicy.html
///   metricStreamToFirehose:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - firehose:PutRecord
///               - firehose:PutRecordBatch
///             resources:
///               - ${s3Stream.arn}
///   firehoseAssumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - firehose.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   firehoseToS3:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - s3:AbortMultipartUpload
///               - s3:GetBucketLocation
///               - s3:GetObject
///               - s3:ListBucket
///               - s3:ListBucketMultipartUploads
///               - s3:PutObject
///             resources:
///               - ${bucket.arn}
///               - ${bucket.arn}/*
/// ```
///
/// ### Additional Statistics
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod metric_stream {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MetricStreamArgs {
        /// List of exclusive metric filters. If you specify this parameter, the stream sends metrics from all metric namespaces except for the namespaces and the conditional metric names that you specify here. If you don't specify metric names or provide empty metric names whole metric namespace is excluded. Conflicts with `include_filter`.
        #[builder(into, default)]
        pub exclude_filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cloudwatch::MetricStreamExcludeFilter>>,
        >,
        /// ARN of the Amazon Kinesis Firehose delivery stream to use for this metric stream.
        #[builder(into)]
        pub firehose_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of inclusive metric filters. If you specify this parameter, the stream sends only the conditional metric names from the metric namespaces that you specify here. If you don't specify metric names or provide empty metric names whole metric namespace is included. Conflicts with `exclude_filter`.
        #[builder(into, default)]
        pub include_filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cloudwatch::MetricStreamIncludeFilter>>,
        >,
        /// If you are creating a metric stream in a monitoring account, specify true to include metrics from source accounts that are linked to this monitoring account, in the metric stream. The default is false. For more information about linking accounts, see [CloudWatch cross-account observability](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Unified-Cross-Account.html).
        #[builder(into, default)]
        pub include_linked_accounts_metrics: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Friendly name of the metric stream. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique friendly name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Output format for the stream. Possible values are `json`, `opentelemetry0.7`, and `opentelemetry1.0`. For more information about output formats, see [Metric streams output formats](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-formats.html).
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub output_format: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the IAM role that this metric stream will use to access Amazon Kinesis Firehose resources. For more information about role permissions, see [Trust between CloudWatch and Kinesis Data Firehose](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-trustpolicy.html).
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// For each entry in this array, you specify one or more metrics and the list of additional statistics to stream for those metrics. The additional statistics that you can stream depend on the stream's `output_format`. If the OutputFormat is `json`, you can stream any additional statistic that is supported by CloudWatch, listed in [CloudWatch statistics definitions](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Statistics-definitions.html.html). If the OutputFormat is `opentelemetry0.7` or `opentelemetry1.0`, you can stream percentile statistics (p99 etc.). See details below.
        #[builder(into, default)]
        pub statistics_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::cloudwatch::MetricStreamStatisticsConfiguration>,
            >,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MetricStreamResult {
        /// ARN of the metric stream.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the metric stream was created.
        pub creation_date: pulumi_gestalt_rust::Output<String>,
        /// List of exclusive metric filters. If you specify this parameter, the stream sends metrics from all metric namespaces except for the namespaces and the conditional metric names that you specify here. If you don't specify metric names or provide empty metric names whole metric namespace is excluded. Conflicts with `include_filter`.
        pub exclude_filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cloudwatch::MetricStreamExcludeFilter>>,
        >,
        /// ARN of the Amazon Kinesis Firehose delivery stream to use for this metric stream.
        pub firehose_arn: pulumi_gestalt_rust::Output<String>,
        /// List of inclusive metric filters. If you specify this parameter, the stream sends only the conditional metric names from the metric namespaces that you specify here. If you don't specify metric names or provide empty metric names whole metric namespace is included. Conflicts with `exclude_filter`.
        pub include_filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cloudwatch::MetricStreamIncludeFilter>>,
        >,
        /// If you are creating a metric stream in a monitoring account, specify true to include metrics from source accounts that are linked to this monitoring account, in the metric stream. The default is false. For more information about linking accounts, see [CloudWatch cross-account observability](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Unified-Cross-Account.html).
        pub include_linked_accounts_metrics: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the metric stream was last updated.
        pub last_update_date: pulumi_gestalt_rust::Output<String>,
        /// Friendly name of the metric stream. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique friendly name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// Output format for the stream. Possible values are `json`, `opentelemetry0.7`, and `opentelemetry1.0`. For more information about output formats, see [Metric streams output formats](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-formats.html).
        ///
        /// The following arguments are optional:
        pub output_format: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM role that this metric stream will use to access Amazon Kinesis Firehose resources. For more information about role permissions, see [Trust between CloudWatch and Kinesis Data Firehose](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-trustpolicy.html).
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// State of the metric stream. Possible values are `running` and `stopped`.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// For each entry in this array, you specify one or more metrics and the list of additional statistics to stream for those metrics. The additional statistics that you can stream depend on the stream's `output_format`. If the OutputFormat is `json`, you can stream any additional statistic that is supported by CloudWatch, listed in [CloudWatch statistics definitions](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Statistics-definitions.html.html). If the OutputFormat is `opentelemetry0.7` or `opentelemetry1.0`, you can stream percentile statistics (p99 etc.). See details below.
        pub statistics_configurations: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::cloudwatch::MetricStreamStatisticsConfiguration>,
            >,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MetricStreamArgs,
    ) -> MetricStreamResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let exclude_filters_binding_1 = args.exclude_filters.get_output(context);
        let exclude_filters_binding = exclude_filters_binding_1.get_inner();
        let firehose_arn_binding_1 = args.firehose_arn.get_output(context);
        let firehose_arn_binding = firehose_arn_binding_1.get_inner();
        let include_filters_binding_1 = args.include_filters.get_output(context);
        let include_filters_binding = include_filters_binding_1.get_inner();
        let include_linked_accounts_metrics_binding_1 = args
            .include_linked_accounts_metrics
            .get_output(context);
        let include_linked_accounts_metrics_binding = include_linked_accounts_metrics_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let name_prefix_binding_1 = args.name_prefix.get_output(context);
        let name_prefix_binding = name_prefix_binding_1.get_inner();
        let output_format_binding_1 = args.output_format.get_output(context);
        let output_format_binding = output_format_binding_1.get_inner();
        let role_arn_binding_1 = args.role_arn.get_output(context);
        let role_arn_binding = role_arn_binding_1.get_inner();
        let statistics_configurations_binding_1 = args
            .statistics_configurations
            .get_output(context);
        let statistics_configurations_binding = statistics_configurations_binding_1
            .get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/metricStream:MetricStream".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        MetricStreamResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            creation_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationDate"),
            ),
            exclude_filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("excludeFilters"),
            ),
            firehose_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("firehoseArn"),
            ),
            include_filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("includeFilters"),
            ),
            include_linked_accounts_metrics: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("includeLinkedAccountsMetrics"),
            ),
            last_update_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastUpdateDate"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            name_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namePrefix"),
            ),
            output_format: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outputFormat"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            statistics_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statisticsConfigurations"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
