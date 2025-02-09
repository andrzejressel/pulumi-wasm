/// Manages a Log Analytics Linked Storage Account.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod linked_storage_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedStorageAccountArgs {
        /// The data source type which should be used for this Log Analytics Linked Storage Account. Possible values are `CustomLogs`, `AzureWatson`, `Query`, `Ingestion` and `Alerts`. Changing this forces a new Log Analytics Linked Storage Account to be created.
        #[builder(into)]
        pub data_source_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Log Analytics Linked Storage Account should exist. Changing this forces a new Log Analytics Linked Storage Account to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The storage account resource ids to be linked.
        #[builder(into)]
        pub storage_account_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The resource ID of the Log Analytics Workspace. Changing this forces a new Log Analytics Linked Storage Account to be created.
        #[builder(into)]
        pub workspace_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LinkedStorageAccountResult {
        /// The data source type which should be used for this Log Analytics Linked Storage Account. Possible values are `CustomLogs`, `AzureWatson`, `Query`, `Ingestion` and `Alerts`. Changing this forces a new Log Analytics Linked Storage Account to be created.
        pub data_source_type: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Log Analytics Linked Storage Account should exist. Changing this forces a new Log Analytics Linked Storage Account to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The storage account resource ids to be linked.
        pub storage_account_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The resource ID of the Log Analytics Workspace. Changing this forces a new Log Analytics Linked Storage Account to be created.
        pub workspace_resource_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LinkedStorageAccountArgs,
    ) -> LinkedStorageAccountResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_source_type_binding = args.data_source_type.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let storage_account_ids_binding = args.storage_account_ids.get_output(context);
        let workspace_resource_id_binding = args
            .workspace_resource_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:loganalytics/linkedStorageAccount:LinkedStorageAccount".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataSourceType".into(),
                    value: data_source_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountIds".into(),
                    value: storage_account_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceResourceId".into(),
                    value: workspace_resource_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LinkedStorageAccountResult {
            data_source_type: o.get_field("dataSourceType"),
            resource_group_name: o.get_field("resourceGroupName"),
            storage_account_ids: o.get_field("storageAccountIds"),
            workspace_resource_id: o.get_field("workspaceResourceId"),
        }
    }
}
