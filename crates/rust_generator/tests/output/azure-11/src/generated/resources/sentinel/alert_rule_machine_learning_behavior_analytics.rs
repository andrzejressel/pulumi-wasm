/// Manages a Sentinel Machine Learning Behavior Analytics Alert Rule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAlertRuleMachineLearningBehaviorAnalytics = alert_rule_machine_learning_behavior_analytics::create(
///         "exampleAlertRuleMachineLearningBehaviorAnalytics",
///         AlertRuleMachineLearningBehaviorAnalyticsArgs::builder()
///             .alert_rule_template_guid("737a2ce1-70a3-4968-9e90-3e6aca836abf")
///             .log_analytics_workspace_id("${exampleAnalyticsWorkspace.id}")
///             .name("example-ml-alert-rule")
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
/// Sentinel Machine Learning Behavior Analytics Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/alertRuleMachineLearningBehaviorAnalytics:AlertRuleMachineLearningBehaviorAnalytics example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/alertRules/rule1
/// ```
///
pub mod alert_rule_machine_learning_behavior_analytics {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertRuleMachineLearningBehaviorAnalyticsArgs {
        /// The GUID of the alert rule template which is used for this Sentinel Machine Learning Behavior Analytics Alert Rule. Changing this forces a new Sentinel Machine Learning Behavior Analytics Alert Rule to be created.
        #[builder(into)]
        pub alert_rule_template_guid: pulumi_wasm_rust::InputOrOutput<String>,
        /// Should this Sentinel Machine Learning Behavior Analytics Alert Rule be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Log Analytics Workspace this SentinelMachine Learning Behavior Analytics Alert Rule belongs to. Changing this forces a new Sentinel Machine Learning Behavior Analytics Alert Rule to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this SentinelMachine Learning Behavior Analytics Alert Rule. Changing this forces a new Sentinel Machine Learning Behavior Analytics Alert Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AlertRuleMachineLearningBehaviorAnalyticsResult {
        /// The GUID of the alert rule template which is used for this Sentinel Machine Learning Behavior Analytics Alert Rule. Changing this forces a new Sentinel Machine Learning Behavior Analytics Alert Rule to be created.
        pub alert_rule_template_guid: pulumi_wasm_rust::Output<String>,
        /// Should this Sentinel Machine Learning Behavior Analytics Alert Rule be enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Log Analytics Workspace this SentinelMachine Learning Behavior Analytics Alert Rule belongs to. Changing this forces a new Sentinel Machine Learning Behavior Analytics Alert Rule to be created.
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this SentinelMachine Learning Behavior Analytics Alert Rule. Changing this forces a new Sentinel Machine Learning Behavior Analytics Alert Rule to be created.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AlertRuleMachineLearningBehaviorAnalyticsArgs,
    ) -> AlertRuleMachineLearningBehaviorAnalyticsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alert_rule_template_guid_binding = args
            .alert_rule_template_guid
            .get_output(context)
            .get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/alertRuleMachineLearningBehaviorAnalytics:AlertRuleMachineLearningBehaviorAnalytics"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alertRuleTemplateGuid".into(),
                    value: &alert_rule_template_guid_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AlertRuleMachineLearningBehaviorAnalyticsResult {
            alert_rule_template_guid: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("alertRuleTemplateGuid"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            log_analytics_workspace_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logAnalyticsWorkspaceId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
