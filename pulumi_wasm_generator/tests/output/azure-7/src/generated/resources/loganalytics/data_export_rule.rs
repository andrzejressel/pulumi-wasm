/// Manages a Log Analytics Data Export Rule.
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
pub mod data_export_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataExportRuleArgs {
        /// The destination resource ID. It should be a storage account, an event hub namespace or an event hub. If the destination is an event hub namespace, an event hub would be created for each table automatically.
        #[builder(into)]
        pub destination_resource_id: pulumi_wasm_rust::Output<String>,
        /// Is this Log Analytics Data Export Rule enabled? Possible values include `true` or `false`. Defaults to `false`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Log Analytics Data Export Rule. Changing this forces a new Log Analytics Data Export Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Log Analytics Data Export should exist. Changing this forces a new Log Analytics Data Export Rule to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A list of table names to export to the destination resource, for example: `["Heartbeat", "SecurityEvent"]`.
        #[builder(into)]
        pub table_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// The resource ID of the workspace. Changing this forces a new Log Analytics Data Export Rule to be created.
        #[builder(into)]
        pub workspace_resource_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DataExportRuleResult {
        /// The destination resource ID. It should be a storage account, an event hub namespace or an event hub. If the destination is an event hub namespace, an event hub would be created for each table automatically.
        pub destination_resource_id: pulumi_wasm_rust::Output<String>,
        /// Is this Log Analytics Data Export Rule enabled? Possible values include `true` or `false`. Defaults to `false`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the created Data Export Rule.
        pub export_rule_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Log Analytics Data Export Rule. Changing this forces a new Log Analytics Data Export Rule to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Log Analytics Data Export should exist. Changing this forces a new Log Analytics Data Export Rule to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A list of table names to export to the destination resource, for example: `["Heartbeat", "SecurityEvent"]`.
        pub table_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// The resource ID of the workspace. Changing this forces a new Log Analytics Data Export Rule to be created.
        pub workspace_resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DataExportRuleArgs) -> DataExportRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_resource_id_binding = args.destination_resource_id.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let table_names_binding = args.table_names.get_inner();
        let workspace_resource_id_binding = args.workspace_resource_id.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "destinationResourceId".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "exportRuleId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tableNames".into(),
                },
                register_interface::ResultField {
                    name: "workspaceResourceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DataExportRuleResult {
            destination_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationResourceId").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            export_rule_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportRuleId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            table_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableNames").unwrap(),
            ),
            workspace_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceResourceId").unwrap(),
            ),
        }
    }
}
