/// Manages a Blob Target within a HPC Cache.
///
/// > **NOTE:**: By request of the service team the provider no longer automatically registering the `Microsoft.StorageCache` Resource Provider for this resource. To register it you can run `az provider register --namespace 'Microsoft.StorageCache'`.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: examplevn
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: examplesubnet
///       resourceGroupName: ${exampleResourceGroup.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///   exampleCache:
///     type: azure:hpc:Cache
///     name: example
///     properties:
///       name: examplehpccache
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       cacheSizeInGb: 3072
///       subnetId: ${exampleSubnet.id}
///       skuName: Standard_2G
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestorgaccount
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: examplestoragecontainer
///       storageAccountName: ${exampleAccount.name}
///   exampleStorageAccountContrib:
///     type: azure:authorization:Assignment
///     name: example_storage_account_contrib
///     properties:
///       scope: ${exampleAccount.id}
///       roleDefinitionName: Storage Account Contributor
///       principalId: ${example.objectId}
///   exampleStorageBlobDataContrib:
///     type: azure:authorization:Assignment
///     name: example_storage_blob_data_contrib
///     properties:
///       scope: ${exampleAccount.id}
///       roleDefinitionName: Storage Blob Data Contributor
///       principalId: ${example.objectId}
///   exampleCacheBlobTarget:
///     type: azure:hpc:CacheBlobTarget
///     name: example
///     properties:
///       name: examplehpccblobtarget
///       resourceGroupName: ${exampleResourceGroup.name}
///       cacheName: ${exampleCache.name}
///       storageContainerId: ${exampleContainer.resourceManagerId}
///       namespacePath: /blob_storage
/// variables:
///   example:
///     fn::invoke:
///       function: azuread:getServicePrincipal
///       arguments:
///         displayName: HPC Cache Resource Provider
/// ```
///
/// ## Import
///
/// Blob Targets within an HPC Cache can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:hpc/cacheBlobTarget:CacheBlobTarget example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StorageCache/caches/cache1/storageTargets/target1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cache_blob_target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CacheBlobTargetArgs {
        /// The name of the access policy applied to this target. Defaults to `default`.
        #[builder(into, default)]
        pub access_policy_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name HPC Cache, which the HPC Cache Blob Target will be added to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cache_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the HPC Cache Blob Target. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The client-facing file path of the HPC Cache Blob Target.
        #[builder(into)]
        pub namespace_path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which to create the HPC Cache Blob Target. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Resource Manager ID of the Storage Container used as the HPC Cache Blob Target. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This is the Resource Manager ID of the Storage Container, rather than the regular ID - and can be accessed on the `azure.storage.Container` Data Source/Resource as `resource_manager_id`.
        #[builder(into)]
        pub storage_container_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CacheBlobTargetResult {
        /// The name of the access policy applied to this target. Defaults to `default`.
        pub access_policy_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name HPC Cache, which the HPC Cache Blob Target will be added to. Changing this forces a new resource to be created.
        pub cache_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the HPC Cache Blob Target. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The client-facing file path of the HPC Cache Blob Target.
        pub namespace_path: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which to create the HPC Cache Blob Target. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Resource Manager ID of the Storage Container used as the HPC Cache Blob Target. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This is the Resource Manager ID of the Storage Container, rather than the regular ID - and can be accessed on the `azure.storage.Container` Data Source/Resource as `resource_manager_id`.
        pub storage_container_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CacheBlobTargetArgs,
    ) -> CacheBlobTargetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_policy_name_binding = args.access_policy_name.get_output(context);
        let cache_name_binding = args.cache_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let namespace_path_binding = args.namespace_path.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let storage_container_id_binding = args.storage_container_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:hpc/cacheBlobTarget:CacheBlobTarget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessPolicyName".into(),
                    value: &access_policy_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cacheName".into(),
                    value: &cache_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespacePath".into(),
                    value: &namespace_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageContainerId".into(),
                    value: &storage_container_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CacheBlobTargetResult {
            access_policy_name: o.get_field("accessPolicyName"),
            cache_name: o.get_field("cacheName"),
            name: o.get_field("name"),
            namespace_path: o.get_field("namespacePath"),
            resource_group_name: o.get_field("resourceGroupName"),
            storage_container_id: o.get_field("storageContainerId"),
        }
    }
}
