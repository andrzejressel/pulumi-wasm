/// A description of the conditions under which some aspect of your system is
/// considered to be "unhealthy" and the ways to notify people or services
/// about this state.
///
///
/// To get more information about AlertPolicy, see:
///
/// * [API documentation](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.alertPolicies)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/monitoring/alerts/)
///
/// ## Example Usage
///
/// ### Monitoring Alert Policy Basic
///
///
/// ```yaml
/// resources:
///   alertPolicy:
///     type: gcp:monitoring:AlertPolicy
///     name: alert_policy
///     properties:
///       displayName: My Alert Policy
///       combiner: OR
///       conditions:
///         - displayName: test condition
///           conditionThreshold:
///             filter: metric.type="compute.googleapis.com/instance/disk/write_bytes_count" AND resource.type="gce_instance"
///             duration: 60s
///             comparison: COMPARISON_GT
///             aggregations:
///               - alignmentPeriod: 60s
///                 perSeriesAligner: ALIGN_RATE
///       userLabels:
///         foo: bar
/// ```
/// ### Monitoring Alert Policy Evaluation Missing Data
///
///
/// ```yaml
/// resources:
///   alertPolicy:
///     type: gcp:monitoring:AlertPolicy
///     name: alert_policy
///     properties:
///       displayName: My Alert Policy
///       combiner: OR
///       conditions:
///         - displayName: test condition
///           conditionThreshold:
///             filter: metric.type="compute.googleapis.com/instance/disk/write_bytes_count" AND resource.type="gce_instance"
///             duration: 60s
///             comparison: COMPARISON_GT
///             aggregations:
///               - alignmentPeriod: 60s
///                 perSeriesAligner: ALIGN_RATE
///             evaluationMissingData: EVALUATION_MISSING_DATA_INACTIVE
///       userLabels:
///         foo: bar
/// ```
/// ### Monitoring Alert Policy Forecast Options
///
///
/// ```yaml
/// resources:
///   alertPolicy:
///     type: gcp:monitoring:AlertPolicy
///     name: alert_policy
///     properties:
///       displayName: My Alert Policy
///       combiner: OR
///       conditions:
///         - displayName: test condition
///           conditionThreshold:
///             filter: metric.type="compute.googleapis.com/instance/disk/write_bytes_count" AND resource.type="gce_instance"
///             duration: 60s
///             forecastOptions:
///               forecastHorizon: 3600s
///             comparison: COMPARISON_GT
///             aggregations:
///               - alignmentPeriod: 60s
///                 perSeriesAligner: ALIGN_RATE
///       userLabels:
///         foo: bar
/// ```
/// ### Monitoring Alert Policy Promql Condition
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let alertPolicy = alert_policy::create(
///         "alertPolicy",
///         AlertPolicyArgs::builder()
///             .alert_strategy(
///                 AlertPolicyAlertStrategy::builder().autoClose("1800s").build_struct(),
///             )
///             .combiner("OR")
///             .conditions(
///                 vec![
///                     AlertPolicyCondition::builder()
///                     .conditionPrometheusQueryLanguage(AlertPolicyConditionConditionPrometheusQueryLanguage::builder()
///                     .alertRule("AlwaysOn").duration("60s").evaluationInterval("60s")
///                     .query("compute_googleapis_com:instance_cpu_usage_time > 0")
///                     .ruleGroup("a test").build_struct()).displayName("test condition")
///                     .build_struct(),
///                 ],
///             )
///             .display_name("My Alert Policy")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// AlertPolicy can be imported using any of these accepted formats:
///
/// * `{{project}}/{{name}}`
///
/// * `{{project}} {{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, AlertPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:monitoring/alertPolicy:AlertPolicy default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:monitoring/alertPolicy:AlertPolicy default "{{project}} {{name}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:monitoring/alertPolicy:AlertPolicy default {{name}}
/// ```
///
pub mod alert_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertPolicyArgs {
        /// Control over how this alert policy's notification channels are notified.
        #[builder(into, default)]
        pub alert_strategy: pulumi_wasm_rust::Output<
            Option<super::super::types::monitoring::AlertPolicyAlertStrategy>,
        >,
        /// How to combine the results of multiple conditions to
        /// determine if an incident should be opened.
        /// Possible values are: `AND`, `OR`, `AND_WITH_MATCHING_RESOURCE`.
        #[builder(into)]
        pub combiner: pulumi_wasm_rust::Output<String>,
        /// A list of conditions for the policy. The conditions are combined by
        /// AND or OR according to the combiner field. If the combined conditions
        /// evaluate to true, then an incident is created. A policy can have from
        /// one to six conditions.
        /// Structure is documented below.
        #[builder(into)]
        pub conditions: pulumi_wasm_rust::Output<
            Vec<super::super::types::monitoring::AlertPolicyCondition>,
        >,
        /// A short name or phrase used to identify the policy in
        /// dashboards, notifications, and incidents. To avoid confusion, don't use
        /// the same display name for multiple policies in the same project. The
        /// name is limited to 512 Unicode characters.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Documentation that is included with notifications and incidents related to this policy. Best practice is for the
        /// documentation to include information to help responders understand, mitigate, escalate, and correct the underlying
        /// problems detected by the alerting policy. Notification channels that have limited capacity might not show this
        /// documentation.
        #[builder(into, default)]
        pub documentation: pulumi_wasm_rust::Output<
            Option<super::super::types::monitoring::AlertPolicyDocumentation>,
        >,
        /// Whether or not the policy is enabled. The default is true.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Identifies the notification channels to which notifications should be sent when incidents are opened or closed or when
        /// new violations occur on an already opened incident. Each element of this array corresponds to the name field in each of
        /// the NotificationChannel objects that are returned from the notificationChannels.list method. The syntax of the entries
        /// in this field is 'projects/[PROJECT_ID]/notificationChannels/[CHANNEL_ID]'
        #[builder(into, default)]
        pub notification_channels: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The severity of an alert policy indicates how important incidents generated by that policy are. The severity level will
        /// be displayed on the Incident detail page and in notifications. Possible values: ["CRITICAL", "ERROR", "WARNING"]
        #[builder(into, default)]
        pub severity: pulumi_wasm_rust::Output<Option<String>>,
        /// This field is intended to be used for organizing and identifying the AlertPolicy objects.The field can contain up to 64
        /// entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values
        /// can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter.
        #[builder(into, default)]
        pub user_labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AlertPolicyResult {
        /// Control over how this alert policy's notification channels are notified.
        pub alert_strategy: pulumi_wasm_rust::Output<
            Option<super::super::types::monitoring::AlertPolicyAlertStrategy>,
        >,
        /// How to combine the results of multiple conditions to
        /// determine if an incident should be opened.
        /// Possible values are: `AND`, `OR`, `AND_WITH_MATCHING_RESOURCE`.
        pub combiner: pulumi_wasm_rust::Output<String>,
        /// A list of conditions for the policy. The conditions are combined by
        /// AND or OR according to the combiner field. If the combined conditions
        /// evaluate to true, then an incident is created. A policy can have from
        /// one to six conditions.
        /// Structure is documented below.
        pub conditions: pulumi_wasm_rust::Output<
            Vec<super::super::types::monitoring::AlertPolicyCondition>,
        >,
        /// A read-only record of the creation of the alerting policy.
        /// If provided in a call to create or update, this field will
        /// be ignored.
        /// Structure is documented below.
        pub creation_records: pulumi_wasm_rust::Output<
            Vec<super::super::types::monitoring::AlertPolicyCreationRecord>,
        >,
        /// A short name or phrase used to identify the policy in
        /// dashboards, notifications, and incidents. To avoid confusion, don't use
        /// the same display name for multiple policies in the same project. The
        /// name is limited to 512 Unicode characters.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Documentation that is included with notifications and incidents related to this policy. Best practice is for the
        /// documentation to include information to help responders understand, mitigate, escalate, and correct the underlying
        /// problems detected by the alerting policy. Notification channels that have limited capacity might not show this
        /// documentation.
        pub documentation: pulumi_wasm_rust::Output<
            Option<super::super::types::monitoring::AlertPolicyDocumentation>,
        >,
        /// Whether or not the policy is enabled. The default is true.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The unique resource name for this policy.
        /// Its syntax is: projects/[PROJECT_ID]/alertPolicies/[ALERT_POLICY_ID]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Identifies the notification channels to which notifications should be sent when incidents are opened or closed or when
        /// new violations occur on an already opened incident. Each element of this array corresponds to the name field in each of
        /// the NotificationChannel objects that are returned from the notificationChannels.list method. The syntax of the entries
        /// in this field is 'projects/[PROJECT_ID]/notificationChannels/[CHANNEL_ID]'
        pub notification_channels: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The severity of an alert policy indicates how important incidents generated by that policy are. The severity level will
        /// be displayed on the Incident detail page and in notifications. Possible values: ["CRITICAL", "ERROR", "WARNING"]
        pub severity: pulumi_wasm_rust::Output<Option<String>>,
        /// This field is intended to be used for organizing and identifying the AlertPolicy objects.The field can contain up to 64
        /// entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values
        /// can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter.
        pub user_labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AlertPolicyArgs) -> AlertPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alert_strategy_binding = args.alert_strategy.get_inner();
        let combiner_binding = args.combiner.get_inner();
        let conditions_binding = args.conditions.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let documentation_binding = args.documentation.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let notification_channels_binding = args.notification_channels.get_inner();
        let project_binding = args.project.get_inner();
        let severity_binding = args.severity.get_inner();
        let user_labels_binding = args.user_labels.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:monitoring/alertPolicy:AlertPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alertStrategy".into(),
                    value: &alert_strategy_binding,
                },
                register_interface::ObjectField {
                    name: "combiner".into(),
                    value: &combiner_binding,
                },
                register_interface::ObjectField {
                    name: "conditions".into(),
                    value: &conditions_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "documentation".into(),
                    value: &documentation_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "notificationChannels".into(),
                    value: &notification_channels_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "severity".into(),
                    value: &severity_binding,
                },
                register_interface::ObjectField {
                    name: "userLabels".into(),
                    value: &user_labels_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alertStrategy".into(),
                },
                register_interface::ResultField {
                    name: "combiner".into(),
                },
                register_interface::ResultField {
                    name: "conditions".into(),
                },
                register_interface::ResultField {
                    name: "creationRecords".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "documentation".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notificationChannels".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "severity".into(),
                },
                register_interface::ResultField {
                    name: "userLabels".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AlertPolicyResult {
            alert_strategy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alertStrategy").unwrap(),
            ),
            combiner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("combiner").unwrap(),
            ),
            conditions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("conditions").unwrap(),
            ),
            creation_records: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationRecords").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            documentation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("documentation").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notification_channels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationChannels").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            severity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("severity").unwrap(),
            ),
            user_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userLabels").unwrap(),
            ),
        }
    }
}
