/// Manages a Sentinel Fusion Alert Rule.
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
///     let exampleAlertRuleFusion = alert_rule_fusion::create(
///         "exampleAlertRuleFusion",
///         AlertRuleFusionArgs::builder()
///             .alert_rule_template_guid("f71aba3d-28fb-450b-b192-4e76a83015c8")
///             .log_analytics_workspace_id(
///                 "${exampleLogAnalyticsWorkspaceOnboarding.workspaceId}",
///             )
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
/// Sentinel Fusion Alert Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/alertRuleFusion:AlertRuleFusion example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/alertRules/rule1
/// ```
///
pub mod alert_rule_fusion {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertRuleFusionArgs {
        /// The GUID of the alert rule template which is used for this Sentinel Fusion Alert Rule. Changing this forces a new Sentinel Fusion Alert Rule to be created.
        #[builder(into)]
        pub alert_rule_template_guid: pulumi_wasm_rust::Output<String>,
        /// Should this Sentinel Fusion Alert Rule be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Log Analytics Workspace this Sentinel Fusion Alert Rule belongs to. Changing this forces a new Sentinel Fusion Alert Rule to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `source` blocks as defined below.
        #[builder(into, default)]
        pub sources: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::sentinel::AlertRuleFusionSource>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AlertRuleFusionResult {
        /// The GUID of the alert rule template which is used for this Sentinel Fusion Alert Rule. Changing this forces a new Sentinel Fusion Alert Rule to be created.
        pub alert_rule_template_guid: pulumi_wasm_rust::Output<String>,
        /// Should this Sentinel Fusion Alert Rule be enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Log Analytics Workspace this Sentinel Fusion Alert Rule belongs to. Changing this forces a new Sentinel Fusion Alert Rule to be created.
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `source` blocks as defined below.
        pub sources: pulumi_wasm_rust::Output<
            Vec<super::super::types::sentinel::AlertRuleFusionSource>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AlertRuleFusionArgs) -> AlertRuleFusionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alert_rule_template_guid_binding = args.alert_rule_template_guid.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let sources_binding = args.sources.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/alertRuleFusion:AlertRuleFusion".into(),
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
                register_interface::ObjectField {
                    name: "sources".into(),
                    value: &sources_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alertRuleTemplateGuid".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "logAnalyticsWorkspaceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "sources".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AlertRuleFusionResult {
            alert_rule_template_guid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alertRuleTemplateGuid").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            log_analytics_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logAnalyticsWorkspaceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sources").unwrap(),
            ),
        }
    }
}
