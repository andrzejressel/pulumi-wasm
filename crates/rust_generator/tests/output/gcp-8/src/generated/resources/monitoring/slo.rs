/// A Service-Level Objective (SLO) describes the level of desired good
/// service. It consists of a service-level indicator (SLI), a performance
/// goal, and a period over which the objective is to be evaluated against
/// that goal. The SLO can use SLIs defined in a number of different manners.
/// Typical SLOs might include "99% of requests in each rolling week have
/// latency below 200 milliseconds" or "99.5% of requests in each calendar
/// month return successfully."
///
///
/// To get more information about Slo, see:
///
/// * [API documentation](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/services.serviceLevelObjectives)
/// * How-to Guides
///     * [Monitoring API Documentation](https://cloud.google.com/monitoring/api/v3/)
///     * [Service Monitoring](https://cloud.google.com/monitoring/service-monitoring)
///
/// ## Example Usage
///
/// ### Monitoring Slo Appengine
///
///
/// ```yaml
/// resources:
///   appengSlo:
///     type: gcp:monitoring:Slo
///     name: appeng_slo
///     properties:
///       service: ${default.serviceId}
///       sloId: ae-slo
///       displayName: Test SLO for App Engine
///       goal: 0.9
///       calendarPeriod: DAY
///       basicSli:
///         latency:
///           threshold: 1s
///       userLabels:
///         my_key: my_value
///         my_other_key: my_other_value
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:monitoring:getAppEngineService
///       arguments:
///         moduleId: default
/// ```
/// ### Monitoring Slo Request Based
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let customsrv = custom_service::create(
///         "customsrv",
///         CustomServiceArgs::builder()
///             .display_name("My Custom Service")
///             .service_id("custom-srv-request-slos")
///             .build_struct(),
///     );
///     let requestBasedSlo = slo::create(
///         "requestBasedSlo",
///         SloArgs::builder()
///             .display_name("Test SLO with request based SLI (good total ratio)")
///             .goal(0.9)
///             .request_based_sli(
///                 SloRequestBasedSli::builder()
///                     .distributionCut(
///                         SloRequestBasedSliDistributionCut::builder()
///                             .distributionFilter(
///                                 "metric.type=\"serviceruntime.googleapis.com/api/request_latencies\" resource.type=\"api\"  ",
///                             )
///                             .range(
///                                 SloRequestBasedSliDistributionCutRange::builder()
///                                     .max(0.5)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .rolling_period_days(30)
///             .service("${customsrv.serviceId}")
///             .slo_id("consumed-api-slo")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Monitoring Slo Windows Based Good Bad Metric Filter
///
///
/// ```yaml
/// resources:
///   customsrv:
///     type: gcp:monitoring:CustomService
///     properties:
///       serviceId: custom-srv-windows-slos
///       displayName: My Custom Service
///   windowsBased:
///     type: gcp:monitoring:Slo
///     name: windows_based
///     properties:
///       service: ${customsrv.serviceId}
///       displayName: Test SLO with window based SLI
///       goal: 0.95
///       calendarPeriod: FORTNIGHT
///       windowsBasedSli:
///         windowPeriod: 400s
///         goodBadMetricFilter:
///           fn::invoke:
///             function: std:join
///             arguments:
///               separator: ' AND '
///               input:
///                 - metric.type="monitoring.googleapis.com/uptime_check/check_passed"
///                 - resource.type="uptime_url"
///             return: result
/// ```
/// ### Monitoring Slo Windows Based Metric Mean
///
///
/// ```yaml
/// resources:
///   customsrv:
///     type: gcp:monitoring:CustomService
///     properties:
///       serviceId: custom-srv-windows-slos
///       displayName: My Custom Service
///   windowsBased:
///     type: gcp:monitoring:Slo
///     name: windows_based
///     properties:
///       service: ${customsrv.serviceId}
///       displayName: Test SLO with window based SLI
///       goal: 0.9
///       rollingPeriodDays: 20
///       windowsBasedSli:
///         windowPeriod: 600s
///         metricMeanInRange:
///           timeSeries:
///             fn::invoke:
///               function: std:join
///               arguments:
///                 separator: ' AND '
///                 input:
///                   - metric.type="agent.googleapis.com/cassandra/client_request/latency/95p"
///                   - resource.type="gce_instance"
///               return: result
///           range:
///             max: 5
/// ```
/// ### Monitoring Slo Windows Based Metric Sum
///
///
/// ```yaml
/// resources:
///   customsrv:
///     type: gcp:monitoring:CustomService
///     properties:
///       serviceId: custom-srv-windows-slos
///       displayName: My Custom Service
///   windowsBased:
///     type: gcp:monitoring:Slo
///     name: windows_based
///     properties:
///       service: ${customsrv.serviceId}
///       displayName: Test SLO with window based SLI
///       goal: 0.9
///       rollingPeriodDays: 20
///       windowsBasedSli:
///         windowPeriod: 400s
///         metricSumInRange:
///           timeSeries:
///             fn::invoke:
///               function: std:join
///               arguments:
///                 separator: ' AND '
///                 input:
///                   - metric.type="monitoring.googleapis.com/uptime_check/request_latency"
///                   - resource.type="uptime_url"
///               return: result
///           range:
///             max: 5000
/// ```
/// ### Monitoring Slo Windows Based Ratio Threshold
///
///
/// ```yaml
/// resources:
///   customsrv:
///     type: gcp:monitoring:CustomService
///     properties:
///       serviceId: custom-srv-windows-slos
///       displayName: My Custom Service
///   windowsBased:
///     type: gcp:monitoring:Slo
///     name: windows_based
///     properties:
///       service: ${customsrv.serviceId}
///       displayName: Test SLO with window based SLI
///       goal: 0.9
///       rollingPeriodDays: 20
///       windowsBasedSli:
///         windowPeriod: 100s
///         goodTotalRatioThreshold:
///           threshold: 0.1
///           performance:
///             distributionCut:
///               distributionFilter:
///                 fn::invoke:
///                   function: std:join
///                   arguments:
///                     separator: ' AND '
///                     input:
///                       - metric.type="serviceruntime.googleapis.com/api/request_latencies"
///                       - resource.type="consumed_api"
///                   return: result
///               range:
///                 min: 1
///                 max: 9
/// ```
///
/// ## Import
///
/// Slo can be imported using any of these accepted formats:
///
/// * `{{project}}/{{name}}`
///
/// * `{{project}} {{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Slo can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:monitoring/slo:Slo default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:monitoring/slo:Slo default "{{project}} {{name}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:monitoring/slo:Slo default {{name}}
/// ```
///
pub mod slo {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SloArgs {
        /// Basic Service-Level Indicator (SLI) on a well-known service type.
        /// Performance will be computed on the basis of pre-defined metrics.
        /// SLIs are used to measure and calculate the quality of the Service's
        /// performance with respect to a single aspect of service quality.
        /// Exactly one of the following must be set:
        /// `basic_sli`, `request_based_sli`, `windows_based_sli`
        /// Structure is documented below.
        #[builder(into, default)]
        pub basic_sli: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::SloBasicSli>,
        >,
        /// A calendar period, semantically "since the start of the current
        /// <calendarPeriod>".
        /// Possible values are: `DAY`, `WEEK`, `FORTNIGHT`, `MONTH`.
        #[builder(into, default)]
        pub calendar_period: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name used for UI elements listing this SLO.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The fraction of service that must be good in order for this objective
        /// to be met. 0 < goal <= 0.999
        #[builder(into)]
        pub goal: pulumi_gestalt_rust::InputOrOutput<f64>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A request-based SLI defines a SLI for which atomic units of
        /// service are counted directly.
        /// A SLI describes a good service.
        /// It is used to measure and calculate the quality of the Service's
        /// performance with respect to a single aspect of service quality.
        /// Exactly one of the following must be set:
        /// `basic_sli`, `request_based_sli`, `windows_based_sli`
        /// Structure is documented below.
        #[builder(into, default)]
        pub request_based_sli: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::SloRequestBasedSli>,
        >,
        /// A rolling time period, semantically "in the past X days".
        /// Must be between 1 to 30 days, inclusive.
        #[builder(into, default)]
        pub rolling_period_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// ID of the service to which this SLO belongs.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id to use for this ServiceLevelObjective. If omitted, an id will be generated instead.
        #[builder(into, default)]
        pub slo_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This field is intended to be used for organizing and identifying the AlertPolicy
        /// objects.The field can contain up to 64 entries. Each key and value is limited
        /// to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values
        /// can contain only lowercase letters, numerals, underscores, and dashes. Keys
        /// must begin with a letter.
        #[builder(into, default)]
        pub user_labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A windows-based SLI defines the criteria for time windows.
        /// good_service is defined based off the count of these time windows
        /// for which the provided service was of good quality.
        /// A SLI describes a good service. It is used to measure and calculate
        /// the quality of the Service's performance with respect to a single
        /// aspect of service quality.
        /// Exactly one of the following must be set:
        /// `basic_sli`, `request_based_sli`, `windows_based_sli`
        /// Structure is documented below.
        #[builder(into, default)]
        pub windows_based_sli: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::SloWindowsBasedSli>,
        >,
    }
    #[allow(dead_code)]
    pub struct SloResult {
        /// Basic Service-Level Indicator (SLI) on a well-known service type.
        /// Performance will be computed on the basis of pre-defined metrics.
        /// SLIs are used to measure and calculate the quality of the Service's
        /// performance with respect to a single aspect of service quality.
        /// Exactly one of the following must be set:
        /// `basic_sli`, `request_based_sli`, `windows_based_sli`
        /// Structure is documented below.
        pub basic_sli: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::SloBasicSli>,
        >,
        /// A calendar period, semantically "since the start of the current
        /// <calendarPeriod>".
        /// Possible values are: `DAY`, `WEEK`, `FORTNIGHT`, `MONTH`.
        pub calendar_period: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name used for UI elements listing this SLO.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The fraction of service that must be good in order for this objective
        /// to be met. 0 < goal <= 0.999
        pub goal: pulumi_gestalt_rust::Output<f64>,
        /// The full resource name for this service. The syntax is:
        /// projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]/serviceLevelObjectives/[SLO_NAME]
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// A request-based SLI defines a SLI for which atomic units of
        /// service are counted directly.
        /// A SLI describes a good service.
        /// It is used to measure and calculate the quality of the Service's
        /// performance with respect to a single aspect of service quality.
        /// Exactly one of the following must be set:
        /// `basic_sli`, `request_based_sli`, `windows_based_sli`
        /// Structure is documented below.
        pub request_based_sli: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::SloRequestBasedSli>,
        >,
        /// A rolling time period, semantically "in the past X days".
        /// Must be between 1 to 30 days, inclusive.
        pub rolling_period_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// ID of the service to which this SLO belongs.
        ///
        ///
        /// - - -
        pub service: pulumi_gestalt_rust::Output<String>,
        /// The id to use for this ServiceLevelObjective. If omitted, an id will be generated instead.
        pub slo_id: pulumi_gestalt_rust::Output<String>,
        /// This field is intended to be used for organizing and identifying the AlertPolicy
        /// objects.The field can contain up to 64 entries. Each key and value is limited
        /// to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values
        /// can contain only lowercase letters, numerals, underscores, and dashes. Keys
        /// must begin with a letter.
        pub user_labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A windows-based SLI defines the criteria for time windows.
        /// good_service is defined based off the count of these time windows
        /// for which the provided service was of good quality.
        /// A SLI describes a good service. It is used to measure and calculate
        /// the quality of the Service's performance with respect to a single
        /// aspect of service quality.
        /// Exactly one of the following must be set:
        /// `basic_sli`, `request_based_sli`, `windows_based_sli`
        /// Structure is documented below.
        pub windows_based_sli: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::SloWindowsBasedSli>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SloArgs,
    ) -> SloResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let basic_sli_binding = args.basic_sli.get_output(context).get_inner();
        let calendar_period_binding = args
            .calendar_period
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let goal_binding = args.goal.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request_based_sli_binding = args
            .request_based_sli
            .get_output(context)
            .get_inner();
        let rolling_period_days_binding = args
            .rolling_period_days
            .get_output(context)
            .get_inner();
        let service_binding = args.service.get_output(context).get_inner();
        let slo_id_binding = args.slo_id.get_output(context).get_inner();
        let user_labels_binding = args.user_labels.get_output(context).get_inner();
        let windows_based_sli_binding = args
            .windows_based_sli
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:monitoring/slo:Slo".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "basicSli".into(),
                    value: &basic_sli_binding,
                },
                register_interface::ObjectField {
                    name: "calendarPeriod".into(),
                    value: &calendar_period_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "goal".into(),
                    value: &goal_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "requestBasedSli".into(),
                    value: &request_based_sli_binding,
                },
                register_interface::ObjectField {
                    name: "rollingPeriodDays".into(),
                    value: &rolling_period_days_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
                register_interface::ObjectField {
                    name: "sloId".into(),
                    value: &slo_id_binding,
                },
                register_interface::ObjectField {
                    name: "userLabels".into(),
                    value: &user_labels_binding,
                },
                register_interface::ObjectField {
                    name: "windowsBasedSli".into(),
                    value: &windows_based_sli_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SloResult {
            basic_sli: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("basicSli"),
            ),
            calendar_period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("calendarPeriod"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            goal: pulumi_gestalt_rust::__private::into_domain(o.extract_field("goal")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            request_based_sli: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requestBasedSli"),
            ),
            rolling_period_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rollingPeriodDays"),
            ),
            service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("service"),
            ),
            slo_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sloId"),
            ),
            user_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userLabels"),
            ),
            windows_based_sli: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("windowsBasedSli"),
            ),
        }
    }
}
