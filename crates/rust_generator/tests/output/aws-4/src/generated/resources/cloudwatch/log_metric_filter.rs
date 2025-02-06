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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LogMetricFilterArgs,
    ) -> LogMetricFilterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let log_group_name_binding = args.log_group_name.get_output(context).get_inner();
        let metric_transformation_binding = args
            .metric_transformation
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let pattern_binding = args.pattern.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/logMetricFilter:LogMetricFilter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "logGroupName".into(),
                    value: &log_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "metricTransformation".into(),
                    value: &metric_transformation_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "pattern".into(),
                    value: &pattern_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LogMetricFilterResult {
            log_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logGroupName"),
            ),
            metric_transformation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metricTransformation"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            pattern: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pattern"),
            ),
        }
    }
}
