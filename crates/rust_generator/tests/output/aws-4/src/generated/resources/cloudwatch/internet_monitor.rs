/// Provides a Internet Monitor Monitor resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = internet_monitor::create(
///         "example",
///         InternetMonitorArgs::builder().monitor_name("exmple").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Internet Monitor Monitors using the `monitor_name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/internetMonitor:InternetMonitor some some-monitor
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod internet_monitor {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InternetMonitorArgs {
        /// Health event thresholds. A health event threshold percentage, for performance and availability, determines when Internet Monitor creates a health event when there's an internet issue that affects your application end users. See Health Events Config below.
        #[builder(into, default)]
        pub health_events_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudwatch::InternetMonitorHealthEventsConfig>,
        >,
        /// Publish internet measurements for Internet Monitor to an Amazon S3 bucket in addition to CloudWatch Logs.
        #[builder(into, default)]
        pub internet_measurements_log_delivery: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::cloudwatch::InternetMonitorInternetMeasurementsLogDelivery,
            >,
        >,
        /// The maximum number of city-networks to monitor for your resources. A city-network is the location (city) where clients access your application resources from and the network or ASN, such as an internet service provider (ISP), that clients access the resources through. This limit helps control billing costs.
        #[builder(into, default)]
        pub max_city_networks_to_monitor: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The name of the monitor.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub monitor_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resources to include in a monitor, which you provide as a set of Amazon Resource Names (ARNs).
        #[builder(into, default)]
        pub resources: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The status for a monitor. The accepted values for Status with the UpdateMonitor API call are the following: `ACTIVE` and `INACTIVE`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The percentage of the internet-facing traffic for your application that you want to monitor with this monitor.
        #[builder(into, default)]
        pub traffic_percentage_to_monitor: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
    }
    #[allow(dead_code)]
    pub struct InternetMonitorResult {
        /// ARN of the Monitor.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Health event thresholds. A health event threshold percentage, for performance and availability, determines when Internet Monitor creates a health event when there's an internet issue that affects your application end users. See Health Events Config below.
        pub health_events_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudwatch::InternetMonitorHealthEventsConfig>,
        >,
        /// Publish internet measurements for Internet Monitor to an Amazon S3 bucket in addition to CloudWatch Logs.
        pub internet_measurements_log_delivery: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::cloudwatch::InternetMonitorInternetMeasurementsLogDelivery,
            >,
        >,
        /// The maximum number of city-networks to monitor for your resources. A city-network is the location (city) where clients access your application resources from and the network or ASN, such as an internet service provider (ISP), that clients access the resources through. This limit helps control billing costs.
        pub max_city_networks_to_monitor: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the monitor.
        ///
        /// The following arguments are optional:
        pub monitor_name: pulumi_gestalt_rust::Output<String>,
        /// The resources to include in a monitor, which you provide as a set of Amazon Resource Names (ARNs).
        pub resources: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The status for a monitor. The accepted values for Status with the UpdateMonitor API call are the following: `ACTIVE` and `INACTIVE`.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The percentage of the internet-facing traffic for your application that you want to monitor with this monitor.
        pub traffic_percentage_to_monitor: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InternetMonitorArgs,
    ) -> InternetMonitorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let health_events_config_binding = args
            .health_events_config
            .get_output(context)
            .get_inner();
        let internet_measurements_log_delivery_binding = args
            .internet_measurements_log_delivery
            .get_output(context)
            .get_inner();
        let max_city_networks_to_monitor_binding = args
            .max_city_networks_to_monitor
            .get_output(context)
            .get_inner();
        let monitor_name_binding = args.monitor_name.get_output(context).get_inner();
        let resources_binding = args.resources.get_output(context).get_inner();
        let status_binding = args.status.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let traffic_percentage_to_monitor_binding = args
            .traffic_percentage_to_monitor
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/internetMonitor:InternetMonitor".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "healthEventsConfig".into(),
                    value: &health_events_config_binding,
                },
                register_interface::ObjectField {
                    name: "internetMeasurementsLogDelivery".into(),
                    value: &internet_measurements_log_delivery_binding,
                },
                register_interface::ObjectField {
                    name: "maxCityNetworksToMonitor".into(),
                    value: &max_city_networks_to_monitor_binding,
                },
                register_interface::ObjectField {
                    name: "monitorName".into(),
                    value: &monitor_name_binding,
                },
                register_interface::ObjectField {
                    name: "resources".into(),
                    value: &resources_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "trafficPercentageToMonitor".into(),
                    value: &traffic_percentage_to_monitor_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InternetMonitorResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            health_events_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("healthEventsConfig"),
            ),
            internet_measurements_log_delivery: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("internetMeasurementsLogDelivery"),
            ),
            max_city_networks_to_monitor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxCityNetworksToMonitor"),
            ),
            monitor_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitorName"),
            ),
            resources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resources"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            traffic_percentage_to_monitor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("trafficPercentageToMonitor"),
            ),
        }
    }
}
