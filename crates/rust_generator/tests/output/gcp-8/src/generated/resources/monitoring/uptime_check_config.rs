/// This message configures which resources and services to monitor for availability.
///
///
/// To get more information about UptimeCheckConfig, see:
///
/// * [API documentation](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.uptimeCheckConfigs)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/monitoring/uptime-checks/)
///
///
///
/// ## Example Usage
///
/// ### Uptime Check Config Http
///
///
/// ```yaml
/// resources:
///   http:
///     type: gcp:monitoring:UptimeCheckConfig
///     properties:
///       displayName: http-uptime-check
///       timeout: 60s
///       userLabels:
///         example-key: example-value
///       httpCheck:
///         path: some-path
///         port: '8010'
///         requestMethod: POST
///         contentType: USER_PROVIDED
///         customContentType: application/json
///         body: Zm9vJTI1M0RiYXI=
///         pingConfig:
///           pingsCount: 1
///       monitoredResource:
///         type: uptime_url
///         labels:
///           project_id: my-project-name
///           host: 192.168.1.1
///       contentMatchers:
///         - content: '"example"'
///           matcher: MATCHES_JSON_PATH
///           jsonPathMatcher:
///             jsonPath: $.path
///             jsonMatcher: EXACT_MATCH
///       checkerType: STATIC_IP_CHECKERS
/// ```
/// ### Uptime Check Config Status Code
///
///
/// ```yaml
/// resources:
///   statusCode:
///     type: gcp:monitoring:UptimeCheckConfig
///     name: status_code
///     properties:
///       displayName: http-uptime-check
///       timeout: 60s
///       httpCheck:
///         path: some-path
///         port: '8010'
///         requestMethod: POST
///         contentType: URL_ENCODED
///         body: Zm9vJTI1M0RiYXI=
///         acceptedResponseStatusCodes:
///           - statusClass: STATUS_CLASS_2XX
///           - statusValue: 301
///           - statusValue: 302
///       monitoredResource:
///         type: uptime_url
///         labels:
///           project_id: my-project-name
///           host: 192.168.1.1
///       contentMatchers:
///         - content: '"example"'
///           matcher: MATCHES_JSON_PATH
///           jsonPathMatcher:
///             jsonPath: $.path
///             jsonMatcher: EXACT_MATCH
///       checkerType: STATIC_IP_CHECKERS
/// ```
/// ### Uptime Check Config Https
///
///
/// ```yaml
/// resources:
///   https:
///     type: gcp:monitoring:UptimeCheckConfig
///     properties:
///       displayName: https-uptime-check
///       timeout: 60s
///       httpCheck:
///         path: /some-path
///         port: '443'
///         useSsl: true
///         validateSsl: true
///         serviceAgentAuthentication:
///           type: OIDC_TOKEN
///       monitoredResource:
///         type: uptime_url
///         labels:
///           project_id: my-project-name
///           host: 192.168.1.1
///       contentMatchers:
///         - content: example
///           matcher: MATCHES_JSON_PATH
///           jsonPathMatcher:
///             jsonPath: $.path
///             jsonMatcher: REGEX_MATCH
/// ```
/// ### Uptime Check Tcp
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let check = group::create(
///         "check",
///         GroupArgs::builder()
///             .display_name("uptime-check-group")
///             .filter("resource.metadata.name=has_substring(\"foo\")")
///             .build_struct(),
///     );
///     let tcpGroup = uptime_check_config::create(
///         "tcpGroup",
///         UptimeCheckConfigArgs::builder()
///             .display_name("tcp-uptime-check")
///             .resource_group(
///                 UptimeCheckConfigResourceGroup::builder()
///                     .groupId("${check.name}")
///                     .resourceType("INSTANCE")
///                     .build_struct(),
///             )
///             .tcp_check(
///                 UptimeCheckConfigTcpCheck::builder()
///                     .pingConfig(
///                         UptimeCheckConfigTcpCheckPingConfig::builder()
///                             .pingsCount(2)
///                             .build_struct(),
///                     )
///                     .port(888)
///                     .build_struct(),
///             )
///             .timeout("60s")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Uptime Check Config Synthetic Monitor
///
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: my-project-name-gcf-source
///       location: US
///       uniformBucketLevelAccess: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: function-source.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: synthetic-fn-source.zip
///   function:
///     type: gcp:cloudfunctionsv2:Function
///     properties:
///       name: synthetic_function
///       location: us-central1
///       buildConfig:
///         runtime: nodejs16
///         entryPoint: SyntheticFunction
///         source:
///           storageSource:
///             bucket: ${bucket.name}
///             object: ${object.name}
///       serviceConfig:
///         maxInstanceCount: 1
///         availableMemory: 256M
///         timeoutSeconds: 60
///   syntheticMonitor:
///     type: gcp:monitoring:UptimeCheckConfig
///     name: synthetic_monitor
///     properties:
///       displayName: synthetic_monitor
///       timeout: 60s
///       syntheticMonitor:
///         cloudFunctionV2:
///           name: ${function.id}
/// ```
///
/// ## Import
///
/// UptimeCheckConfig can be imported using any of these accepted formats:
///
/// * `{{project}}/{{name}}`
///
/// * `{{project}} {{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, UptimeCheckConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:monitoring/uptimeCheckConfig:UptimeCheckConfig default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:monitoring/uptimeCheckConfig:UptimeCheckConfig default "{{project}} {{name}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:monitoring/uptimeCheckConfig:UptimeCheckConfig default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod uptime_check_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UptimeCheckConfigArgs {
        /// The checker type to use for the check. If the monitored resource type is `servicedirectory_service`, `checker_type` must be set to `VPC_CHECKERS`.
        /// Possible values are: `STATIC_IP_CHECKERS`, `VPC_CHECKERS`.
        #[builder(into, default)]
        pub checker_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The expected content on the page the check is run against. Currently, only the first entry in the list is supported, and other entries will be ignored. The server will look for an exact match of the string in the page response's content. This field is optional and should only be specified if a content match is required.
        /// Structure is documented below.
        #[builder(into, default)]
        pub content_matchers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::UptimeCheckConfigContentMatcher>>,
        >,
        /// A human-friendly name for the uptime check configuration. The display name should be unique within a Stackdriver Workspace in order to make it easier to identify; however, uniqueness is not enforced.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Contains information needed to make an HTTP or HTTPS check.
        /// Structure is documented below.
        #[builder(into, default)]
        pub http_check: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::UptimeCheckConfigHttpCheck>,
        >,
        /// The [monitored resource]
        /// (https://cloud.google.com/monitoring/api/resources) associated with the
        /// configuration. The following monitored resource types are supported for
        /// uptime checks:
        #[builder(into, default)]
        pub monitored_resource: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::UptimeCheckConfigMonitoredResource>,
        >,
        /// How often, in seconds, the uptime check is performed. Currently, the only supported values are 60s (1 minute), 300s (5 minutes), 600s (10 minutes), and 900s (15 minutes). Optional, defaults to 300s.
        #[builder(into, default)]
        pub period: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The group resource associated with the configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub resource_group: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::UptimeCheckConfigResourceGroup>,
        >,
        /// The list of regions from which the check will be run. Some regions contain one location, and others contain more than one. If this field is specified, enough regions to include a minimum of 3 locations must be provided, or an error message is returned. Not specifying this field will result in uptime checks running from all regions.
        #[builder(into, default)]
        pub selected_regions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A Synthetic Monitor deployed to a Cloud Functions V2 instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub synthetic_monitor: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::UptimeCheckConfigSyntheticMonitor>,
        >,
        /// Contains information needed to make a TCP check.
        /// Structure is documented below.
        #[builder(into, default)]
        pub tcp_check: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::UptimeCheckConfigTcpCheck>,
        >,
        /// The maximum amount of time to wait for the request to complete (must be between 1 and 60 seconds). See the accepted formats
        ///
        ///
        /// - - -
        #[builder(into)]
        pub timeout: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User-supplied key/value data to be used for organizing and identifying the `UptimeCheckConfig` objects. The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter.
        #[builder(into, default)]
        pub user_labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct UptimeCheckConfigResult {
        /// The checker type to use for the check. If the monitored resource type is `servicedirectory_service`, `checker_type` must be set to `VPC_CHECKERS`.
        /// Possible values are: `STATIC_IP_CHECKERS`, `VPC_CHECKERS`.
        pub checker_type: pulumi_gestalt_rust::Output<String>,
        /// The expected content on the page the check is run against. Currently, only the first entry in the list is supported, and other entries will be ignored. The server will look for an exact match of the string in the page response's content. This field is optional and should only be specified if a content match is required.
        /// Structure is documented below.
        pub content_matchers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::UptimeCheckConfigContentMatcher>>,
        >,
        /// A human-friendly name for the uptime check configuration. The display name should be unique within a Stackdriver Workspace in order to make it easier to identify; however, uniqueness is not enforced.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Contains information needed to make an HTTP or HTTPS check.
        /// Structure is documented below.
        pub http_check: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::UptimeCheckConfigHttpCheck>,
        >,
        /// The [monitored resource]
        /// (https://cloud.google.com/monitoring/api/resources) associated with the
        /// configuration. The following monitored resource types are supported for
        /// uptime checks:
        pub monitored_resource: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::UptimeCheckConfigMonitoredResource>,
        >,
        /// A unique resource name for this UptimeCheckConfig. The format is `projects/[PROJECT_ID]/uptimeCheckConfigs/[UPTIME_CHECK_ID]`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// How often, in seconds, the uptime check is performed. Currently, the only supported values are 60s (1 minute), 300s (5 minutes), 600s (10 minutes), and 900s (15 minutes). Optional, defaults to 300s.
        pub period: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The group resource associated with the configuration.
        /// Structure is documented below.
        pub resource_group: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::UptimeCheckConfigResourceGroup>,
        >,
        /// The list of regions from which the check will be run. Some regions contain one location, and others contain more than one. If this field is specified, enough regions to include a minimum of 3 locations must be provided, or an error message is returned. Not specifying this field will result in uptime checks running from all regions.
        pub selected_regions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A Synthetic Monitor deployed to a Cloud Functions V2 instance.
        /// Structure is documented below.
        pub synthetic_monitor: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::UptimeCheckConfigSyntheticMonitor>,
        >,
        /// Contains information needed to make a TCP check.
        /// Structure is documented below.
        pub tcp_check: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::UptimeCheckConfigTcpCheck>,
        >,
        /// The maximum amount of time to wait for the request to complete (must be between 1 and 60 seconds). See the accepted formats
        ///
        ///
        /// - - -
        pub timeout: pulumi_gestalt_rust::Output<String>,
        /// The id of the uptime check
        pub uptime_check_id: pulumi_gestalt_rust::Output<String>,
        /// User-supplied key/value data to be used for organizing and identifying the `UptimeCheckConfig` objects. The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter.
        pub user_labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: UptimeCheckConfigArgs,
    ) -> UptimeCheckConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let checker_type_binding_1 = args.checker_type.get_output(context);
        let checker_type_binding = checker_type_binding_1.get_inner();
        let content_matchers_binding_1 = args.content_matchers.get_output(context);
        let content_matchers_binding = content_matchers_binding_1.get_inner();
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let http_check_binding_1 = args.http_check.get_output(context);
        let http_check_binding = http_check_binding_1.get_inner();
        let monitored_resource_binding_1 = args.monitored_resource.get_output(context);
        let monitored_resource_binding = monitored_resource_binding_1.get_inner();
        let period_binding_1 = args.period.get_output(context);
        let period_binding = period_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let resource_group_binding_1 = args.resource_group.get_output(context);
        let resource_group_binding = resource_group_binding_1.get_inner();
        let selected_regions_binding_1 = args.selected_regions.get_output(context);
        let selected_regions_binding = selected_regions_binding_1.get_inner();
        let synthetic_monitor_binding_1 = args.synthetic_monitor.get_output(context);
        let synthetic_monitor_binding = synthetic_monitor_binding_1.get_inner();
        let tcp_check_binding_1 = args.tcp_check.get_output(context);
        let tcp_check_binding = tcp_check_binding_1.get_inner();
        let timeout_binding_1 = args.timeout.get_output(context);
        let timeout_binding = timeout_binding_1.get_inner();
        let user_labels_binding_1 = args.user_labels.get_output(context);
        let user_labels_binding = user_labels_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:monitoring/uptimeCheckConfig:UptimeCheckConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "checkerType".into(),
                    value: &checker_type_binding,
                },
                register_interface::ObjectField {
                    name: "contentMatchers".into(),
                    value: &content_matchers_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "httpCheck".into(),
                    value: &http_check_binding,
                },
                register_interface::ObjectField {
                    name: "monitoredResource".into(),
                    value: &monitored_resource_binding,
                },
                register_interface::ObjectField {
                    name: "period".into(),
                    value: &period_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroup".into(),
                    value: &resource_group_binding,
                },
                register_interface::ObjectField {
                    name: "selectedRegions".into(),
                    value: &selected_regions_binding,
                },
                register_interface::ObjectField {
                    name: "syntheticMonitor".into(),
                    value: &synthetic_monitor_binding,
                },
                register_interface::ObjectField {
                    name: "tcpCheck".into(),
                    value: &tcp_check_binding,
                },
                register_interface::ObjectField {
                    name: "timeout".into(),
                    value: &timeout_binding,
                },
                register_interface::ObjectField {
                    name: "userLabels".into(),
                    value: &user_labels_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        UptimeCheckConfigResult {
            checker_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("checkerType"),
            ),
            content_matchers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentMatchers"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            http_check: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpCheck"),
            ),
            monitored_resource: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitoredResource"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("period"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            resource_group: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroup"),
            ),
            selected_regions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selectedRegions"),
            ),
            synthetic_monitor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("syntheticMonitor"),
            ),
            tcp_check: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tcpCheck"),
            ),
            timeout: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeout"),
            ),
            uptime_check_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("uptimeCheckId"),
            ),
            user_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userLabels"),
            ),
        }
    }
}
