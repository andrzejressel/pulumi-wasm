/// Manages a Microsoft Cloud App Security Data Connector.
///
///  !> **NOTE:** This resource requires that [Enterprise Mobility + Security E5](https://www.microsoft.com/en-us/microsoft-365/enterprise-mobility-security) is enabled on the tenant being connected to.
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
///     let exampleDataConnectorMicrosoftCloudAppSecurity = data_connector_microsoft_cloud_app_security::create(
///         "exampleDataConnectorMicrosoftCloudAppSecurity",
///         DataConnectorMicrosoftCloudAppSecurityArgs::builder()
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
/// Microsoft Cloud App Security Data Connectors can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/dataConnectorMicrosoftCloudAppSecurity:DataConnectorMicrosoftCloudAppSecurity example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/dataConnectors/dc1
/// ```
///
pub mod data_connector_microsoft_cloud_app_security {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataConnectorMicrosoftCloudAppSecurityArgs {
        /// Should the alerts be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub alerts_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Should the Discovery Logs be enabled? Defaults to `true`.
        ///
        /// > **NOTE:** One of either `alerts_enabled` or `discovery_logs_enabled` has to be specified.
        #[builder(into, default)]
        pub discovery_logs_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Log Analytics Workspace that this Microsoft Cloud App Security Data Connector resides in. Changing this forces a new Microsoft Cloud App Security Data Connector to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this Microsoft Cloud App Security Data Connector. Changing this forces a new Microsoft Cloud App Security Data Connector to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Tenant that this Microsoft Cloud App Security Data Connector connects to.
        ///
        /// > **NOTE** Currently, only the same tenant as the running account is allowed. Cross-tenant scenario is not supported yet.
        #[builder(into, default)]
        pub tenant_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DataConnectorMicrosoftCloudAppSecurityResult {
        /// Should the alerts be enabled? Defaults to `true`.
        pub alerts_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the Discovery Logs be enabled? Defaults to `true`.
        ///
        /// > **NOTE:** One of either `alerts_enabled` or `discovery_logs_enabled` has to be specified.
        pub discovery_logs_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Log Analytics Workspace that this Microsoft Cloud App Security Data Connector resides in. Changing this forces a new Microsoft Cloud App Security Data Connector to be created.
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Microsoft Cloud App Security Data Connector. Changing this forces a new Microsoft Cloud App Security Data Connector to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Tenant that this Microsoft Cloud App Security Data Connector connects to.
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
        args: DataConnectorMicrosoftCloudAppSecurityArgs,
    ) -> DataConnectorMicrosoftCloudAppSecurityResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alerts_enabled_binding = args.alerts_enabled.get_output(context).get_inner();
        let discovery_logs_enabled_binding = args
            .discovery_logs_enabled
            .get_output(context)
            .get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tenant_id_binding = args.tenant_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/dataConnectorMicrosoftCloudAppSecurity:DataConnectorMicrosoftCloudAppSecurity"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alertsEnabled".into(),
                    value: &alerts_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "discoveryLogsEnabled".into(),
                    value: &discovery_logs_enabled_binding,
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
                    name: "tenantId".into(),
                    value: &tenant_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DataConnectorMicrosoftCloudAppSecurityResult {
            alerts_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("alertsEnabled"),
            ),
            discovery_logs_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("discoveryLogsEnabled"),
            ),
            log_analytics_workspace_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logAnalyticsWorkspaceId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tenantId"),
            ),
        }
    }
}
