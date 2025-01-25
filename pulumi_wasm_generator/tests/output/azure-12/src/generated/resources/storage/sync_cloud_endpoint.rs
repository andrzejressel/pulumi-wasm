/// Manages a Storage Sync Cloud Endpoint.
///
/// > **NOTE:** Please ensure Azure File Sync has access to the storage account in your subscription, which indicates that `Microsoft.StorageSync` is assigned role `Reader and Data Access` ( refer to details [here](https://docs.microsoft.com/azure/storage/files/storage-sync-files-troubleshoot?tabs=portal1%2Cazure-portal#common-troubleshooting-steps)).
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
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleShare = share::create(
///         "exampleShare",
///         ShareArgs::builder()
///             .acls(
///                 vec![
///                     ShareAcl::builder()
///                     .accessPolicies(vec![ShareAclAccessPolicy::builder().permissions("r")
///                     .build_struct(),]).id("GhostedRecall").build_struct(),
///                 ],
///             )
///             .name("example-share")
///             .quota(50)
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleSync = sync::create(
///         "exampleSync",
///         SyncArgs::builder()
///             .location("${example.location}")
///             .name("example-ss")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSyncCloudEndpoint = sync_cloud_endpoint::create(
///         "exampleSyncCloudEndpoint",
///         SyncCloudEndpointArgs::builder()
///             .file_share_name("${exampleShare.name}")
///             .name("example-ss-ce")
///             .storage_account_id("${exampleAccount.id}")
///             .storage_sync_group_id("${exampleSyncGroup.id}")
///             .build_struct(),
///     );
///     let exampleSyncGroup = sync_group::create(
///         "exampleSyncGroup",
///         SyncGroupArgs::builder()
///             .name("example-ss-group")
///             .storage_sync_id("${exampleSync.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Sync Cloud Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/syncCloudEndpoint:SyncCloudEndpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StorageSync/storageSyncServices/sync1/syncGroups/syncgroup1/cloudEndpoints/cloudEndpoint1
/// ```
///
pub mod sync_cloud_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SyncCloudEndpointArgs {
        /// The Storage Share name to be synchronized in this Storage Sync Cloud Endpoint. Changing this forces a new Storage Sync Cloud Endpoint to be created.
        #[builder(into)]
        pub file_share_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this Storage Sync Cloud Endpoint. Changing this forces a new Storage Sync Cloud Endpoint to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Storage Account where the Storage Share exists. Changing this forces a new Storage Sync Cloud Endpoint to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Tenant ID of the Storage Account where the Storage Share exists. Changing this forces a new Storage Sync Cloud Endpoint to be created. Defaults to the current tenant id.
        #[builder(into, default)]
        pub storage_account_tenant_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Storage Sync Group where this Cloud Endpoint should be created. Changing this forces a new Storage Sync Cloud Endpoint to be created.
        #[builder(into)]
        pub storage_sync_group_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SyncCloudEndpointResult {
        /// The Storage Share name to be synchronized in this Storage Sync Cloud Endpoint. Changing this forces a new Storage Sync Cloud Endpoint to be created.
        pub file_share_name: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Storage Sync Cloud Endpoint. Changing this forces a new Storage Sync Cloud Endpoint to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Storage Account where the Storage Share exists. Changing this forces a new Storage Sync Cloud Endpoint to be created.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The Tenant ID of the Storage Account where the Storage Share exists. Changing this forces a new Storage Sync Cloud Endpoint to be created. Defaults to the current tenant id.
        pub storage_account_tenant_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Storage Sync Group where this Cloud Endpoint should be created. Changing this forces a new Storage Sync Cloud Endpoint to be created.
        pub storage_sync_group_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SyncCloudEndpointArgs,
    ) -> SyncCloudEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let file_share_name_binding = args
            .file_share_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let storage_account_id_binding = args
            .storage_account_id
            .get_output(context)
            .get_inner();
        let storage_account_tenant_id_binding = args
            .storage_account_tenant_id
            .get_output(context)
            .get_inner();
        let storage_sync_group_id_binding = args
            .storage_sync_group_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/syncCloudEndpoint:SyncCloudEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "fileShareName".into(),
                    value: &file_share_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountTenantId".into(),
                    value: &storage_account_tenant_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageSyncGroupId".into(),
                    value: &storage_sync_group_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "fileShareName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountTenantId".into(),
                },
                register_interface::ResultField {
                    name: "storageSyncGroupId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SyncCloudEndpointResult {
            file_share_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileShareName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
            storage_account_tenant_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountTenantId").unwrap(),
            ),
            storage_sync_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageSyncGroupId").unwrap(),
            ),
        }
    }
}
