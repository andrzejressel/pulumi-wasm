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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod alert_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertPolicyArgs {
        /// Control over how this alert policy's notification channels are notified.
        #[builder(into, default)]
        pub alert_strategy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::AlertPolicyAlertStrategy>,
        >,
        /// How to combine the results of multiple conditions to
        /// determine if an incident should be opened.
        /// Possible values are: `AND`, `OR`, `AND_WITH_MATCHING_RESOURCE`.
        #[builder(into)]
        pub combiner: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of conditions for the policy. The conditions are combined by
        /// AND or OR according to the combiner field. If the combined conditions
        /// evaluate to true, then an incident is created. A policy can have from
        /// one to six conditions.
        /// Structure is documented below.
        #[builder(into)]
        pub conditions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::monitoring::AlertPolicyCondition>,
        >,
        /// A short name or phrase used to identify the policy in
        /// dashboards, notifications, and incidents. To avoid confusion, don't use
        /// the same display name for multiple policies in the same project. The
        /// name is limited to 512 Unicode characters.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Documentation that is included with notifications and incidents related to this policy. Best practice is for the
        /// documentation to include information to help responders understand, mitigate, escalate, and correct the underlying
        /// problems detected by the alerting policy. Notification channels that have limited capacity might not show this
        /// documentation.
        #[builder(into, default)]
        pub documentation: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::AlertPolicyDocumentation>,
        >,
        /// Whether or not the policy is enabled. The default is true.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Identifies the notification channels to which notifications should be sent when incidents are opened or closed or when
        /// new violations occur on an already opened incident. Each element of this array corresponds to the name field in each of
        /// the NotificationChannel objects that are returned from the notificationChannels.list method. The syntax of the entries
        /// in this field is 'projects/[PROJECT_ID]/notificationChannels/[CHANNEL_ID]'
        #[builder(into, default)]
        pub notification_channels: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The severity of an alert policy indicates how important incidents generated by that policy are. The severity level will
        /// be displayed on the Incident detail page and in notifications. Possible values: ["CRITICAL", "ERROR", "WARNING"]
        #[builder(into, default)]
        pub severity: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This field is intended to be used for organizing and identifying the AlertPolicy objects.The field can contain up to 64
        /// entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values
        /// can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter.
        #[builder(into, default)]
        pub user_labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AlertPolicyResult {
        /// Control over how this alert policy's notification channels are notified.
        pub alert_strategy: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::AlertPolicyAlertStrategy>,
        >,
        /// How to combine the results of multiple conditions to
        /// determine if an incident should be opened.
        /// Possible values are: `AND`, `OR`, `AND_WITH_MATCHING_RESOURCE`.
        pub combiner: pulumi_gestalt_rust::Output<String>,
        /// A list of conditions for the policy. The conditions are combined by
        /// AND or OR according to the combiner field. If the combined conditions
        /// evaluate to true, then an incident is created. A policy can have from
        /// one to six conditions.
        /// Structure is documented below.
        pub conditions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::monitoring::AlertPolicyCondition>,
        >,
        /// A read-only record of the creation of the alerting policy.
        /// If provided in a call to create or update, this field will
        /// be ignored.
        /// Structure is documented below.
        pub creation_records: pulumi_gestalt_rust::Output<
            Vec<super::super::types::monitoring::AlertPolicyCreationRecord>,
        >,
        /// A short name or phrase used to identify the policy in
        /// dashboards, notifications, and incidents. To avoid confusion, don't use
        /// the same display name for multiple policies in the same project. The
        /// name is limited to 512 Unicode characters.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Documentation that is included with notifications and incidents related to this policy. Best practice is for the
        /// documentation to include information to help responders understand, mitigate, escalate, and correct the underlying
        /// problems detected by the alerting policy. Notification channels that have limited capacity might not show this
        /// documentation.
        pub documentation: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::AlertPolicyDocumentation>,
        >,
        /// Whether or not the policy is enabled. The default is true.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The unique resource name for this policy.
        /// Its syntax is: projects/[PROJECT_ID]/alertPolicies/[ALERT_POLICY_ID]
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Identifies the notification channels to which notifications should be sent when incidents are opened or closed or when
        /// new violations occur on an already opened incident. Each element of this array corresponds to the name field in each of
        /// the NotificationChannel objects that are returned from the notificationChannels.list method. The syntax of the entries
        /// in this field is 'projects/[PROJECT_ID]/notificationChannels/[CHANNEL_ID]'
        pub notification_channels: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The severity of an alert policy indicates how important incidents generated by that policy are. The severity level will
        /// be displayed on the Incident detail page and in notifications. Possible values: ["CRITICAL", "ERROR", "WARNING"]
        pub severity: pulumi_gestalt_rust::Output<Option<String>>,
        /// This field is intended to be used for organizing and identifying the AlertPolicy objects.The field can contain up to 64
        /// entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values
        /// can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter.
        pub user_labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AlertPolicyArgs,
    ) -> AlertPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alert_strategy_binding = args.alert_strategy.get_output(context);
        let combiner_binding = args.combiner.get_output(context);
        let conditions_binding = args.conditions.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let documentation_binding = args.documentation.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let notification_channels_binding = args
            .notification_channels
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let severity_binding = args.severity.get_output(context);
        let user_labels_binding = args.user_labels.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:monitoring/alertPolicy:AlertPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alertStrategy".into(),
                    value: alert_strategy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "combiner".into(),
                    value: combiner_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "conditions".into(),
                    value: conditions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "documentation".into(),
                    value: documentation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationChannels".into(),
                    value: notification_channels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "severity".into(),
                    value: severity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userLabels".into(),
                    value: user_labels_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AlertPolicyResult {
            alert_strategy: o.get_field("alertStrategy"),
            combiner: o.get_field("combiner"),
            conditions: o.get_field("conditions"),
            creation_records: o.get_field("creationRecords"),
            display_name: o.get_field("displayName"),
            documentation: o.get_field("documentation"),
            enabled: o.get_field("enabled"),
            name: o.get_field("name"),
            notification_channels: o.get_field("notificationChannels"),
            project: o.get_field("project"),
            severity: o.get_field("severity"),
            user_labels: o.get_field("userLabels"),
        }
    }
}
