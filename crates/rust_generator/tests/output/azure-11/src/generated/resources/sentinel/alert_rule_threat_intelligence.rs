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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod alert_rule_threat_intelligence {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertRuleThreatIntelligenceArgs {
        /// The GUID of the alert rule template which is used for this Sentinel Threat Intelligence Alert Rule. Changing this forces a new Sentinel Threat Intelligence Alert Rule to be created.
        #[builder(into)]
        pub alert_rule_template_guid: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the Threat Intelligence Alert rule enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Log Analytics Workspace this Sentinel Threat Intelligence Alert Rule belongs to. Changing this forces a new Sentinel Threat Intelligence Alert Rule to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Sentinel Threat Intelligence Alert Rule. Changing this forces a new Sentinel Threat Intelligence Alert Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AlertRuleThreatIntelligenceResult {
        /// The GUID of the alert rule template which is used for this Sentinel Threat Intelligence Alert Rule. Changing this forces a new Sentinel Threat Intelligence Alert Rule to be created.
        pub alert_rule_template_guid: pulumi_gestalt_rust::Output<String>,
        /// Whether the Threat Intelligence Alert rule enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Log Analytics Workspace this Sentinel Threat Intelligence Alert Rule belongs to. Changing this forces a new Sentinel Threat Intelligence Alert Rule to be created.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Sentinel Threat Intelligence Alert Rule. Changing this forces a new Sentinel Threat Intelligence Alert Rule to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AlertRuleThreatIntelligenceArgs,
    ) -> AlertRuleThreatIntelligenceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
            type_: "azure:sentinel/alertRuleThreatIntelligence:AlertRuleThreatIntelligence"
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
        AlertRuleThreatIntelligenceResult {
            alert_rule_template_guid: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("alertRuleTemplateGuid"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            log_analytics_workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logAnalyticsWorkspaceId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
