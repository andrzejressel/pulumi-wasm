/// Manages a Log Analytics Storage Insights resource.
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
///     let exampleStorageInsights = storage_insights::create(
///         "exampleStorageInsights",
///         StorageInsightsArgs::builder()
///             .name("example-storageinsightconfig")
///             .resource_group_name("${example.name}")
///             .storage_account_id("${exampleAccount.id}")
///             .storage_account_key("${exampleAccount.primaryAccessKey}")
///             .workspace_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Log Analytics Storage Insight Configs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:loganalytics/storageInsights:StorageInsights example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/storageInsightConfigs/storageInsight1
/// ```
///
pub mod storage_insights {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StorageInsightsArgs {
        /// The names of the blob containers that the workspace should read.
        #[builder(into, default)]
        pub blob_container_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name which should be used for this Log Analytics Storage Insights. Changing this forces a new Log Analytics Storage Insights to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Log Analytics Storage Insights should exist. Changing this forces a new Log Analytics Storage Insights to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Storage Account used by this Log Analytics Storage Insights.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The storage access key to be used to connect to the storage account.
        #[builder(into)]
        pub storage_account_key: pulumi_wasm_rust::Output<String>,
        /// The names of the Azure tables that the workspace should read.
        #[builder(into, default)]
        pub table_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the Log Analytics Workspace within which the Storage Insights should exist. Changing this forces a new Log Analytics Storage Insights to be created.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct StorageInsightsResult {
        /// The names of the blob containers that the workspace should read.
        pub blob_container_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name which should be used for this Log Analytics Storage Insights. Changing this forces a new Log Analytics Storage Insights to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Log Analytics Storage Insights should exist. Changing this forces a new Log Analytics Storage Insights to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Storage Account used by this Log Analytics Storage Insights.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The storage access key to be used to connect to the storage account.
        pub storage_account_key: pulumi_wasm_rust::Output<String>,
        /// The names of the Azure tables that the workspace should read.
        pub table_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the Log Analytics Workspace within which the Storage Insights should exist. Changing this forces a new Log Analytics Storage Insights to be created.
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: StorageInsightsArgs) -> StorageInsightsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let blob_container_names_binding = args.blob_container_names.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let storage_account_key_binding = args.storage_account_key.get_inner();
        let table_names_binding = args.table_names.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:loganalytics/storageInsights:StorageInsights".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "blobContainerNames".into(),
                    value: &blob_container_names_binding,
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
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountKey".into(),
                    value: &storage_account_key_binding,
                },
                register_interface::ObjectField {
                    name: "tableNames".into(),
                    value: &table_names_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "blobContainerNames".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountKey".into(),
                },
                register_interface::ResultField {
                    name: "tableNames".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StorageInsightsResult {
            blob_container_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blobContainerNames").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
            storage_account_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountKey").unwrap(),
            ),
            table_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableNames").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}