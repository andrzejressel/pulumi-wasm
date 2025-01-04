/// Manages a Sentinel Threat Intelligence Alert Rule.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAnalyticsWorkspace:
///     type: azure:operationalinsights:AnalyticsWorkspace
///     name: example
///     properties:
///       name: example-workspace
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       sku: pergb2018
///   exampleAnalyticsSolution:
///     type: azure:operationalinsights:AnalyticsSolution
///     name: example
///     properties:
///       solutionName: SecurityInsights
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       workspaceResourceId: ${exampleAnalyticsWorkspace.id}
///       workspaceName: ${exampleAnalyticsWorkspace.name}
///       plan:
///         publisher: Microsoft
///         product: OMSGallery/SecurityInsights
///   exampleAlertRuleThreatIntelligence:
///     type: azure:sentinel:AlertRuleThreatIntelligence
///     name: example
///     properties:
///       name: example-rule
///       logAnalyticsWorkspaceId: ${exampleAnalyticsSolution.workspaceResourceId}
///       alertRuleTemplateGuid: ${example.name}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:sentinel:getAlertRuleTemplate
///       arguments:
///         displayName: (Preview) Microsoft Defender Threat Intelligence Analytics
///         logAnalyticsWorkspaceId: ${exampleAnalyticsSolution.workspaceResourceId}
/// ```
///
/// ## Import
///
/// Sentinel Threat Intelligence Alert Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/alertRuleThreatIntelligence:AlertRuleThreatIntelligence example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/alertRules/rule1
/// ```
///
pub mod alert_rule_threat_intelligence {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertRuleThreatIntelligenceArgs {
        /// The GUID of the alert rule template which is used for this Sentinel Threat Intelligence Alert Rule. Changing this forces a new Sentinel Threat Intelligence Alert Rule to be created.
        #[builder(into)]
        pub alert_rule_template_guid: pulumi_wasm_rust::Output<String>,
        /// Whether the Threat Intelligence Alert rule enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Log Analytics Workspace this Sentinel Threat Intelligence Alert Rule belongs to. Changing this forces a new Sentinel Threat Intelligence Alert Rule to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Sentinel Threat Intelligence Alert Rule. Changing this forces a new Sentinel Threat Intelligence Alert Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AlertRuleThreatIntelligenceResult {
        /// The GUID of the alert rule template which is used for this Sentinel Threat Intelligence Alert Rule. Changing this forces a new Sentinel Threat Intelligence Alert Rule to be created.
        pub alert_rule_template_guid: pulumi_wasm_rust::Output<String>,
        /// Whether the Threat Intelligence Alert rule enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Log Analytics Workspace this Sentinel Threat Intelligence Alert Rule belongs to. Changing this forces a new Sentinel Threat Intelligence Alert Rule to be created.
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Sentinel Threat Intelligence Alert Rule. Changing this forces a new Sentinel Threat Intelligence Alert Rule to be created.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AlertRuleThreatIntelligenceArgs,
    ) -> AlertRuleThreatIntelligenceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alert_rule_template_guid_binding = args.alert_rule_template_guid.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/alertRuleThreatIntelligence:AlertRuleThreatIntelligence"
                .into(),
            name: name.to_string(),
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
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AlertRuleThreatIntelligenceResult {
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
        }
    }
}
