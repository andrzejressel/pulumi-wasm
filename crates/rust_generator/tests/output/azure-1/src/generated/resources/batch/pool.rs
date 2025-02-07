/// Manages an Azure Batch pool.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: testaccbatch
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: testaccsa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleAccount2:
///     type: azure:batch:Account
///     name: example
///     properties:
///       name: testaccbatch
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       poolAllocationMode: BatchService
///       storageAccountId: ${exampleAccount.id}
///       storageAccountAuthenticationMode: StorageKeys
///       tags:
///         env: test
///   exampleCertificate:
///     type: azure:batch:Certificate
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       accountName: ${exampleAccount2.name}
///       certificate:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: certificate.cer
///           return: result
///       format: Cer
///       thumbprint: 312d31a79fa0cef49c00f769afc2b73e9f4edf34
///       thumbprintAlgorithm: SHA1
///   examplePool:
///     type: azure:batch:Pool
///     name: example
///     properties:
///       name: testaccpool
///       resourceGroupName: ${example.name}
///       accountName: ${exampleAccount2.name}
///       displayName: Test Acc Pool Auto
///       vmSize: Standard_A1
///       nodeAgentSkuId: batch.node.ubuntu 20.04
///       autoScale:
///         evaluationInterval: PT15M
///         formula: |2
///                 startingNumberOfVMs = 1;
///                 maxNumberofVMs = 25;
///                 pendingTaskSamplePercent = $PendingTasks.GetSamplePercent(180 * TimeInterval_Second);
///                 pendingTaskSamples = pendingTaskSamplePercent < 70 ? startingNumberOfVMs : avg($PendingTasks.GetSample(180 *   TimeInterval_Second));
///                 $TargetDedicatedNodes=min(maxNumberofVMs, pendingTaskSamples);
///       storageImageReference:
///         publisher: microsoft-azure-batch
///         offer: ubuntu-server-container
///         sku: 20-04-lts
///         version: latest
///       containerConfiguration:
///         type: DockerCompatible
///         containerRegistries:
///           - registryServer: docker.io
///             userName: login
///             password: apassword
///       startTask:
///         commandLine: echo 'Hello World from $env'
///         taskRetryMaximum: 1
///         waitForSuccess: true
///         commonEnvironmentProperties:
///           env: TEST
///         userIdentity:
///           autoUser:
///             elevationLevel: NonAdmin
///             scope: Task
///       certificates:
///         - id: ${exampleCertificate.id}
///           storeLocation: CurrentUser
///           visibilities:
///             - StartTask
/// ```
///
/// ## Import
///
/// Batch Pools can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:batch/pool:Pool example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myGroup1/providers/Microsoft.Batch/batchAccounts/myBatchAccount1/pools/myBatchPool1
/// ```
///
pub mod pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PoolArgs {
        /// Specifies the name of the Batch account in which the pool will be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `auto_scale` block that describes the scale settings when using auto scale as defined below.
        #[builder(into, default)]
        pub auto_scale: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::batch::PoolAutoScale>,
        >,
        /// One or more `certificate` blocks that describe the certificates to be installed on each compute node in the pool as defined below.
        #[builder(into, default)]
        pub certificates: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::batch::PoolCertificate>>,
        >,
        /// The container configuration used in the pool's VMs. One `container_configuration` block as defined below.
        #[builder(into, default)]
        pub container_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::batch::PoolContainerConfiguration>,
        >,
        /// A `data_disks` block describes the data disk settings as defined below.
        #[builder(into, default)]
        pub data_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::batch::PoolDataDisk>>,
        >,
        /// A `disk_encryption` block, as defined below, describes the disk encryption configuration applied on compute nodes in the pool. Disk encryption configuration is not supported on Linux pool created with Virtual Machine Image or Shared Image Gallery Image.
        #[builder(into, default)]
        pub disk_encryptions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::batch::PoolDiskEncryption>>,
        >,
        /// Specifies the display name of the Batch pool. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `extensions` block as defined below.
        #[builder(into, default)]
        pub extensions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::batch::PoolExtension>>,
        >,
        /// A `fixed_scale` block that describes the scale settings when using fixed scale as defined below.
        #[builder(into, default)]
        pub fixed_scale: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::batch::PoolFixedScale>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::batch::PoolIdentity>,
        >,
        /// Whether the pool permits direct communication between nodes. This imposes restrictions on which nodes can be assigned to the pool. Enabling this value can reduce the chance of the requested number of nodes to be allocated in the pool. Values allowed are `Disabled` and `Enabled`. Defaults to `Enabled`.
        #[builder(into, default)]
        pub inter_node_communication: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of on-premises license to be used when deploying the operating system. This only applies to images that contain the Windows operating system, and should only be used when you hold valid on-premises licenses for the nodes which will be deployed. If omitted, no on-premises licensing discount is applied. Values are: "Windows_Server" - The on-premises license is for Windows Server. "Windows_Client" - The on-premises license is for Windows Client.
        #[builder(into, default)]
        pub license_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the maximum number of tasks that can run concurrently on a single compute node in the pool. Defaults to `1`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub max_tasks_per_node: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A map of custom batch pool metadata.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `mount` block defined as below.
        #[builder(into, default)]
        pub mounts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::batch::PoolMount>>,
        >,
        /// Specifies the name of the Batch pool. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `network_configuration` block that describes the network configurations for the Batch pool as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub network_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::batch::PoolNetworkConfiguration>,
        >,
        /// Specifies the SKU of the node agents that will be created in the Batch pool. Changing this forces a new resource to be created.
        #[builder(into)]
        pub node_agent_sku_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `node_placement` block that describes the placement policy for allocating nodes in the pool as defined below.
        #[builder(into, default)]
        pub node_placements: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::batch::PoolNodePlacement>>,
        >,
        /// Specifies the ephemeral disk placement for operating system disk for all VMs in the pool. This property can be used by user in the request to choose which location the operating system should be in. e.g., cache disk space for Ephemeral OS disk provisioning. For more information on Ephemeral OS disk size requirements, please refer to Ephemeral OS disk size requirements for Windows VMs at <https://docs.microsoft.com/en-us/azure/virtual-machines/windows/ephemeral-os-disks#size-requirements> and Linux VMs at <https://docs.microsoft.com/en-us/azure/virtual-machines/linux/ephemeral-os-disks#size-requirements>. The only possible value is `CacheDisk`.
        #[builder(into, default)]
        pub os_disk_placement: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Batch pool. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `start_task` block that describes the start task settings for the Batch pool as defined below.
        #[builder(into, default)]
        pub start_task: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::batch::PoolStartTask>,
        >,
        /// Whether to stop if there is a pending resize operation on this pool.
        #[builder(into, default)]
        pub stop_pending_resize_operation: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A `storage_image_reference` block for the virtual machines that will compose the Batch pool as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_image_reference: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::batch::PoolStorageImageReference,
        >,
        /// The desired node communication mode for the pool. Possible values are `Classic`, `Default` and `Simplified`.
        #[builder(into, default)]
        pub target_node_communication_mode: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A `task_scheduling_policy` block that describes how tasks are distributed across compute nodes in a pool as defined below. If not specified, the default is spread as defined below.
        #[builder(into, default)]
        pub task_scheduling_policies: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::batch::PoolTaskSchedulingPolicy>>,
        >,
        /// A `user_accounts` block that describes the list of user accounts to be created on each node in the pool as defined below.
        #[builder(into, default)]
        pub user_accounts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::batch::PoolUserAccount>>,
        >,
        /// Specifies the size of the VM created in the Batch pool. Changing this forces a new resource to be created.
        #[builder(into)]
        pub vm_size: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `windows` block that describes the Windows configuration in the pool as defined below.
        ///
        /// > **NOTE:** For Windows compute nodes, the Batch service installs the certificates to the specified certificate store and location. For Linux compute nodes, the certificates are stored in a directory inside the task working directory and an environment variable `AZ_BATCH_CERTIFICATES_DIR` is supplied to the task to query for this location. For certificates with visibility of `remoteUser`, a `certs` directory is created in the user's home directory (e.g., `/home/{user-name}/certs`) and certificates are placed in that directory.
        ///
        /// > **Please Note:** `fixed_scale` and `auto_scale` blocks cannot be used both at the same time.
        #[builder(into, default)]
        pub windows: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::batch::PoolWindow>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PoolResult {
        /// Specifies the name of the Batch account in which the pool will be created. Changing this forces a new resource to be created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// A `auto_scale` block that describes the scale settings when using auto scale as defined below.
        pub auto_scale: pulumi_gestalt_rust::Output<
            Option<super::super::types::batch::PoolAutoScale>,
        >,
        /// One or more `certificate` blocks that describe the certificates to be installed on each compute node in the pool as defined below.
        pub certificates: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::batch::PoolCertificate>>,
        >,
        /// The container configuration used in the pool's VMs. One `container_configuration` block as defined below.
        pub container_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::batch::PoolContainerConfiguration>,
        >,
        /// A `data_disks` block describes the data disk settings as defined below.
        pub data_disks: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::batch::PoolDataDisk>>,
        >,
        /// A `disk_encryption` block, as defined below, describes the disk encryption configuration applied on compute nodes in the pool. Disk encryption configuration is not supported on Linux pool created with Virtual Machine Image or Shared Image Gallery Image.
        pub disk_encryptions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::batch::PoolDiskEncryption>>,
        >,
        /// Specifies the display name of the Batch pool. Changing this forces a new resource to be created.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `extensions` block as defined below.
        pub extensions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::batch::PoolExtension>>,
        >,
        /// A `fixed_scale` block that describes the scale settings when using fixed scale as defined below.
        pub fixed_scale: pulumi_gestalt_rust::Output<
            Option<super::super::types::batch::PoolFixedScale>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::batch::PoolIdentity>,
        >,
        /// Whether the pool permits direct communication between nodes. This imposes restrictions on which nodes can be assigned to the pool. Enabling this value can reduce the chance of the requested number of nodes to be allocated in the pool. Values allowed are `Disabled` and `Enabled`. Defaults to `Enabled`.
        pub inter_node_communication: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of on-premises license to be used when deploying the operating system. This only applies to images that contain the Windows operating system, and should only be used when you hold valid on-premises licenses for the nodes which will be deployed. If omitted, no on-premises licensing discount is applied. Values are: "Windows_Server" - The on-premises license is for Windows Server. "Windows_Client" - The on-premises license is for Windows Client.
        pub license_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the maximum number of tasks that can run concurrently on a single compute node in the pool. Defaults to `1`. Changing this forces a new resource to be created.
        pub max_tasks_per_node: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A map of custom batch pool metadata.
        pub metadata: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `mount` block defined as below.
        pub mounts: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::batch::PoolMount>>,
        >,
        /// Specifies the name of the Batch pool. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `network_configuration` block that describes the network configurations for the Batch pool as defined below. Changing this forces a new resource to be created.
        pub network_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::batch::PoolNetworkConfiguration>,
        >,
        /// Specifies the SKU of the node agents that will be created in the Batch pool. Changing this forces a new resource to be created.
        pub node_agent_sku_id: pulumi_gestalt_rust::Output<String>,
        /// A `node_placement` block that describes the placement policy for allocating nodes in the pool as defined below.
        pub node_placements: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::batch::PoolNodePlacement>>,
        >,
        /// Specifies the ephemeral disk placement for operating system disk for all VMs in the pool. This property can be used by user in the request to choose which location the operating system should be in. e.g., cache disk space for Ephemeral OS disk provisioning. For more information on Ephemeral OS disk size requirements, please refer to Ephemeral OS disk size requirements for Windows VMs at <https://docs.microsoft.com/en-us/azure/virtual-machines/windows/ephemeral-os-disks#size-requirements> and Linux VMs at <https://docs.microsoft.com/en-us/azure/virtual-machines/linux/ephemeral-os-disks#size-requirements>. The only possible value is `CacheDisk`.
        pub os_disk_placement: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Batch pool. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `start_task` block that describes the start task settings for the Batch pool as defined below.
        pub start_task: pulumi_gestalt_rust::Output<
            Option<super::super::types::batch::PoolStartTask>,
        >,
        /// Whether to stop if there is a pending resize operation on this pool.
        pub stop_pending_resize_operation: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `storage_image_reference` block for the virtual machines that will compose the Batch pool as defined below. Changing this forces a new resource to be created.
        pub storage_image_reference: pulumi_gestalt_rust::Output<
            super::super::types::batch::PoolStorageImageReference,
        >,
        /// The desired node communication mode for the pool. Possible values are `Classic`, `Default` and `Simplified`.
        pub target_node_communication_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `task_scheduling_policy` block that describes how tasks are distributed across compute nodes in a pool as defined below. If not specified, the default is spread as defined below.
        pub task_scheduling_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::types::batch::PoolTaskSchedulingPolicy>,
        >,
        /// A `user_accounts` block that describes the list of user accounts to be created on each node in the pool as defined below.
        pub user_accounts: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::batch::PoolUserAccount>>,
        >,
        /// Specifies the size of the VM created in the Batch pool. Changing this forces a new resource to be created.
        pub vm_size: pulumi_gestalt_rust::Output<String>,
        /// A `windows` block that describes the Windows configuration in the pool as defined below.
        ///
        /// > **NOTE:** For Windows compute nodes, the Batch service installs the certificates to the specified certificate store and location. For Linux compute nodes, the certificates are stored in a directory inside the task working directory and an environment variable `AZ_BATCH_CERTIFICATES_DIR` is supplied to the task to query for this location. For certificates with visibility of `remoteUser`, a `certs` directory is created in the user's home directory (e.g., `/home/{user-name}/certs`) and certificates are placed in that directory.
        ///
        /// > **Please Note:** `fixed_scale` and `auto_scale` blocks cannot be used both at the same time.
        pub windows: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::batch::PoolWindow>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PoolArgs,
    ) -> PoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_output(context).get_inner();
        let auto_scale_binding = args.auto_scale.get_output(context).get_inner();
        let certificates_binding = args.certificates.get_output(context).get_inner();
        let container_configuration_binding = args
            .container_configuration
            .get_output(context)
            .get_inner();
        let data_disks_binding = args.data_disks.get_output(context).get_inner();
        let disk_encryptions_binding = args
            .disk_encryptions
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let extensions_binding = args.extensions.get_output(context).get_inner();
        let fixed_scale_binding = args.fixed_scale.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let inter_node_communication_binding = args
            .inter_node_communication
            .get_output(context)
            .get_inner();
        let license_type_binding = args.license_type.get_output(context).get_inner();
        let max_tasks_per_node_binding = args
            .max_tasks_per_node
            .get_output(context)
            .get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let mounts_binding = args.mounts.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_configuration_binding = args
            .network_configuration
            .get_output(context)
            .get_inner();
        let node_agent_sku_id_binding = args
            .node_agent_sku_id
            .get_output(context)
            .get_inner();
        let node_placements_binding = args
            .node_placements
            .get_output(context)
            .get_inner();
        let os_disk_placement_binding = args
            .os_disk_placement
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let start_task_binding = args.start_task.get_output(context).get_inner();
        let stop_pending_resize_operation_binding = args
            .stop_pending_resize_operation
            .get_output(context)
            .get_inner();
        let storage_image_reference_binding = args
            .storage_image_reference
            .get_output(context)
            .get_inner();
        let target_node_communication_mode_binding = args
            .target_node_communication_mode
            .get_output(context)
            .get_inner();
        let task_scheduling_policies_binding = args
            .task_scheduling_policies
            .get_output(context)
            .get_inner();
        let user_accounts_binding = args.user_accounts.get_output(context).get_inner();
        let vm_size_binding = args.vm_size.get_output(context).get_inner();
        let windows_binding = args.windows.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:batch/pool:Pool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "autoScale".into(),
                    value: &auto_scale_binding,
                },
                register_interface::ObjectField {
                    name: "certificates".into(),
                    value: &certificates_binding,
                },
                register_interface::ObjectField {
                    name: "containerConfiguration".into(),
                    value: &container_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "dataDisks".into(),
                    value: &data_disks_binding,
                },
                register_interface::ObjectField {
                    name: "diskEncryptions".into(),
                    value: &disk_encryptions_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "extensions".into(),
                    value: &extensions_binding,
                },
                register_interface::ObjectField {
                    name: "fixedScale".into(),
                    value: &fixed_scale_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "interNodeCommunication".into(),
                    value: &inter_node_communication_binding,
                },
                register_interface::ObjectField {
                    name: "licenseType".into(),
                    value: &license_type_binding,
                },
                register_interface::ObjectField {
                    name: "maxTasksPerNode".into(),
                    value: &max_tasks_per_node_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "mounts".into(),
                    value: &mounts_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfiguration".into(),
                    value: &network_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "nodeAgentSkuId".into(),
                    value: &node_agent_sku_id_binding,
                },
                register_interface::ObjectField {
                    name: "nodePlacements".into(),
                    value: &node_placements_binding,
                },
                register_interface::ObjectField {
                    name: "osDiskPlacement".into(),
                    value: &os_disk_placement_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "startTask".into(),
                    value: &start_task_binding,
                },
                register_interface::ObjectField {
                    name: "stopPendingResizeOperation".into(),
                    value: &stop_pending_resize_operation_binding,
                },
                register_interface::ObjectField {
                    name: "storageImageReference".into(),
                    value: &storage_image_reference_binding,
                },
                register_interface::ObjectField {
                    name: "targetNodeCommunicationMode".into(),
                    value: &target_node_communication_mode_binding,
                },
                register_interface::ObjectField {
                    name: "taskSchedulingPolicies".into(),
                    value: &task_scheduling_policies_binding,
                },
                register_interface::ObjectField {
                    name: "userAccounts".into(),
                    value: &user_accounts_binding,
                },
                register_interface::ObjectField {
                    name: "vmSize".into(),
                    value: &vm_size_binding,
                },
                register_interface::ObjectField {
                    name: "windows".into(),
                    value: &windows_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PoolResult {
            account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountName"),
            ),
            auto_scale: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoScale"),
            ),
            certificates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificates"),
            ),
            container_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("containerConfiguration"),
            ),
            data_disks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataDisks"),
            ),
            disk_encryptions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskEncryptions"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            extensions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("extensions"),
            ),
            fixed_scale: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fixedScale"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            inter_node_communication: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("interNodeCommunication"),
            ),
            license_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseType"),
            ),
            max_tasks_per_node: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxTasksPerNode"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            mounts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mounts"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkConfiguration"),
            ),
            node_agent_sku_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeAgentSkuId"),
            ),
            node_placements: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodePlacements"),
            ),
            os_disk_placement: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("osDiskPlacement"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            start_task: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("startTask"),
            ),
            stop_pending_resize_operation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stopPendingResizeOperation"),
            ),
            storage_image_reference: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageImageReference"),
            ),
            target_node_communication_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetNodeCommunicationMode"),
            ),
            task_scheduling_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("taskSchedulingPolicies"),
            ),
            user_accounts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userAccounts"),
            ),
            vm_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vmSize"),
            ),
            windows: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("windows"),
            ),
        }
    }
}
