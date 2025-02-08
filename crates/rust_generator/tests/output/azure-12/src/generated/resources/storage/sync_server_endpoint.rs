/// Manages a Storage Sync Server Endpoint.
///
/// > **NOTE:** The parent `azure.storage.SyncGroup` must have an `azure.storage.SyncCloudEndpoint` available before an `azure.storage.SyncServerEndpoint` resource can be created.
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
///             .name("example-storage-account")
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
///             .name("example-storage-share")
///             .quota(1)
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleSync = sync::create(
///         "exampleSync",
///         SyncArgs::builder()
///             .location("${example.location}")
///             .name("example-storage-sync")
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
///             .name("example-storage-sync-group")
///             .storage_sync_id("${exampleSync.id}")
///             .build_struct(),
///     );
///     let exampleSyncServerEndpoint = sync_server_endpoint::create(
///         "exampleSyncServerEndpoint",
///         SyncServerEndpointArgs::builder()
///             .name("example-storage-sync-server-endpoint")
///             .registered_server_id("${exampleSync.registeredServers[0]}")
///             .storage_sync_group_id("${exampleSyncGroup.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Sync Server Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/syncServerEndpoint:SyncServerEndpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StorageSync/storageSyncServices/sync1/syncGroups/syncGroup1/serverEndpoints/endpoint1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sync_server_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SyncServerEndpointArgs {
        /// Is Cloud Tiering Enabled? Defaults to `false`.
        #[builder(into, default)]
        pub cloud_tiering_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies how the server initially downloads the Azure file share data. Valid Values includes `NamespaceThenModifiedFiles`, `NamespaceOnly`, and `AvoidTieredFiles`. Defaults to `NamespaceThenModifiedFiles`.
        #[builder(into, default)]
        pub initial_download_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies how to handle the local cache. Valid Values include `UpdateLocallyCachedFiles` and `DownloadNewAndModifiedFiles`. Defaults to `UpdateLocallyCachedFiles`.
        #[builder(into, default)]
        pub local_cache_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Storage Sync. Changing this forces a new Storage Sync Server Endpoint to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Registered Server that will be associate with the Storage Sync Server Endpoint. Changing this forces a new Storage Sync Server Endpoint to be created.
        ///
        /// > **NOTE:** The target server must already be registered with the parent `azure.storage.Sync` prior to creating this endpoint. For more information on registering a server see the [Microsoft documentation](https://learn.microsoft.com/azure/storage/file-sync/file-sync-server-registration)
        #[builder(into)]
        pub registered_server_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The path on the Windows Server to be synced to the Azure file share. Changing this forces a new Storage Sync Server Endpoint to be created.
        #[builder(into)]
        pub server_local_path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Storage Sync Group where the Storage Sync Server Endpoint should exist. Changing this forces a new Storage Sync Server Endpoint to be created.
        #[builder(into)]
        pub storage_sync_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Files older than the specified age will be tiered to the cloud.
        #[builder(into, default)]
        pub tier_files_older_than_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// What percentage of free space on the volume should be preserved? Defaults to `20`.
        #[builder(into, default)]
        pub volume_free_space_percent: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct SyncServerEndpointResult {
        /// Is Cloud Tiering Enabled? Defaults to `false`.
        pub cloud_tiering_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies how the server initially downloads the Azure file share data. Valid Values includes `NamespaceThenModifiedFiles`, `NamespaceOnly`, and `AvoidTieredFiles`. Defaults to `NamespaceThenModifiedFiles`.
        pub initial_download_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies how to handle the local cache. Valid Values include `UpdateLocallyCachedFiles` and `DownloadNewAndModifiedFiles`. Defaults to `UpdateLocallyCachedFiles`.
        pub local_cache_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Storage Sync. Changing this forces a new Storage Sync Server Endpoint to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Registered Server that will be associate with the Storage Sync Server Endpoint. Changing this forces a new Storage Sync Server Endpoint to be created.
        ///
        /// > **NOTE:** The target server must already be registered with the parent `azure.storage.Sync` prior to creating this endpoint. For more information on registering a server see the [Microsoft documentation](https://learn.microsoft.com/azure/storage/file-sync/file-sync-server-registration)
        pub registered_server_id: pulumi_gestalt_rust::Output<String>,
        /// The path on the Windows Server to be synced to the Azure file share. Changing this forces a new Storage Sync Server Endpoint to be created.
        pub server_local_path: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Storage Sync Group where the Storage Sync Server Endpoint should exist. Changing this forces a new Storage Sync Server Endpoint to be created.
        pub storage_sync_group_id: pulumi_gestalt_rust::Output<String>,
        /// Files older than the specified age will be tiered to the cloud.
        pub tier_files_older_than_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// What percentage of free space on the volume should be preserved? Defaults to `20`.
        pub volume_free_space_percent: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SyncServerEndpointArgs,
    ) -> SyncServerEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cloud_tiering_enabled_binding = args
            .cloud_tiering_enabled
            .get_output(context)
            .get_inner();
        let initial_download_policy_binding = args
            .initial_download_policy
            .get_output(context)
            .get_inner();
        let local_cache_mode_binding = args
            .local_cache_mode
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let registered_server_id_binding = args
            .registered_server_id
            .get_output(context)
            .get_inner();
        let server_local_path_binding = args
            .server_local_path
            .get_output(context)
            .get_inner();
        let storage_sync_group_id_binding = args
            .storage_sync_group_id
            .get_output(context)
            .get_inner();
        let tier_files_older_than_days_binding = args
            .tier_files_older_than_days
            .get_output(context)
            .get_inner();
        let volume_free_space_percent_binding = args
            .volume_free_space_percent
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/syncServerEndpoint:SyncServerEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudTieringEnabled".into(),
                    value: &cloud_tiering_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "initialDownloadPolicy".into(),
                    value: &initial_download_policy_binding,
                },
                register_interface::ObjectField {
                    name: "localCacheMode".into(),
                    value: &local_cache_mode_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "registeredServerId".into(),
                    value: &registered_server_id_binding,
                },
                register_interface::ObjectField {
                    name: "serverLocalPath".into(),
                    value: &server_local_path_binding,
                },
                register_interface::ObjectField {
                    name: "storageSyncGroupId".into(),
                    value: &storage_sync_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "tierFilesOlderThanDays".into(),
                    value: &tier_files_older_than_days_binding,
                },
                register_interface::ObjectField {
                    name: "volumeFreeSpacePercent".into(),
                    value: &volume_free_space_percent_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SyncServerEndpointResult {
            cloud_tiering_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cloudTieringEnabled"),
            ),
            initial_download_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("initialDownloadPolicy"),
            ),
            local_cache_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localCacheMode"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            registered_server_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registeredServerId"),
            ),
            server_local_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverLocalPath"),
            ),
            storage_sync_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageSyncGroupId"),
            ),
            tier_files_older_than_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tierFilesOlderThanDays"),
            ),
            volume_free_space_percent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeFreeSpacePercent"),
            ),
        }
    }
}
