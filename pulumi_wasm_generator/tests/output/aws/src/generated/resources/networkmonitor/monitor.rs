/// Resource for managing an AWS Network Monitor Monitor.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = monitor::create(
///         "example",
///         MonitorArgs::builder()
///             .aggregation_period(30)
///             .monitor_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmonitor_monitor` using the monitor name. For example:
///
/// ```sh
/// $ pulumi import aws:networkmonitor/monitor:Monitor example monitor-7786087912324693644
/// ```
pub mod monitor {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitorArgs {
        /// The time, in seconds, that metrics are aggregated and sent to Amazon CloudWatch. Valid values are either 30 or 60.
        #[builder(into, default)]
        pub aggregation_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the monitor.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub monitor_name: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the monitor. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MonitorResult {
        /// The time, in seconds, that metrics are aggregated and sent to Amazon CloudWatch. Valid values are either 30 or 60.
        pub aggregation_period: pulumi_wasm_rust::Output<i32>,
        /// The ARN of the monitor.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the monitor.
        ///
        /// The following arguments are optional:
        pub monitor_name: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the monitor. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: MonitorArgs) -> MonitorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aggregation_period_binding = args.aggregation_period.get_inner();
        let monitor_name_binding = args.monitor_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmonitor/monitor:Monitor".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aggregationPeriod".into(),
                    value: &aggregation_period_binding,
                },
                register_interface::ObjectField {
                    name: "monitorName".into(),
                    value: &monitor_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "aggregationPeriod".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "monitorName".into(),
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
        MonitorResult {
            aggregation_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aggregationPeriod").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            monitor_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitorName").unwrap(),
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