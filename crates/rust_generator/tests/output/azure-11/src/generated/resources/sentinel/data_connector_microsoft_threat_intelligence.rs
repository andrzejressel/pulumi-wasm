/// Manages a Microsoft Threat Intelligence Data Connector.
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
///             .location("east us")
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
///     let exampleDataConnectorMicrosoftThreatIntelligence = data_connector_microsoft_threat_intelligence::create(
///         "exampleDataConnectorMicrosoftThreatIntelligence",
///         DataConnectorMicrosoftThreatIntelligenceArgs::builder()
///             .log_analytics_workspace_id(
///                 "${exampleLogAnalyticsWorkspaceOnboarding.workspaceId}",
///             )
///             .microsoft_emerging_threat_feed_lookback_date("1970-01-01T00:00:00Z")
///             .name("example-dc-msti")
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
/// sentinels can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/dataConnectorMicrosoftThreatIntelligence:DataConnectorMicrosoftThreatIntelligence example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/dataConnectors/dc1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_connector_microsoft_threat_intelligence {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataConnectorMicrosoftThreatIntelligenceArgs {
        /// The ID of the Log Analytics Workspace. Changing this forces a new Data Connector to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The lookback date for the Microsoft Emerging Threat Feed in RFC3339. Changing this forces a new Data Connector to be created.
        #[builder(into)]
        pub microsoft_emerging_threat_feed_lookback_date: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// The name which should be used for this Microsoft Threat Intelligence Data Connector. Changing this forces a new Microsoft Threat Intelligence Data Connector to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the tenant that this Microsoft Threat Intelligence Data Connector connects to. Changing this forces a new Microsoft Threat Intelligence Data Connector to be created.
        ///
        /// > **NOTE** Currently, only the same tenant as the running account is allowed. Cross-tenant scenario is not supported yet.
        #[builder(into, default)]
        pub tenant_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DataConnectorMicrosoftThreatIntelligenceResult {
        /// The ID of the Log Analytics Workspace. Changing this forces a new Data Connector to be created.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The lookback date for the Microsoft Emerging Threat Feed in RFC3339. Changing this forces a new Data Connector to be created.
        pub microsoft_emerging_threat_feed_lookback_date: pulumi_gestalt_rust::Output<
            String,
        >,
        /// The name which should be used for this Microsoft Threat Intelligence Data Connector. Changing this forces a new Microsoft Threat Intelligence Data Connector to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the tenant that this Microsoft Threat Intelligence Data Connector connects to. Changing this forces a new Microsoft Threat Intelligence Data Connector to be created.
        ///
        /// > **NOTE** Currently, only the same tenant as the running account is allowed. Cross-tenant scenario is not supported yet.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataConnectorMicrosoftThreatIntelligenceArgs,
    ) -> DataConnectorMicrosoftThreatIntelligenceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context);
        let microsoft_emerging_threat_feed_lookback_date_binding = args
            .microsoft_emerging_threat_feed_lookback_date
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let tenant_id_binding = args.tenant_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:sentinel/dataConnectorMicrosoftThreatIntelligence:DataConnectorMicrosoftThreatIntelligence"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: log_analytics_workspace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "microsoftEmergingThreatFeedLookbackDate".into(),
                    value: microsoft_emerging_threat_feed_lookback_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenantId".into(),
                    value: tenant_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataConnectorMicrosoftThreatIntelligenceResult {
            log_analytics_workspace_id: o.get_field("logAnalyticsWorkspaceId"),
            microsoft_emerging_threat_feed_lookback_date: o
                .get_field("microsoftEmergingThreatFeedLookbackDate"),
            name: o.get_field("name"),
            tenant_id: o.get_field("tenantId"),
        }
    }
}
