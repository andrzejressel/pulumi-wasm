/// Manages a Log Analytics Data Export Rule.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("examplestoracc")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("exampleworkspace")
///             .resource_group_name("${example.name}")
///             .retention_in_days(30)
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleDataExportRule = data_export_rule::create(
///         "exampleDataExportRule",
///         DataExportRuleArgs::builder()
///             .destination_resource_id("${exampleAccount.id}")
///             .enabled(true)
///             .name("dataExport1")
///             .resource_group_name("${example.name}")
///             .table_names(vec!["Heartbeat",])
///             .workspace_resource_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Log Analytics Data Export Rule can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:loganalytics/dataExportRule:DataExportRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/dataExports/dataExport1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_export_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataExportRuleArgs {
        /// The destination resource ID. It should be a storage account, an event hub namespace or an event hub. If the destination is an event hub namespace, an event hub would be created for each table automatically.
        #[builder(into)]
        pub destination_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is this Log Analytics Data Export Rule enabled? Possible values include `true` or `false`. Defaults to `false`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Log Analytics Data Export Rule. Changing this forces a new Log Analytics Data Export Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Log Analytics Data Export should exist. Changing this forces a new Log Analytics Data Export Rule to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of table names to export to the destination resource, for example: `["Heartbeat", "SecurityEvent"]`.
        #[builder(into)]
        pub table_names: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The resource ID of the workspace. Changing this forces a new Log Analytics Data Export Rule to be created.
        #[builder(into)]
        pub workspace_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DataExportRuleResult {
        /// The destination resource ID. It should be a storage account, an event hub namespace or an event hub. If the destination is an event hub namespace, an event hub would be created for each table automatically.
        pub destination_resource_id: pulumi_gestalt_rust::Output<String>,
        /// Is this Log Analytics Data Export Rule enabled? Possible values include `true` or `false`. Defaults to `false`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the created Data Export Rule.
        pub export_rule_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Log Analytics Data Export Rule. Changing this forces a new Log Analytics Data Export Rule to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Log Analytics Data Export should exist. Changing this forces a new Log Analytics Data Export Rule to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A list of table names to export to the destination resource, for example: `["Heartbeat", "SecurityEvent"]`.
        pub table_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The resource ID of the workspace. Changing this forces a new Log Analytics Data Export Rule to be created.
        pub workspace_resource_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DataExportRuleArgs,
    ) -> DataExportRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let destination_resource_id_binding_1 = args
            .destination_resource_id
            .get_output(context);
        let destination_resource_id_binding = destination_resource_id_binding_1
            .get_inner();
        let enabled_binding_1 = args.enabled.get_output(context);
        let enabled_binding = enabled_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let table_names_binding_1 = args.table_names.get_output(context);
        let table_names_binding = table_names_binding_1.get_inner();
        let workspace_resource_id_binding_1 = args
            .workspace_resource_id
            .get_output(context);
        let workspace_resource_id_binding = workspace_resource_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:loganalytics/dataExportRule:DataExportRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationResourceId".into(),
                    value: &destination_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tableNames".into(),
                    value: &table_names_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceResourceId".into(),
                    value: &workspace_resource_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DataExportRuleResult {
            destination_resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationResourceId"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            export_rule_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("exportRuleId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            table_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tableNames"),
            ),
            workspace_resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceResourceId"),
            ),
        }
    }
}
