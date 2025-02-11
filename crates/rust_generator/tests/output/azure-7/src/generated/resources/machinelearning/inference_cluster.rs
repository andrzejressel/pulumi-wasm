/// Manages a Machine Learning Inference Cluster.
///
/// > **NOTE:** The Machine Learning Inference Cluster resource is used to attach an existing AKS cluster to the Machine Learning Workspace, it doesn't create the AKS cluster itself. Therefore it can only be created and deleted, not updated. Any change to the configuration will recreate the resource.
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
///   exampleKubernetesCluster:
///     type: azure:containerservice:KubernetesCluster
///     name: example
///     properties:
///       name: example-aks
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       dnsPrefixPrivateCluster: prefix
///       defaultNodePool:
///         name: default
///         nodeCount: 3
///         vmSize: Standard_D3_v2
///         vnetSubnetId: ${exampleSubnet.id}
///       identity:
///         type: SystemAssigned
///   exampleInferenceCluster:
///     type: azure:machinelearning:InferenceCluster
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       clusterPurpose: FastProd
///       kubernetesClusterId: ${exampleKubernetesCluster.id}
///       description: This is an example cluster used with Terraform
///       machineLearningWorkspaceId: ${exampleWorkspace.id}
///       tags:
///         stage: example
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Machine Learning Inference Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:machinelearning/inferenceCluster:InferenceCluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.MachineLearningServices/workspaces/workspace1/computes/cluster1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod inference_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InferenceClusterArgs {
        /// The purpose of the Inference Cluster. Options are `DevTest`, `DenseProd` and `FastProd`. If used for Development or Testing, use `DevTest` here. Default purpose is `FastProd`, which is recommended for production workloads. Changing this forces a new Machine Learning Inference Cluster to be created.
        ///
        /// > **NOTE:** When creating or attaching a cluster, if the cluster will be used for production (`cluster_purpose = "FastProd"`), then it must contain at least 12 virtual CPUs. The number of virtual CPUs can be calculated by multiplying the number of nodes in the cluster by the number of cores provided by the VM size selected. For example, if you use a VM size of "Standard_D3_v2", which has 4 virtual cores, then you should select 3 or greater as the number of nodes.
        #[builder(into, default)]
        pub cluster_purpose: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description of the Machine Learning Inference Cluster. Changing this forces a new Machine Learning Inference Cluster to be created.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below. Changing this forces a new Machine Learning Inference Cluster to be created.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::machinelearning::InferenceClusterIdentity>,
        >,
        /// The ID of the Kubernetes Cluster. Changing this forces a new Machine Learning Inference Cluster to be created.
        #[builder(into)]
        pub kubernetes_cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region where the Machine Learning Inference Cluster should exist. Changing this forces a new Machine Learning Inference Cluster to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Machine Learning Workspace. Changing this forces a new Machine Learning Inference Cluster to be created.
        #[builder(into)]
        pub machine_learning_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Machine Learning Inference Cluster. Changing this forces a new Machine Learning Inference Cluster to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `ssl` block as defined below. Changing this forces a new Machine Learning Inference Cluster to be created.
        #[builder(into, default)]
        pub ssl: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::machinelearning::InferenceClusterSsl>,
        >,
        /// A mapping of tags which should be assigned to the Machine Learning Inference Cluster. Changing this forces a new Machine Learning Inference Cluster to be created.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct InferenceClusterResult {
        /// The purpose of the Inference Cluster. Options are `DevTest`, `DenseProd` and `FastProd`. If used for Development or Testing, use `DevTest` here. Default purpose is `FastProd`, which is recommended for production workloads. Changing this forces a new Machine Learning Inference Cluster to be created.
        ///
        /// > **NOTE:** When creating or attaching a cluster, if the cluster will be used for production (`cluster_purpose = "FastProd"`), then it must contain at least 12 virtual CPUs. The number of virtual CPUs can be calculated by multiplying the number of nodes in the cluster by the number of cores provided by the VM size selected. For example, if you use a VM size of "Standard_D3_v2", which has 4 virtual cores, then you should select 3 or greater as the number of nodes.
        pub cluster_purpose: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description of the Machine Learning Inference Cluster. Changing this forces a new Machine Learning Inference Cluster to be created.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identity` block as defined below. Changing this forces a new Machine Learning Inference Cluster to be created.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::machinelearning::InferenceClusterIdentity>,
        >,
        /// The ID of the Kubernetes Cluster. Changing this forces a new Machine Learning Inference Cluster to be created.
        pub kubernetes_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Machine Learning Inference Cluster should exist. Changing this forces a new Machine Learning Inference Cluster to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Machine Learning Workspace. Changing this forces a new Machine Learning Inference Cluster to be created.
        pub machine_learning_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Machine Learning Inference Cluster. Changing this forces a new Machine Learning Inference Cluster to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `ssl` block as defined below. Changing this forces a new Machine Learning Inference Cluster to be created.
        pub ssl: pulumi_gestalt_rust::Output<
            Option<super::super::types::machinelearning::InferenceClusterSsl>,
        >,
        /// A mapping of tags which should be assigned to the Machine Learning Inference Cluster. Changing this forces a new Machine Learning Inference Cluster to be created.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InferenceClusterArgs,
    ) -> InferenceClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_purpose_binding = args.cluster_purpose.get_output(context);
        let description_binding = args.description.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let kubernetes_cluster_id_binding = args
            .kubernetes_cluster_id
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let machine_learning_workspace_id_binding = args
            .machine_learning_workspace_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let ssl_binding = args.ssl.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:machinelearning/inferenceCluster:InferenceCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterPurpose".into(),
                    value: &cluster_purpose_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kubernetesClusterId".into(),
                    value: &kubernetes_cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "machineLearningWorkspaceId".into(),
                    value: &machine_learning_workspace_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ssl".into(),
                    value: &ssl_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InferenceClusterResult {
            cluster_purpose: o.get_field("clusterPurpose"),
            description: o.get_field("description"),
            identity: o.get_field("identity"),
            kubernetes_cluster_id: o.get_field("kubernetesClusterId"),
            location: o.get_field("location"),
            machine_learning_workspace_id: o.get_field("machineLearningWorkspaceId"),
            name: o.get_field("name"),
            ssl: o.get_field("ssl"),
            tags: o.get_field("tags"),
        }
    }
}
