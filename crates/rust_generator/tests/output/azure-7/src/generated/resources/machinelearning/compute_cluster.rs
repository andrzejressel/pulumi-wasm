/// Manages a Machine Learning Compute Cluster.
/// **NOTE:** At this point in time the resource cannot be updated (not supported by the backend Azure Go SDK). Therefore it can only be created and deleted, not updated. At the moment, there is also no possibility to specify ssh User Account Credentials to ssh into the compute cluster.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: west europe
///       tags:
///         stage: example
///   exampleInsights:
///     type: azure:appinsights:Insights
///     name: example
///     properties:
///       name: example-ai
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationType: web
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: example-kv
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: standard
///       purgeProtectionEnabled: true
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplesa
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleWorkspace:
///     type: azure:machinelearning:Workspace
///     name: example
///     properties:
///       name: example-mlw
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationInsightsId: ${exampleInsights.id}
///       keyVaultId: ${exampleKeyVault.id}
///       storageAccountId: ${exampleAccount.id}
///       identity:
///         type: SystemAssigned
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.1.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.1.0.0/24
///   test:
///     type: azure:machinelearning:ComputeCluster
///     properties:
///       name: example
///       location: ${example.location}
///       vmPriority: LowPriority
///       vmSize: Standard_DS2_v2
///       machineLearningWorkspaceId: ${exampleWorkspace.id}
///       subnetResourceId: ${exampleSubnet.id}
///       scaleSettings:
///         minNodeCount: 0
///         maxNodeCount: 1
///         scaleDownNodesAfterIdleDuration: PT30S
///       identity:
///         type: SystemAssigned
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Machine Learning Compute Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:machinelearning/computeCluster:ComputeCluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.MachineLearningServices/workspaces/workspace1/computes/cluster1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod compute_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ComputeClusterArgs {
        /// The description of the Machine Learning compute. Changing this forces a new Machine Learning Compute Cluster to be created.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::machinelearning::ComputeClusterIdentity>,
        >,
        /// Whether local authentication methods is enabled. Defaults to `true`. Changing this forces a new Machine Learning Compute Cluster to be created.
        #[builder(into, default)]
        pub local_auth_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Azure Region where the Machine Learning Compute Cluster should exist. Changing this forces a new Machine Learning Compute Cluster to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Machine Learning Workspace. Changing this forces a new Machine Learning Compute Cluster to be created.
        #[builder(into)]
        pub machine_learning_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Machine Learning Compute Cluster. Changing this forces a new Machine Learning Compute Cluster to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the compute cluster will have a public ip. To set this to false a `subnet_resource_id` needs to be set. Defaults to `true`. Changing this forces a new Machine Learning Compute Cluster to be created.
        #[builder(into, default)]
        pub node_public_ip_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `scale_settings` block as defined below. Changing this forces a new Machine Learning Compute Cluster to be created.
        #[builder(into)]
        pub scale_settings: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::machinelearning::ComputeClusterScaleSettings,
        >,
        /// Credentials for an administrator user account that will be created on each compute node. A `ssh` block as defined below. Changing this forces a new Machine Learning Compute Cluster to be created.
        #[builder(into, default)]
        pub ssh: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::machinelearning::ComputeClusterSsh>,
        >,
        /// A boolean value indicating whether enable the public SSH port. Defaults to `false`. Changing this forces a new Machine Learning Compute Cluster to be created.
        #[builder(into, default)]
        pub ssh_public_access_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Subnet that the Compute Cluster should reside in. Changing this forces a new Machine Learning Compute Cluster to be created.
        #[builder(into, default)]
        pub subnet_resource_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Machine Learning Compute Cluster. Changing this forces a new Machine Learning Compute Cluster to be created.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The priority of the VM. Changing this forces a new Machine Learning Compute Cluster to be created. Accepted values are `Dedicated` and `LowPriority`.
        #[builder(into)]
        pub vm_priority: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The size of the VM. Changing this forces a new Machine Learning Compute Cluster to be created.
        #[builder(into)]
        pub vm_size: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ComputeClusterResult {
        /// The description of the Machine Learning compute. Changing this forces a new Machine Learning Compute Cluster to be created.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::machinelearning::ComputeClusterIdentity>,
        >,
        /// Whether local authentication methods is enabled. Defaults to `true`. Changing this forces a new Machine Learning Compute Cluster to be created.
        pub local_auth_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Azure Region where the Machine Learning Compute Cluster should exist. Changing this forces a new Machine Learning Compute Cluster to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Machine Learning Workspace. Changing this forces a new Machine Learning Compute Cluster to be created.
        pub machine_learning_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Machine Learning Compute Cluster. Changing this forces a new Machine Learning Compute Cluster to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether the compute cluster will have a public ip. To set this to false a `subnet_resource_id` needs to be set. Defaults to `true`. Changing this forces a new Machine Learning Compute Cluster to be created.
        pub node_public_ip_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `scale_settings` block as defined below. Changing this forces a new Machine Learning Compute Cluster to be created.
        pub scale_settings: pulumi_gestalt_rust::Output<
            super::super::types::machinelearning::ComputeClusterScaleSettings,
        >,
        /// Credentials for an administrator user account that will be created on each compute node. A `ssh` block as defined below. Changing this forces a new Machine Learning Compute Cluster to be created.
        pub ssh: pulumi_gestalt_rust::Output<
            Option<super::super::types::machinelearning::ComputeClusterSsh>,
        >,
        /// A boolean value indicating whether enable the public SSH port. Defaults to `false`. Changing this forces a new Machine Learning Compute Cluster to be created.
        pub ssh_public_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Subnet that the Compute Cluster should reside in. Changing this forces a new Machine Learning Compute Cluster to be created.
        pub subnet_resource_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Machine Learning Compute Cluster. Changing this forces a new Machine Learning Compute Cluster to be created.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The priority of the VM. Changing this forces a new Machine Learning Compute Cluster to be created. Accepted values are `Dedicated` and `LowPriority`.
        pub vm_priority: pulumi_gestalt_rust::Output<String>,
        /// The size of the VM. Changing this forces a new Machine Learning Compute Cluster to be created.
        pub vm_size: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ComputeClusterArgs,
    ) -> ComputeClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let local_auth_enabled_binding = args.local_auth_enabled.get_output(context);
        let location_binding = args.location.get_output(context);
        let machine_learning_workspace_id_binding = args
            .machine_learning_workspace_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let node_public_ip_enabled_binding = args
            .node_public_ip_enabled
            .get_output(context);
        let scale_settings_binding = args.scale_settings.get_output(context);
        let ssh_binding = args.ssh.get_output(context);
        let ssh_public_access_enabled_binding = args
            .ssh_public_access_enabled
            .get_output(context);
        let subnet_resource_id_binding = args.subnet_resource_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vm_priority_binding = args.vm_priority.get_output(context);
        let vm_size_binding = args.vm_size.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:machinelearning/computeCluster:ComputeCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localAuthEnabled".into(),
                    value: local_auth_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "machineLearningWorkspaceId".into(),
                    value: machine_learning_workspace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodePublicIpEnabled".into(),
                    value: node_public_ip_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scaleSettings".into(),
                    value: scale_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ssh".into(),
                    value: ssh_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sshPublicAccessEnabled".into(),
                    value: ssh_public_access_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetResourceId".into(),
                    value: subnet_resource_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vmPriority".into(),
                    value: vm_priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vmSize".into(),
                    value: vm_size_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ComputeClusterResult {
            description: o.get_field("description"),
            identity: o.get_field("identity"),
            local_auth_enabled: o.get_field("localAuthEnabled"),
            location: o.get_field("location"),
            machine_learning_workspace_id: o.get_field("machineLearningWorkspaceId"),
            name: o.get_field("name"),
            node_public_ip_enabled: o.get_field("nodePublicIpEnabled"),
            scale_settings: o.get_field("scaleSettings"),
            ssh: o.get_field("ssh"),
            ssh_public_access_enabled: o.get_field("sshPublicAccessEnabled"),
            subnet_resource_id: o.get_field("subnetResourceId"),
            tags: o.get_field("tags"),
            vm_priority: o.get_field("vmPriority"),
            vm_size: o.get_field("vmSize"),
        }
    }
}
