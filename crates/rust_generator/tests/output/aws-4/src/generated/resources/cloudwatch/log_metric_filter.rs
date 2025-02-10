/// Provides a CloudWatch Log Metric Filter resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dada = log_group::create(
///         "dada",
///         LogGroupArgs::builder().name("MyApp/access.log").build_struct(),
///     );
///     let yada = log_metric_filter::create(
///         "yada",
///         LogMetricFilterArgs::builder()
///             .log_group_name("${dada.name}")
///             .metric_transformation(
///                 LogMetricFilterMetricTransformation::builder()
///                     .name("EventCount")
///                     .namespace("YourNamespace")
///                     .value("1")
///                     .build_struct(),
///             )
///             .name("MyAppAccessCount")
///             .pattern("")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Log Metric Filter using the `log_group_name:name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logMetricFilter:LogMetricFilter test /aws/lambda/function:test
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod log_metric_filter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogMetricFilterArgs {
        /// The name of the log group to associate the metric filter with.
        #[builder(into)]
        pub log_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A block defining collection of information needed to define how metric data gets emitted. See below.
        #[builder(into)]
        pub metric_transformation: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudwatch::LogMetricFilterMetricTransformation,
        >,
        /// A name for the metric filter.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A valid [CloudWatch Logs filter pattern](https://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/FilterAndPatternSyntax.html)
        /// for extracting metric data out of ingested log events.
        #[builder(into)]
        pub pattern: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LogMetricFilterResult {
        /// The name of the log group to associate the metric filter with.
        pub log_group_name: pulumi_gestalt_rust::Output<String>,
        /// A block defining collection of information needed to define how metric data gets emitted. See below.
        pub metric_transformation: pulumi_gestalt_rust::Output<
            super::super::types::cloudwatch::LogMetricFilterMetricTransformation,
        >,
        /// A name for the metric filter.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A valid [CloudWatch Logs filter pattern](https://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/FilterAndPatternSyntax.html)
        /// for extracting metric data out of ingested log events.
        pub pattern: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogMetricFilterArgs,
    ) -> LogMetricFilterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let log_group_name_binding = args.log_group_name.get_output(context);
        let metric_transformation_binding = args
            .metric_transformation
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let pattern_binding = args.pattern.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/logMetricFilter:LogMetricFilter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logGroupName".into(),
                    value: log_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metricTransformation".into(),
                    value: metric_transformation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pattern".into(),
                    value: pattern_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LogMetricFilterResult {
            log_group_name: o.get_field("logGroupName"),
            metric_transformation: o.get_field("metricTransformation"),
            name: o.get_field("name"),
            pattern: o.get_field("pattern"),
        }
    }
}
