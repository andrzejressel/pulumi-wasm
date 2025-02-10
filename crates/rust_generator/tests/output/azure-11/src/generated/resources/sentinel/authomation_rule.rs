/// Manages a Sentinel Automation Rule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("west europe")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("example-workspace")
///             .resource_group_name("${example.name}")
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleAutomationRule = automation_rule::create(
///         "exampleAutomationRule",
///         AutomationRuleArgs::builder()
///             .action_incidents(
///                 vec![
///                     AutomationRuleActionIncident::builder().order(1).status("Active")
///                     .build_struct(),
///                 ],
///             )
///             .display_name("automation_rule1")
///             .log_analytics_workspace_id(
///                 "${exampleLogAnalyticsWorkspaceOnboarding.workspaceId}",
///             )
///             .name("56094f72-ac3f-40e7-a0c0-47bd95f70336")
///             .order(1)
///             .build_struct(),
///     );
///     let exampleLogAnalyticsWorkspaceOnboarding = log_analytics_workspace_onboarding::create(
///         "exampleLogAnalyticsWorkspaceOnboarding",
///         LogAnalyticsWorkspaceOnboardingArgs::builder()
///             .workspace_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Sentinel Automation Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/authomationRule:AuthomationRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/automationRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod authomation_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthomationRuleArgs {
        /// One or more `action_incident` blocks as defined below.
        #[builder(into, default)]
        pub action_incidents: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::sentinel::AuthomationRuleActionIncident>>,
        >,
        /// One or more `action_playbook` blocks as defined below.
        ///
        /// > **Note:** Either one `action_incident` block or `action_playbook` block has to be specified.
        #[builder(into, default)]
        pub action_playbooks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::sentinel::AuthomationRuleActionPlaybook>>,
        >,
        /// A JSON array of one or more condition JSON objects as is defined [here](https://learn.microsoft.com/en-us/rest/api/securityinsights/preview/automation-rules/create-or-update?tabs=HTTP#automationruletriggeringlogic).
        #[builder(into, default)]
        pub condition_json: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name which should be used for this Sentinel Automation Rule.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether this Sentinel Automation Rule is enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The time in RFC3339 format of kind `UTC` that determines when this Automation Rule should expire and be disabled.
        #[builder(into, default)]
        pub expiration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Log Analytics Workspace where this Sentinel applies to. Changing this forces a new Sentinel Automation Rule to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The UUID which should be used for this Sentinel Automation Rule. Changing this forces a new Sentinel Automation Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The order of this Sentinel Automation Rule. Possible values varies between `1` and `1000`.
        #[builder(into)]
        pub order: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Specifies what triggers this automation rule. Possible values are `Alerts` and `Incidents`. Defaults to `Incidents`.
        #[builder(into, default)]
        pub triggers_on: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies when will this automation rule be triggered. Possible values are `Created` and `Updated`. Defaults to `Created`.
        #[builder(into, default)]
        pub triggers_when: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AuthomationRuleResult {
        /// One or more `action_incident` blocks as defined below.
        pub action_incidents: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::sentinel::AuthomationRuleActionIncident>>,
        >,
        /// One or more `action_playbook` blocks as defined below.
        ///
        /// > **Note:** Either one `action_incident` block or `action_playbook` block has to be specified.
        pub action_playbooks: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::sentinel::AuthomationRuleActionPlaybook>>,
        >,
        /// A JSON array of one or more condition JSON objects as is defined [here](https://learn.microsoft.com/en-us/rest/api/securityinsights/preview/automation-rules/create-or-update?tabs=HTTP#automationruletriggeringlogic).
        pub condition_json: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name which should be used for this Sentinel Automation Rule.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Whether this Sentinel Automation Rule is enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The time in RFC3339 format of kind `UTC` that determines when this Automation Rule should expire and be disabled.
        pub expiration: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Log Analytics Workspace where this Sentinel applies to. Changing this forces a new Sentinel Automation Rule to be created.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The UUID which should be used for this Sentinel Automation Rule. Changing this forces a new Sentinel Automation Rule to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The order of this Sentinel Automation Rule. Possible values varies between `1` and `1000`.
        pub order: pulumi_gestalt_rust::Output<i32>,
        /// Specifies what triggers this automation rule. Possible values are `Alerts` and `Incidents`. Defaults to `Incidents`.
        pub triggers_on: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies when will this automation rule be triggered. Possible values are `Created` and `Updated`. Defaults to `Created`.
        pub triggers_when: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthomationRuleArgs,
    ) -> AuthomationRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_incidents_binding = args.action_incidents.get_output(context);
        let action_playbooks_binding = args.action_playbooks.get_output(context);
        let condition_json_binding = args.condition_json.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let expiration_binding = args.expiration.get_output(context);
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let order_binding = args.order.get_output(context);
        let triggers_on_binding = args.triggers_on.get_output(context);
        let triggers_when_binding = args.triggers_when.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:sentinel/authomationRule:AuthomationRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actionIncidents".into(),
                    value: action_incidents_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actionPlaybooks".into(),
                    value: action_playbooks_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "conditionJson".into(),
                    value: condition_json_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expiration".into(),
                    value: expiration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: log_analytics_workspace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "order".into(),
                    value: order_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggersOn".into(),
                    value: triggers_on_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggersWhen".into(),
                    value: triggers_when_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AuthomationRuleResult {
            action_incidents: o.get_field("actionIncidents"),
            action_playbooks: o.get_field("actionPlaybooks"),
            condition_json: o.get_field("conditionJson"),
            display_name: o.get_field("displayName"),
            enabled: o.get_field("enabled"),
            expiration: o.get_field("expiration"),
            log_analytics_workspace_id: o.get_field("logAnalyticsWorkspaceId"),
            name: o.get_field("name"),
            order: o.get_field("order"),
            triggers_on: o.get_field("triggersOn"),
            triggers_when: o.get_field("triggersWhen"),
        }
    }
}
