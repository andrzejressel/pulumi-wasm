/// Provides a CloudWatch Log Metric Filter resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   yada:
///     type: aws:cloudwatch:LogMetricFilter
///     properties:
///       name: MyAppAccessCount
///       pattern:
///       logGroupName: ${dada.name}
///       metricTransformation:
///         name: EventCount
///         namespace: YourNamespace
///         value: '1'
///   dada:
///     type: aws:cloudwatch:LogGroup
///     properties:
///       name: MyApp/access.log
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogMetricFilterArgs {
        /// The name of the log group to associate the metric filter with.
        #[builder(into)]
        pub log_group_name: pulumi_wasm_rust::Output<String>,
        /// A block defining collection of information needed to define how metric data gets emitted. See below.
        #[builder(into)]
        pub metric_transformation: pulumi_wasm_rust::Output<
            super::super::types::cloudwatch::LogMetricFilterMetricTransformation,
        >,
        /// A name for the metric filter.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A valid [CloudWatch Logs filter pattern](https://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/FilterAndPatternSyntax.html)
        /// for extracting metric data out of ingested log events.
        #[builder(into)]
        pub pattern: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LogMetricFilterResult {
        /// The name of the log group to associate the metric filter with.
        pub log_group_name: pulumi_wasm_rust::Output<String>,
        /// A block defining collection of information needed to define how metric data gets emitted. See below.
        pub metric_transformation: pulumi_wasm_rust::Output<
            super::super::types::cloudwatch::LogMetricFilterMetricTransformation,
        >,
        /// A name for the metric filter.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A valid [CloudWatch Logs filter pattern](https://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/FilterAndPatternSyntax.html)
        /// for extracting metric data out of ingested log events.
        pub pattern: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LogMetricFilterArgs) -> LogMetricFilterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let log_group_name_binding = args.log_group_name.get_inner();
        let metric_transformation_binding = args.metric_transformation.get_inner();
        let name_binding = args.name.get_inner();
        let pattern_binding = args.pattern.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/logMetricFilter:LogMetricFilter".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "logGroupName".into(),
                },
                register_interface::ResultField {
                    name: "metricTransformation".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pattern".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LogMetricFilterResult {
            log_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logGroupName").unwrap(),
            ),
            metric_transformation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metricTransformation").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pattern: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pattern").unwrap(),
            ),
        }
    }
}
