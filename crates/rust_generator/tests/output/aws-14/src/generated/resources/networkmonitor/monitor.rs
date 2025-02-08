/// Resource for managing an AWS Network Monitor Monitor.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod monitor {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitorArgs {
        /// The time, in seconds, that metrics are aggregated and sent to Amazon CloudWatch. Valid values are either 30 or 60.
        #[builder(into, default)]
        pub aggregation_period: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the monitor.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub monitor_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value tags for the monitor. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MonitorResult {
        /// The time, in seconds, that metrics are aggregated and sent to Amazon CloudWatch. Valid values are either 30 or 60.
        pub aggregation_period: pulumi_gestalt_rust::Output<i32>,
        /// The ARN of the monitor.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the monitor.
        ///
        /// The following arguments are optional:
        pub monitor_name: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the monitor. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: MonitorArgs,
    ) -> MonitorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let aggregation_period_binding = args
            .aggregation_period
            .get_output(context)
            .get_inner();
        let monitor_name_binding = args.monitor_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmonitor/monitor:Monitor".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        MonitorResult {
            aggregation_period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("aggregationPeriod"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            monitor_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitorName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
