/// Manages a NFS Target within a HPC Cache.
///
/// > **NOTE:**: By request of the service team the provider no longer automatically registering the `Microsoft.StorageCache` Resource Provider for this resource. To register it you can run `az provider register --namespace 'Microsoft.StorageCache'`.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
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
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleHpc:
///     type: azure:network:Subnet
///     name: example_hpc
///     properties:
///       name: examplesubnethpc
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///   exampleCache:
///     type: azure:hpc:Cache
///     name: example
///     properties:
///       name: examplehpccache
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       cacheSizeInGb: 3072
///       subnetId: ${exampleHpc.id}
///       skuName: Standard_2G
///   exampleVm:
///     type: azure:network:Subnet
///     name: example_vm
///     properties:
///       name: examplesubnetvm
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleNetworkInterface:
///     type: azure:network:NetworkInterface
///     name: example
///     properties:
///       name: examplenic
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       ipConfigurations:
///         - name: internal
///           subnetId: ${exampleVm.id}
///           privateIpAddressAllocation: Dynamic
///   exampleLinuxVirtualMachine:
///     type: azure:compute:LinuxVirtualMachine
///     name: example
///     properties:
///       name: examplevm
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       size: Standard_F2
///       adminUsername: adminuser
///       networkInterfaceIds:
///         - ${exampleNetworkInterface.id}
///       adminSshKeys:
///         - username: adminuser
///           publicKey:
///             fn::invoke:
///               function: std:file
///               arguments:
///                 input: ~/.ssh/id_rsa.pub
///               return: result
///       osDisk:
///         caching: ReadWrite
///         storageAccountType: Standard_LRS
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///       customData:
///         fn::invoke:
///           function: std:base64encode
///           arguments:
///             input: ${customData}
///           return: result
///   exampleCacheNfsTarget:
///     type: azure:hpc:CacheNfsTarget
///     name: example
///     properties:
///       name: examplehpcnfstarget
///       resourceGroupName: ${example.name}
///       cacheName: ${exampleCache.name}
///       targetHostName: ${exampleLinuxVirtualMachine.privateIpAddress}
///       usageModel: READ_HEAVY_INFREQ
///       namespaceJunctions:
///         - namespacePath: /nfs/a1
///           nfsExport: /export/a
///           targetPath: '1'
///         - namespacePath: /nfs/b
///           nfsExport: /export/b
/// variables:
///   customData: "#!/bin/bash\nsudo -i \napt-get install -y nfs-kernel-server\nmkdir -p /export/a/1\nmkdir -p /export/a/2\nmkdir -p /export/b\ncat << EOF > /etc/exports\n/export/a *(rw,fsid=0,insecure,no_subtree_check,async)\n/export/b *(rw,fsid=0,insecure,no_subtree_check,async)\nEOF\nsystemctl start nfs-server\nexportfs -arv\n"
/// ```
///
/// ## Import
///
/// NFS Target within a HPC Cache can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:hpc/cacheNfsTarget:CacheNfsTarget example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StorageCache/caches/cache1/storageTargets/target1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cache_nfs_target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CacheNfsTargetArgs {
        /// The name HPC Cache, which the HPC Cache NFS Target will be added to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cache_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the HPC Cache NFS Target. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Can be specified multiple times to define multiple `namespace_junction`. Each `namespace_junction` block supports fields documented below.
        #[builder(into)]
        pub namespace_junctions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::hpc::CacheNfsTargetNamespaceJunction>,
        >,
        /// The name of the Resource Group in which to create the HPC Cache NFS Target. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IP address or fully qualified domain name (FQDN) of the HPC Cache NFS target. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_host_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of usage of the HPC Cache NFS Target. Possible values are: `READ_HEAVY_INFREQ`, `READ_HEAVY_CHECK_180`, `READ_ONLY`, `READ_WRITE`, `WRITE_WORKLOAD_15`, `WRITE_AROUND`, `WRITE_WORKLOAD_CHECK_30`, `WRITE_WORKLOAD_CHECK_60` and `WRITE_WORKLOAD_CLOUDWS`.
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
    pub struct CacheNfsTargetResult {
        /// The name HPC Cache, which the HPC Cache NFS Target will be added to. Changing this forces a new resource to be created.
        pub cache_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the HPC Cache NFS Target. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Can be specified multiple times to define multiple `namespace_junction`. Each `namespace_junction` block supports fields documented below.
        pub namespace_junctions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::hpc::CacheNfsTargetNamespaceJunction>,
        >,
        /// The name of the Resource Group in which to create the HPC Cache NFS Target. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The IP address or fully qualified domain name (FQDN) of the HPC Cache NFS target. Changing this forces a new resource to be created.
        pub target_host_name: pulumi_gestalt_rust::Output<String>,
        /// The type of usage of the HPC Cache NFS Target. Possible values are: `READ_HEAVY_INFREQ`, `READ_HEAVY_CHECK_180`, `READ_ONLY`, `READ_WRITE`, `WRITE_WORKLOAD_15`, `WRITE_AROUND`, `WRITE_WORKLOAD_CHECK_30`, `WRITE_WORKLOAD_CHECK_60` and `WRITE_WORKLOAD_CLOUDWS`.
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
        args: CacheNfsTargetArgs,
    ) -> CacheNfsTargetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cache_name_binding = args.cache_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let namespace_junctions_binding = args
            .namespace_junctions
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let target_host_name_binding = args
            .target_host_name
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
            type_: "azure:hpc/cacheNfsTarget:CacheNfsTarget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cacheName".into(),
                    value: &cache_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namespaceJunctions".into(),
                    value: &namespace_junctions_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "targetHostName".into(),
                    value: &target_host_name_binding,
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
        CacheNfsTargetResult {
            cache_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cacheName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            namespace_junctions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namespaceJunctions"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            target_host_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetHostName"),
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
