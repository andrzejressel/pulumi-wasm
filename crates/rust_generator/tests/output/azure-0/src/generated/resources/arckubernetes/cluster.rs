/// Manages an Arc Kubernetes Cluster.
///
/// > **Note:** Installing and configuring the Azure Arc Agent on your Kubernetes Cluster to establish connectivity is outside the scope of this document. For more details refer to [Deploy agents to your cluster](https://learn.microsoft.com/en-us/azure/azure-arc/kubernetes/conceptual-agent-overview#deploy-agents-to-your-cluster) and [Connect an existing Kubernetes Cluster](https://learn.microsoft.com/en-us/azure/azure-arc/kubernetes/quickstart-connect-cluster?tabs=azure-cli#connect-an-existing-kubernetes-cluster). If you encounter issues connecting your Kubernetes Cluster to Azure Arc, we'd recommend opening a ticket with Microsoft Support.
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
///   exampleCluster:
///     type: azure:arckubernetes:Cluster
///     name: example
///     properties:
///       name: example-akcc
///       resourceGroupName: ${example.name}
///       location: West Europe
///       agentPublicKeyCertificate:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: testdata/public.cer
///           return: result
///       identity:
///         type: SystemAssigned
///       tags:
///         ENV: Test
/// ```
///
/// > **Note:** An extensive example on connecting the `azure.arckubernetes.Cluster` to an external kubernetes cluster can be found in the `./examples/arckubernetes` directory within the GitHub Repository.
///
/// ## Import
///
/// Arc Kubernetes Cluster can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:arckubernetes/cluster:Cluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Kubernetes/connectedClusters/cluster1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// Specifies the base64-encoded public certificate used by the agent to do the initial handshake to the backend services in Azure. Changing this forces a new Arc Kubernetes Cluster to be created.
        #[builder(into)]
        pub agent_public_key_certificate: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `identity` block as defined below. Changing this forces a new Arc Kubernetes Cluster to be created.
        #[builder(into)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::arckubernetes::ClusterIdentity,
        >,
        /// Specifies the Azure Region where the Arc Kubernetes Cluster should exist. Changing this forces a new Arc Kubernetes Cluster to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Arc Kubernetes Cluster. Changing this forces a new Arc Kubernetes Cluster to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group where the Arc Kubernetes Cluster should exist. Changing this forces a new Arc Kubernetes Cluster to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Arc Kubernetes Cluster.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// Specifies the base64-encoded public certificate used by the agent to do the initial handshake to the backend services in Azure. Changing this forces a new Arc Kubernetes Cluster to be created.
        pub agent_public_key_certificate: pulumi_gestalt_rust::Output<String>,
        /// Version of the agent running on the cluster resource.
        pub agent_version: pulumi_gestalt_rust::Output<String>,
        /// The distribution running on this Arc Kubernetes Cluster.
        pub distribution: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below. Changing this forces a new Arc Kubernetes Cluster to be created.
        pub identity: pulumi_gestalt_rust::Output<
            super::super::types::arckubernetes::ClusterIdentity,
        >,
        /// The infrastructure on which the Arc Kubernetes Cluster is running on.
        pub infrastructure: pulumi_gestalt_rust::Output<String>,
        /// The Kubernetes version of the cluster resource.
        pub kubernetes_version: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Azure Region where the Arc Kubernetes Cluster should exist. Changing this forces a new Arc Kubernetes Cluster to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Arc Kubernetes Cluster. Changing this forces a new Arc Kubernetes Cluster to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The cluster offering.
        pub offering: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Arc Kubernetes Cluster should exist. Changing this forces a new Arc Kubernetes Cluster to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Arc Kubernetes Cluster.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Number of CPU cores present in the cluster resource.
        pub total_core_count: pulumi_gestalt_rust::Output<i32>,
        /// Number of nodes present in the cluster resource.
        pub total_node_count: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let agent_public_key_certificate_binding = args
            .agent_public_key_certificate
            .get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:arckubernetes/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentPublicKeyCertificate".into(),
                    value: &agent_public_key_certificate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            agent_public_key_certificate: o.get_field("agentPublicKeyCertificate"),
            agent_version: o.get_field("agentVersion"),
            distribution: o.get_field("distribution"),
            identity: o.get_field("identity"),
            infrastructure: o.get_field("infrastructure"),
            kubernetes_version: o.get_field("kubernetesVersion"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            offering: o.get_field("offering"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            total_core_count: o.get_field("totalCoreCount"),
            total_node_count: o.get_field("totalNodeCount"),
        }
    }
}
