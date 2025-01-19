/// Provides a Internet Monitor Monitor resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod internet_monitor {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InternetMonitorArgs {
        /// Health event thresholds. A health event threshold percentage, for performance and availability, determines when Internet Monitor creates a health event when there's an internet issue that affects your application end users. See Health Events Config below.
        #[builder(into, default)]
        pub health_events_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::InternetMonitorHealthEventsConfig>,
        >,
        /// Publish internet measurements for Internet Monitor to an Amazon S3 bucket in addition to CloudWatch Logs.
        #[builder(into, default)]
        pub internet_measurements_log_delivery: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cloudwatch::InternetMonitorInternetMeasurementsLogDelivery,
            >,
        >,
        /// The maximum number of city-networks to monitor for your resources. A city-network is the location (city) where clients access your application resources from and the network or ASN, such as an internet service provider (ISP), that clients access the resources through. This limit helps control billing costs.
        #[builder(into, default)]
        pub max_city_networks_to_monitor: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the monitor.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub monitor_name: pulumi_wasm_rust::Output<String>,
        /// The resources to include in a monitor, which you provide as a set of Amazon Resource Names (ARNs).
        #[builder(into, default)]
        pub resources: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The status for a monitor. The accepted values for Status with the UpdateMonitor API call are the following: `ACTIVE` and `INACTIVE`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The percentage of the internet-facing traffic for your application that you want to monitor with this monitor.
        #[builder(into, default)]
        pub traffic_percentage_to_monitor: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct InternetMonitorResult {
        /// ARN of the Monitor.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Health event thresholds. A health event threshold percentage, for performance and availability, determines when Internet Monitor creates a health event when there's an internet issue that affects your application end users. See Health Events Config below.
        pub health_events_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::InternetMonitorHealthEventsConfig>,
        >,
        /// Publish internet measurements for Internet Monitor to an Amazon S3 bucket in addition to CloudWatch Logs.
        pub internet_measurements_log_delivery: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cloudwatch::InternetMonitorInternetMeasurementsLogDelivery,
            >,
        >,
        /// The maximum number of city-networks to monitor for your resources. A city-network is the location (city) where clients access your application resources from and the network or ASN, such as an internet service provider (ISP), that clients access the resources through. This limit helps control billing costs.
        pub max_city_networks_to_monitor: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the monitor.
        ///
        /// The following arguments are optional:
        pub monitor_name: pulumi_wasm_rust::Output<String>,
        /// The resources to include in a monitor, which you provide as a set of Amazon Resource Names (ARNs).
        pub resources: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The status for a monitor. The accepted values for Status with the UpdateMonitor API call are the following: `ACTIVE` and `INACTIVE`.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The percentage of the internet-facing traffic for your application that you want to monitor with this monitor.
        pub traffic_percentage_to_monitor: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InternetMonitorArgs) -> InternetMonitorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let health_events_config_binding = args.health_events_config.get_inner();
        let internet_measurements_log_delivery_binding = args
            .internet_measurements_log_delivery
            .get_inner();
        let max_city_networks_to_monitor_binding = args
            .max_city_networks_to_monitor
            .get_inner();
        let monitor_name_binding = args.monitor_name.get_inner();
        let resources_binding = args.resources.get_inner();
        let status_binding = args.status.get_inner();
        let tags_binding = args.tags.get_inner();
        let traffic_percentage_to_monitor_binding = args
            .traffic_percentage_to_monitor
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "healthEventsConfig".into(),
                },
                register_interface::ResultField {
                    name: "internetMeasurementsLogDelivery".into(),
                },
                register_interface::ResultField {
                    name: "maxCityNetworksToMonitor".into(),
                },
                register_interface::ResultField {
                    name: "monitorName".into(),
                },
                register_interface::ResultField {
                    name: "resources".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "trafficPercentageToMonitor".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InternetMonitorResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            health_events_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthEventsConfig").unwrap(),
            ),
            internet_measurements_log_delivery: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internetMeasurementsLogDelivery").unwrap(),
            ),
            max_city_networks_to_monitor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxCityNetworksToMonitor").unwrap(),
            ),
            monitor_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitorName").unwrap(),
            ),
            resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resources").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            traffic_percentage_to_monitor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficPercentageToMonitor").unwrap(),
            ),
        }
    }
}
