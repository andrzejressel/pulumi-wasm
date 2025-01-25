/// Manages a Office 365 Data Connector.
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
///     let exampleDataConnectorOffice365 = data_connector_office_365::create(
///         "exampleDataConnectorOffice365",
///         DataConnectorOffice365Args::builder()
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
/// Office 365 Data Connectors can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/dataConnectorOffice365:DataConnectorOffice365 example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/dataConnectors/dc1
/// ```
///
pub mod data_connector_office_365 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataConnectorOffice365Args {
        /// Should the Exchange data connector be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub exchange_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Log Analytics Workspace that this Office 365 Data Connector resides in. Changing this forces a new Office 365 Data Connector to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this Office 365 Data Connector. Changing this forces a new Office 365 Data Connector to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Should the SharePoint data connector be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub sharepoint_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Should the Microsoft Teams data connector be enabled? Defaults to `true`.
        ///
        /// > **NOTE:** At least one of `exchange_enabled`, `sharedpoint_enabled` and `teams_enabled` has to be specified.
        #[builder(into, default)]
        pub teams_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Tenant that this Office 365 Data Connector connects to. Changing this forces a new Office 365 Data Connector to be created.
        ///
        /// > **NOTE** Currently, only the same tenant as the running account is allowed. Cross-tenant scenario is not supported yet.
        #[builder(into, default)]
        pub tenant_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DataConnectorOffice365Result {
        /// Should the Exchange data connector be enabled? Defaults to `true`.
        pub exchange_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Log Analytics Workspace that this Office 365 Data Connector resides in. Changing this forces a new Office 365 Data Connector to be created.
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Office 365 Data Connector. Changing this forces a new Office 365 Data Connector to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Should the SharePoint data connector be enabled? Defaults to `true`.
        pub sharepoint_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the Microsoft Teams data connector be enabled? Defaults to `true`.
        ///
        /// > **NOTE:** At least one of `exchange_enabled`, `sharedpoint_enabled` and `teams_enabled` has to be specified.
        pub teams_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Tenant that this Office 365 Data Connector connects to. Changing this forces a new Office 365 Data Connector to be created.
        ///
        /// > **NOTE** Currently, only the same tenant as the running account is allowed. Cross-tenant scenario is not supported yet.
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DataConnectorOffice365Args,
    ) -> DataConnectorOffice365Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let exchange_enabled_binding = args
            .exchange_enabled
            .get_output(context)
            .get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let sharepoint_enabled_binding = args
            .sharepoint_enabled
            .get_output(context)
            .get_inner();
        let teams_enabled_binding = args.teams_enabled.get_output(context).get_inner();
        let tenant_id_binding = args.tenant_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/dataConnectorOffice365:DataConnectorOffice365".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "exchangeEnabled".into(),
                    value: &exchange_enabled_binding,
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
                    name: "sharepointEnabled".into(),
                    value: &sharepoint_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "teamsEnabled".into(),
                    value: &teams_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "exchangeEnabled".into(),
                },
                register_interface::ResultField {
                    name: "logAnalyticsWorkspaceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "sharepointEnabled".into(),
                },
                register_interface::ResultField {
                    name: "teamsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "tenantId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DataConnectorOffice365Result {
            exchange_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exchangeEnabled").unwrap(),
            ),
            log_analytics_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logAnalyticsWorkspaceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            sharepoint_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharepointEnabled").unwrap(),
            ),
            teams_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("teamsEnabled").unwrap(),
            ),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantId").unwrap(),
            ),
        }
    }
}
