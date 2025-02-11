/// Manages an Alert Processing Rule which suppress notifications.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAlertProcessingRuleSuppression:
///     type: azure:monitoring:AlertProcessingRuleSuppression
///     name: example
///     properties:
///       name: example
///       resourceGroupName: example
///       scopes:
///         - ${example.id}
///       condition:
///         targetResourceType:
///           operator: Equals
///           values:
///             - Microsoft.Compute/VirtualMachines
///         severity:
///           operator: Equals
///           values:
///             - Sev0
///             - Sev1
///             - Sev2
///       schedule:
///         effectiveFrom: 2022-01-01T01:02:03
///         effectiveUntil: 2022-02-02T01:02:03
///         timeZone: Pacific Standard Time
///         recurrence:
///           dailies:
///             - startTime: 17:00:00
///               endTime: 09:00:00
///           weeklies:
///             - daysOfWeeks:
///                 - Saturday
///                 - Sunday
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Alert Processing Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/alertProcessingRuleSuppression:AlertProcessingRuleSuppression example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.AlertsManagement/actionRules/actionRule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod alert_processing_rule_suppression {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertProcessingRuleSuppressionArgs {
        /// A `condition` block as defined below.
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::monitoring::AlertProcessingRuleSuppressionCondition,
            >,
        >,
        /// Specifies a description for the Alert Processing Rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the Alert Processing Rule be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this Alert Processing Rule. Changing this forces a new Alert Processing Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Alert Processing Rule should exist. Changing this forces a new Alert Processing Rule to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `schedule` block as defined below.
        #[builder(into, default)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::monitoring::AlertProcessingRuleSuppressionSchedule,
            >,
        >,
        /// A list of resource IDs which will be the target of Alert Processing Rule.
        #[builder(into)]
        pub scopes: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A mapping of tags which should be assigned to the Alert Processing Rule.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AlertProcessingRuleSuppressionResult {
        /// A `condition` block as defined below.
        pub condition: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::monitoring::AlertProcessingRuleSuppressionCondition,
            >,
        >,
        /// Specifies a description for the Alert Processing Rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should the Alert Processing Rule be enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name which should be used for this Alert Processing Rule. Changing this forces a new Alert Processing Rule to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Alert Processing Rule should exist. Changing this forces a new Alert Processing Rule to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `schedule` block as defined below.
        pub schedule: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::monitoring::AlertProcessingRuleSuppressionSchedule,
            >,
        >,
        /// A list of resource IDs which will be the target of Alert Processing Rule.
        pub scopes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A mapping of tags which should be assigned to the Alert Processing Rule.
        pub tags: pulumi_gestalt_rust::Output<
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
        args: AlertProcessingRuleSuppressionArgs,
    ) -> AlertProcessingRuleSuppressionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let description_binding = args.description.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let scopes_binding = args.scopes.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:monitoring/alertProcessingRuleSuppression:AlertProcessingRuleSuppression"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AlertProcessingRuleSuppressionResult {
            condition: o.get_field("condition"),
            description: o.get_field("description"),
            enabled: o.get_field("enabled"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            schedule: o.get_field("schedule"),
            scopes: o.get_field("scopes"),
            tags: o.get_field("tags"),
        }
    }
}
