/// Manages a Log Analytics Linked Storage Account.
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
///             .account_replication_type("GRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("examplesa")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("exampleworkspace")
///             .resource_group_name("${example.name}")
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleLinkedStorageAccount = linked_storage_account::create(
///         "exampleLinkedStorageAccount",
///         LinkedStorageAccountArgs::builder()
///             .data_source_type("CustomLogs")
///             .resource_group_name("${example.name}")
///             .storage_account_ids(vec!["${exampleAccount.id}",])
///             .workspace_resource_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Log Analytics Linked Storage Accounts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:loganalytics/linkedStorageAccount:LinkedStorageAccount example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/linkedStorageAccounts/{dataSourceType}
/// ```
///
pub mod linked_storage_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedStorageAccountArgs {
        /// The data source type which should be used for this Log Analytics Linked Storage Account. Possible values are `CustomLogs`, `AzureWatson`, `Query`, `Ingestion` and `Alerts`. Changing this forces a new Log Analytics Linked Storage Account to be created.
        #[builder(into)]
        pub data_source_type: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Log Analytics Linked Storage Account should exist. Changing this forces a new Log Analytics Linked Storage Account to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The storage account resource ids to be linked.
        #[builder(into)]
        pub storage_account_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The resource ID of the Log Analytics Workspace. Changing this forces a new Log Analytics Linked Storage Account to be created.
        #[builder(into)]
        pub workspace_resource_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LinkedStorageAccountResult {
        /// The data source type which should be used for this Log Analytics Linked Storage Account. Possible values are `CustomLogs`, `AzureWatson`, `Query`, `Ingestion` and `Alerts`. Changing this forces a new Log Analytics Linked Storage Account to be created.
        pub data_source_type: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Log Analytics Linked Storage Account should exist. Changing this forces a new Log Analytics Linked Storage Account to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The storage account resource ids to be linked.
        pub storage_account_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The resource ID of the Log Analytics Workspace. Changing this forces a new Log Analytics Linked Storage Account to be created.
        pub workspace_resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: LinkedStorageAccountArgs,
    ) -> LinkedStorageAccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_source_type_binding = args.data_source_type.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let storage_account_ids_binding = args.storage_account_ids.get_inner();
        let workspace_resource_id_binding = args.workspace_resource_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:loganalytics/linkedStorageAccount:LinkedStorageAccount".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataSourceType".into(),
                    value: &data_source_type_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountIds".into(),
                    value: &storage_account_ids_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceResourceId".into(),
                    value: &workspace_resource_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dataSourceType".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountIds".into(),
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
        LinkedStorageAccountResult {
            data_source_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSourceType").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            storage_account_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountIds").unwrap(),
            ),
            workspace_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceResourceId").unwrap(),
            ),
        }
    }
}