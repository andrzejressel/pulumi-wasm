/// Manages a Log Analytics Storage Insights resource.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod storage_insights {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StorageInsightsArgs {
        /// The names of the blob containers that the workspace should read.
        #[builder(into, default)]
        pub blob_container_names: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The name which should be used for this Log Analytics Storage Insights. Changing this forces a new Log Analytics Storage Insights to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Log Analytics Storage Insights should exist. Changing this forces a new Log Analytics Storage Insights to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Storage Account used by this Log Analytics Storage Insights.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The storage access key to be used to connect to the storage account.
        #[builder(into)]
        pub storage_account_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The names of the Azure tables that the workspace should read.
        #[builder(into, default)]
        pub table_names: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the Log Analytics Workspace within which the Storage Insights should exist. Changing this forces a new Log Analytics Storage Insights to be created.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct StorageInsightsResult {
        /// The names of the blob containers that the workspace should read.
        pub blob_container_names: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name which should be used for this Log Analytics Storage Insights. Changing this forces a new Log Analytics Storage Insights to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Log Analytics Storage Insights should exist. Changing this forces a new Log Analytics Storage Insights to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Storage Account used by this Log Analytics Storage Insights.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// The storage access key to be used to connect to the storage account.
        pub storage_account_key: pulumi_gestalt_rust::Output<String>,
        /// The names of the Azure tables that the workspace should read.
        pub table_names: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of the Log Analytics Workspace within which the Storage Insights should exist. Changing this forces a new Log Analytics Storage Insights to be created.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StorageInsightsArgs,
    ) -> StorageInsightsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let blob_container_names_binding = args.blob_container_names.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let storage_account_key_binding = args.storage_account_key.get_output(context);
        let table_names_binding = args.table_names.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:loganalytics/storageInsights:StorageInsights".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blobContainerNames".into(),
                    value: blob_container_names_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: storage_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountKey".into(),
                    value: storage_account_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableNames".into(),
                    value: table_names_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        StorageInsightsResult {
            blob_container_names: o.get_field("blobContainerNames"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            storage_account_id: o.get_field("storageAccountId"),
            storage_account_key: o.get_field("storageAccountKey"),
            table_names: o.get_field("tableNames"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
