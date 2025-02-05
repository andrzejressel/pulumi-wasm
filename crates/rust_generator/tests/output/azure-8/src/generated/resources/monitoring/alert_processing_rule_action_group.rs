/// Manages an Alert Processing Rule which apply action group.
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
///   exampleActionGroup:
///     type: azure:monitoring:ActionGroup
///     name: example
///     properties:
///       name: example-action-group
///       resourceGroupName: ${example.name}
///       shortName: action
///   exampleAlertProcessingRuleActionGroup:
///     type: azure:monitoring:AlertProcessingRuleActionGroup
///     name: example
///     properties:
///       name: example
///       resourceGroupName: example
///       scopes:
///         - ${example.id}
///       addActionGroupIds:
///         - ${exampleActionGroup.id}
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
/// $ pulumi import azure:monitoring/alertProcessingRuleActionGroup:AlertProcessingRuleActionGroup example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.AlertsManagement/actionRules/actionRule1
/// ```
///
pub mod alert_processing_rule_action_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertProcessingRuleActionGroupArgs {
        /// Specifies a list of Action Group IDs.
        #[builder(into)]
        pub add_action_group_ids: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// A `condition` block as defined below.
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::monitoring::AlertProcessingRuleActionGroupCondition,
            >,
        >,
        /// Specifies a description for the Alert Processing Rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Should the Alert Processing Rule be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this Alert Processing Rule. Changing this forces a new Alert Processing Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Alert Processing Rule should exist. Changing this forces a new Alert Processing Rule to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `schedule` block as defined below.
        #[builder(into, default)]
        pub schedule: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::monitoring::AlertProcessingRuleActionGroupSchedule,
            >,
        >,
        /// A list of resource IDs which will be the target of alert processing rule.
        #[builder(into)]
        pub scopes: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// A mapping of tags which should be assigned to the Alert Processing Rule.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AlertProcessingRuleActionGroupResult {
        /// Specifies a list of Action Group IDs.
        pub add_action_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A `condition` block as defined below.
        pub condition: pulumi_wasm_rust::Output<
            Option<
                super::super::types::monitoring::AlertProcessingRuleActionGroupCondition,
            >,
        >,
        /// Specifies a description for the Alert Processing Rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Alert Processing Rule be enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Alert Processing Rule. Changing this forces a new Alert Processing Rule to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Alert Processing Rule should exist. Changing this forces a new Alert Processing Rule to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `schedule` block as defined below.
        pub schedule: pulumi_wasm_rust::Output<
            Option<
                super::super::types::monitoring::AlertProcessingRuleActionGroupSchedule,
            >,
        >,
        /// A list of resource IDs which will be the target of alert processing rule.
        pub scopes: pulumi_wasm_rust::Output<Vec<String>>,
        /// A mapping of tags which should be assigned to the Alert Processing Rule.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AlertProcessingRuleActionGroupArgs,
    ) -> AlertProcessingRuleActionGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let add_action_group_ids_binding = args
            .add_action_group_ids
            .get_output(context)
            .get_inner();
        let condition_binding = args.condition.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let schedule_binding = args.schedule.get_output(context).get_inner();
        let scopes_binding = args.scopes.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/alertProcessingRuleActionGroup:AlertProcessingRuleActionGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addActionGroupIds".into(),
                    value: &add_action_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AlertProcessingRuleActionGroupResult {
            add_action_group_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("addActionGroupIds"),
            ),
            condition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("condition"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            schedule: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("schedule"),
            ),
            scopes: pulumi_wasm_rust::__private::into_domain(o.extract_field("scopes")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
