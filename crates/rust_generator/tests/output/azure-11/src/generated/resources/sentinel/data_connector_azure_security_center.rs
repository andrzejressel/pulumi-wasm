/// Manages a Azure Security Center Data Connector.
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
///     let exampleDataConnectorAzureSecurityCenter = data_connector_azure_security_center::create(
///         "exampleDataConnectorAzureSecurityCenter",
///         DataConnectorAzureSecurityCenterArgs::builder()
///             .log_analytics_workspace_id(
///                 "${exampleLogAnalyticsWorkspaceOnboarding.workspaceId}",
///             )
///             .name("example")
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
/// Azure Security Center Data Connectors can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/dataConnectorAzureSecurityCenter:DataConnectorAzureSecurityCenter example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/dataConnectors/dc1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_connector_azure_security_center {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataConnectorAzureSecurityCenterArgs {
        /// The ID of the Log Analytics Workspace that this Azure Security Center Data Connector resides in. Changing this forces a new Azure Security Center Data Connector to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Azure Security Center Data Connector. Changing this forces a new Azure Security Center Data Connector to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the subscription that this Azure Security Center Data Connector connects to. Changing this forces a new Azure Security Center Data Connector to be created.
        #[builder(into, default)]
        pub subscription_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DataConnectorAzureSecurityCenterResult {
        /// The ID of the Log Analytics Workspace that this Azure Security Center Data Connector resides in. Changing this forces a new Azure Security Center Data Connector to be created.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Azure Security Center Data Connector. Changing this forces a new Azure Security Center Data Connector to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the subscription that this Azure Security Center Data Connector connects to. Changing this forces a new Azure Security Center Data Connector to be created.
        pub subscription_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataConnectorAzureSecurityCenterArgs,
    ) -> DataConnectorAzureSecurityCenterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let subscription_id_binding = args.subscription_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:sentinel/dataConnectorAzureSecurityCenter:DataConnectorAzureSecurityCenter"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: log_analytics_workspace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriptionId".into(),
                    value: subscription_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataConnectorAzureSecurityCenterResult {
            log_analytics_workspace_id: o.get_field("logAnalyticsWorkspaceId"),
            name: o.get_field("name"),
            subscription_id: o.get_field("subscriptionId"),
        }
    }
}
