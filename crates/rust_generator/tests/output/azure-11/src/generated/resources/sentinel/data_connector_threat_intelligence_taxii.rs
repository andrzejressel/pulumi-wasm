/// Manages an Threat Intelligence TAXII Data Connector.
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
///             .location("West Europe")
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
///     let exampleDataConnectorThreatIntelligenceTaxii = data_connector_threat_intelligence_taxii::create(
///         "exampleDataConnectorThreatIntelligenceTaxii",
///         DataConnectorThreatIntelligenceTaxiiArgs::builder()
///             .api_root_url("https://foo/taxii2/api2/")
///             .collection_id("someid")
///             .display_name("example")
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
/// Threat Intelligence TAXII Data Connectors can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/dataConnectorThreatIntelligenceTaxii:DataConnectorThreatIntelligenceTaxii example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/dataConnectors/dc1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_connector_threat_intelligence_taxii {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataConnectorThreatIntelligenceTaxiiArgs {
        /// The API root URI of the TAXII server.
        #[builder(into)]
        pub api_root_url: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The collection ID of the TAXII server.
        #[builder(into)]
        pub collection_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The friendly name which should be used for this Threat Intelligence TAXII Data Connector.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Log Analytics Workspace that this Threat Intelligence TAXII Data Connector resides in. Changing this forces a new Threat Intelligence TAXII Data Connector to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The lookback date for the TAXII server in RFC3339. Defaults to `1970-01-01T00:00:00Z`.
        #[builder(into, default)]
        pub lookback_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Threat Intelligence TAXII Data Connector. Changing this forces a new Threat Intelligence TAXII Data Connector to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The password for the TAXII server.
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The polling frequency for the TAXII server. Possible values are `OnceAMinute`, `OnceAnHour` and `OnceADay`. Defaults to `OnceAnHour`.
        #[builder(into, default)]
        pub polling_frequency: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the tenant that this Threat Intelligence TAXII Data Connector connects to. Changing this forces a new Threat Intelligence TAXII Data Connector to be created.
        ///
        /// > **NOTE** Currently, only the same tenant as the running account is allowed. Cross-tenant scenario is not supported yet.
        #[builder(into, default)]
        pub tenant_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user name for the TAXII server.
        #[builder(into, default)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DataConnectorThreatIntelligenceTaxiiResult {
        /// The API root URI of the TAXII server.
        pub api_root_url: pulumi_gestalt_rust::Output<String>,
        /// The collection ID of the TAXII server.
        pub collection_id: pulumi_gestalt_rust::Output<String>,
        /// The friendly name which should be used for this Threat Intelligence TAXII Data Connector.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Log Analytics Workspace that this Threat Intelligence TAXII Data Connector resides in. Changing this forces a new Threat Intelligence TAXII Data Connector to be created.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The lookback date for the TAXII server in RFC3339. Defaults to `1970-01-01T00:00:00Z`.
        pub lookback_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Threat Intelligence TAXII Data Connector. Changing this forces a new Threat Intelligence TAXII Data Connector to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The password for the TAXII server.
        pub password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The polling frequency for the TAXII server. Possible values are `OnceAMinute`, `OnceAnHour` and `OnceADay`. Defaults to `OnceAnHour`.
        pub polling_frequency: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the tenant that this Threat Intelligence TAXII Data Connector connects to. Changing this forces a new Threat Intelligence TAXII Data Connector to be created.
        ///
        /// > **NOTE** Currently, only the same tenant as the running account is allowed. Cross-tenant scenario is not supported yet.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
        /// The user name for the TAXII server.
        pub user_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataConnectorThreatIntelligenceTaxiiArgs,
    ) -> DataConnectorThreatIntelligenceTaxiiResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_root_url_binding = args.api_root_url.get_output(context);
        let collection_id_binding = args.collection_id.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context);
        let lookback_date_binding = args.lookback_date.get_output(context);
        let name_binding = args.name.get_output(context);
        let password_binding = args.password.get_output(context);
        let polling_frequency_binding = args.polling_frequency.get_output(context);
        let tenant_id_binding = args.tenant_id.get_output(context);
        let user_name_binding = args.user_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:sentinel/dataConnectorThreatIntelligenceTaxii:DataConnectorThreatIntelligenceTaxii"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiRootUrl".into(),
                    value: api_root_url_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "collectionId".into(),
                    value: collection_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: log_analytics_workspace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lookbackDate".into(),
                    value: lookback_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pollingFrequency".into(),
                    value: polling_frequency_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenantId".into(),
                    value: tenant_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: user_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataConnectorThreatIntelligenceTaxiiResult {
            api_root_url: o.get_field("apiRootUrl"),
            collection_id: o.get_field("collectionId"),
            display_name: o.get_field("displayName"),
            log_analytics_workspace_id: o.get_field("logAnalyticsWorkspaceId"),
            lookback_date: o.get_field("lookbackDate"),
            name: o.get_field("name"),
            password: o.get_field("password"),
            polling_frequency: o.get_field("pollingFrequency"),
            tenant_id: o.get_field("tenantId"),
            user_name: o.get_field("userName"),
        }
    }
}
