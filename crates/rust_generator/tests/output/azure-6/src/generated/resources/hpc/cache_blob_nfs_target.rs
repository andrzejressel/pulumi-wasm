/// Manages a Blob NFSv3 Target within a HPC Cache.
///
/// > **NOTE:**: By request of the service team the provider no longer automatically registering the `Microsoft.StorageCache` Resource Provider for this resource. To register it you can run `az provider register --namespace 'Microsoft.StorageCache'`.
///
/// > **NOTE:**: This resource depends on the NFSv3 enabled Storage Account, which has some prerequisites need to meet. Please checkout: <https://docs.microsoft.com/azure/storage/blobs/network-file-system-protocol-support-how-to?tabs=azure-powershell>.
///
/// ## Import
///
/// HPC Cache Blob NFS Targets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:hpc/cacheBlobNfsTarget:CacheBlobNfsTarget example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StorageCache/caches/cache1/storageTargets/target1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cache_blob_nfs_target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CacheBlobNfsTargetArgs {
        /// The name of the access policy applied to this target. Defaults to `default`.
        #[builder(into, default)]
        pub access_policy_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the HPC Cache, which the HPC Cache Blob NFS Target will be added to. Changing this forces a new HPC Cache Blob NFS Target to be created.
        #[builder(into)]
        pub cache_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this HPC Cache Blob NFS Target. Changing this forces a new HPC Cache Blob NFS Target to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The client-facing file path of the HPC Cache Blob NFS Target.
        #[builder(into)]
        pub namespace_path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the HPC Cache Blob NFS Target should exist. Changing this forces a new HPC Cache Blob NFS Target to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Resource Manager ID of the Storage Container used as the HPC Cache Blob NFS Target. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This is the Resource Manager ID of the Storage Container, rather than the regular ID - and can be accessed on the `azure.storage.Container` Data Source/Resource as `resource_manager_id`.
        #[builder(into)]
        pub storage_container_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of usage of the HPC Cache Blob NFS Target. Possible values are: `READ_HEAVY_INFREQ`, `READ_HEAVY_CHECK_180`, `READ_ONLY`, `READ_WRITE`, `WRITE_WORKLOAD_15`, `WRITE_AROUND`, `WRITE_WORKLOAD_CHECK_30`, `WRITE_WORKLOAD_CHECK_60` and `WRITE_WORKLOAD_CLOUDWS`.
        #[builder(into)]
        pub usage_model: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The amount of time the cache waits before it checks the back-end storage for file updates. Possible values are between `1` and `31536000`.
        #[builder(into, default)]
        pub verification_timer_in_seconds: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The amount of time the cache waits after the last file change before it copies the changed file to back-end storage. Possible values are between `1` and `31536000`.
        #[builder(into, default)]
        pub write_back_timer_in_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct CacheBlobNfsTargetResult {
        /// The name of the access policy applied to this target. Defaults to `default`.
        pub access_policy_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the HPC Cache, which the HPC Cache Blob NFS Target will be added to. Changing this forces a new HPC Cache Blob NFS Target to be created.
        pub cache_name: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this HPC Cache Blob NFS Target. Changing this forces a new HPC Cache Blob NFS Target to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The client-facing file path of the HPC Cache Blob NFS Target.
        pub namespace_path: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the HPC Cache Blob NFS Target should exist. Changing this forces a new HPC Cache Blob NFS Target to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Resource Manager ID of the Storage Container used as the HPC Cache Blob NFS Target. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This is the Resource Manager ID of the Storage Container, rather than the regular ID - and can be accessed on the `azure.storage.Container` Data Source/Resource as `resource_manager_id`.
        pub storage_container_id: pulumi_gestalt_rust::Output<String>,
        /// The type of usage of the HPC Cache Blob NFS Target. Possible values are: `READ_HEAVY_INFREQ`, `READ_HEAVY_CHECK_180`, `READ_ONLY`, `READ_WRITE`, `WRITE_WORKLOAD_15`, `WRITE_AROUND`, `WRITE_WORKLOAD_CHECK_30`, `WRITE_WORKLOAD_CHECK_60` and `WRITE_WORKLOAD_CLOUDWS`.
        pub usage_model: pulumi_gestalt_rust::Output<String>,
        /// The amount of time the cache waits before it checks the back-end storage for file updates. Possible values are between `1` and `31536000`.
        pub verification_timer_in_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The amount of time the cache waits after the last file change before it copies the changed file to back-end storage. Possible values are between `1` and `31536000`.
        pub write_back_timer_in_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CacheBlobNfsTargetArgs,
    ) -> CacheBlobNfsTargetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let access_policy_name_binding = args
            .access_policy_name
            .get_output(context)
            .get_inner();
        let cache_name_binding = args.cache_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let namespace_path_binding = args.namespace_path.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let storage_container_id_binding = args
            .storage_container_id
            .get_output(context)
            .get_inner();
        let usage_model_binding = args.usage_model.get_output(context).get_inner();
        let verification_timer_in_seconds_binding = args
            .verification_timer_in_seconds
            .get_output(context)
            .get_inner();
        let write_back_timer_in_seconds_binding = args
            .write_back_timer_in_seconds
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:hpc/cacheBlobNfsTarget:CacheBlobNfsTarget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessPolicyName".into(),
                    value: &access_policy_name_binding,
                },
                register_interface::ObjectField {
                    name: "cacheName".into(),
                    value: &cache_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namespacePath".into(),
                    value: &namespace_path_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageContainerId".into(),
                    value: &storage_container_id_binding,
                },
                register_interface::ObjectField {
                    name: "usageModel".into(),
                    value: &usage_model_binding,
                },
                register_interface::ObjectField {
                    name: "verificationTimerInSeconds".into(),
                    value: &verification_timer_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "writeBackTimerInSeconds".into(),
                    value: &write_back_timer_in_seconds_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CacheBlobNfsTargetResult {
            access_policy_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessPolicyName"),
            ),
            cache_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cacheName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            namespace_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namespacePath"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            storage_container_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageContainerId"),
            ),
            usage_model: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("usageModel"),
            ),
            verification_timer_in_seconds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("verificationTimerInSeconds"),
            ),
            write_back_timer_in_seconds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("writeBackTimerInSeconds"),
            ),
        }
    }
}
