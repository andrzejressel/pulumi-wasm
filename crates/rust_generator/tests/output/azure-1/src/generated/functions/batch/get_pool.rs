#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPoolArgs {
        /// The Azure Storage Account name.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the user account.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPoolResult {
        /// The Azure Storage Account name.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// A `auto_scale` block that describes the scale settings when using auto scale.
        pub auto_scales: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetPoolAutoScale>,
        >,
        /// One or more `certificate` blocks that describe the certificates installed on each compute node in the pool.
        pub certificates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetPoolCertificate>,
        >,
        /// The container configuration used in the pool's VMs.
        pub container_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetPoolContainerConfiguration>,
        >,
        /// A `data_disks` block describes the data disk settings.
        pub data_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetPoolDataDisk>,
        >,
        /// A `disk_encryption` block describes the disk encryption configuration applied on compute nodes in the pool.
        pub disk_encryptions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetPoolDiskEncryption>,
        >,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// An `extensions` block describes the extension settings
        pub extensions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetPoolExtension>,
        >,
        /// A `fixed_scale` block that describes the scale settings when using fixed scale.
        pub fixed_scales: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetPoolFixedScale>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Whether the pool permits direct communication between nodes. This imposes restrictions on which nodes can be assigned to the pool. Enabling this value can reduce the chance of the requested number of nodes to be allocated in the pool.
        pub inter_node_communication: pulumi_gestalt_rust::Output<String>,
        /// The type of on-premises license to be used when deploying the operating system.
        pub license_type: pulumi_gestalt_rust::Output<String>,
        /// The maximum number of tasks that can run concurrently on a single compute node in the pool.
        pub max_tasks_per_node: pulumi_gestalt_rust::Output<i32>,
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A `mount` block that describes mount configuration.
        pub mounts: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetPoolMount>,
        >,
        /// The name of the user account.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetPoolNetworkConfiguration>,
        >,
        /// The SKU of the node agents in the Batch pool.
        pub node_agent_sku_id: pulumi_gestalt_rust::Output<String>,
        /// A `node_placement` block that describes the placement policy for allocating nodes in the pool.
        pub node_placements: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetPoolNodePlacement>,
        >,
        /// Specifies the ephemeral disk placement for operating system disk for all VMs in the pool.
        pub os_disk_placement: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `start_task` block that describes the start task settings for the Batch pool.
        pub start_tasks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetPoolStartTask>,
        >,
        /// The reference of the storage image used by the nodes in the Batch pool.
        pub storage_image_references: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetPoolStorageImageReference>,
        >,
        /// A `task_scheduling_policy` block that describes how tasks are distributed across compute nodes in a pool.
        pub task_scheduling_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetPoolTaskSchedulingPolicy>,
        >,
        /// A `user_accounts` block that describes the list of user accounts to be created on each node in the pool.
        pub user_accounts: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetPoolUserAccount>,
        >,
        /// The size of the VM created in the Batch pool.
        pub vm_size: pulumi_gestalt_rust::Output<String>,
        /// A `windows` block that describes the Windows configuration in the pool.
        pub windows: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetPoolWindow>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPoolArgs,
    ) -> GetPoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:batch/getPool:getPool".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPoolResult {
            account_name: o.get_field("accountName"),
            auto_scales: o.get_field("autoScales"),
            certificates: o.get_field("certificates"),
            container_configurations: o.get_field("containerConfigurations"),
            data_disks: o.get_field("dataDisks"),
            disk_encryptions: o.get_field("diskEncryptions"),
            display_name: o.get_field("displayName"),
            extensions: o.get_field("extensions"),
            fixed_scales: o.get_field("fixedScales"),
            id: o.get_field("id"),
            inter_node_communication: o.get_field("interNodeCommunication"),
            license_type: o.get_field("licenseType"),
            max_tasks_per_node: o.get_field("maxTasksPerNode"),
            metadata: o.get_field("metadata"),
            mounts: o.get_field("mounts"),
            name: o.get_field("name"),
            network_configurations: o.get_field("networkConfigurations"),
            node_agent_sku_id: o.get_field("nodeAgentSkuId"),
            node_placements: o.get_field("nodePlacements"),
            os_disk_placement: o.get_field("osDiskPlacement"),
            resource_group_name: o.get_field("resourceGroupName"),
            start_tasks: o.get_field("startTasks"),
            storage_image_references: o.get_field("storageImageReferences"),
            task_scheduling_policies: o.get_field("taskSchedulingPolicies"),
            user_accounts: o.get_field("userAccounts"),
            vm_size: o.get_field("vmSize"),
            windows: o.get_field("windows"),
        }
    }
}
