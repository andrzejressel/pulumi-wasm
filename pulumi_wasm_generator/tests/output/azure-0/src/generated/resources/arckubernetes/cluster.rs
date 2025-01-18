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
pub mod cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// Specifies the base64-encoded public certificate used by the agent to do the initial handshake to the backend services in Azure. Changing this forces a new Arc Kubernetes Cluster to be created.
        #[builder(into)]
        pub agent_public_key_certificate: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below. Changing this forces a new Arc Kubernetes Cluster to be created.
        #[builder(into)]
        pub identity: pulumi_wasm_rust::Output<
            super::super::types::arckubernetes::ClusterIdentity,
        >,
        /// Specifies the Azure Region where the Arc Kubernetes Cluster should exist. Changing this forces a new Arc Kubernetes Cluster to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Arc Kubernetes Cluster. Changing this forces a new Arc Kubernetes Cluster to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Resource Group where the Arc Kubernetes Cluster should exist. Changing this forces a new Arc Kubernetes Cluster to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Arc Kubernetes Cluster.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// Specifies the base64-encoded public certificate used by the agent to do the initial handshake to the backend services in Azure. Changing this forces a new Arc Kubernetes Cluster to be created.
        pub agent_public_key_certificate: pulumi_wasm_rust::Output<String>,
        /// Version of the agent running on the cluster resource.
        pub agent_version: pulumi_wasm_rust::Output<String>,
        /// The distribution running on this Arc Kubernetes Cluster.
        pub distribution: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below. Changing this forces a new Arc Kubernetes Cluster to be created.
        pub identity: pulumi_wasm_rust::Output<
            super::super::types::arckubernetes::ClusterIdentity,
        >,
        /// The infrastructure on which the Arc Kubernetes Cluster is running on.
        pub infrastructure: pulumi_wasm_rust::Output<String>,
        /// The Kubernetes version of the cluster resource.
        pub kubernetes_version: pulumi_wasm_rust::Output<String>,
        /// Specifies the Azure Region where the Arc Kubernetes Cluster should exist. Changing this forces a new Arc Kubernetes Cluster to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Arc Kubernetes Cluster. Changing this forces a new Arc Kubernetes Cluster to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The cluster offering.
        pub offering: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Arc Kubernetes Cluster should exist. Changing this forces a new Arc Kubernetes Cluster to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Arc Kubernetes Cluster.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Number of CPU cores present in the cluster resource.
        pub total_core_count: pulumi_wasm_rust::Output<i32>,
        /// Number of nodes present in the cluster resource.
        pub total_node_count: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ClusterArgs) -> ClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let agent_public_key_certificate_binding = args
            .agent_public_key_certificate
            .get_inner();
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:arckubernetes/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "agentPublicKeyCertificate".into(),
                    value: &agent_public_key_certificate_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "agentPublicKeyCertificate".into(),
                },
                register_interface::ResultField {
                    name: "agentVersion".into(),
                },
                register_interface::ResultField {
                    name: "distribution".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "infrastructure".into(),
                },
                register_interface::ResultField {
                    name: "kubernetesVersion".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "offering".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "totalCoreCount".into(),
                },
                register_interface::ResultField {
                    name: "totalNodeCount".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterResult {
            agent_public_key_certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentPublicKeyCertificate").unwrap(),
            ),
            agent_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentVersion").unwrap(),
            ),
            distribution: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("distribution").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            infrastructure: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("infrastructure").unwrap(),
            ),
            kubernetes_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubernetesVersion").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            offering: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("offering").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            total_core_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalCoreCount").unwrap(),
            ),
            total_node_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalNodeCount").unwrap(),
            ),
        }
    }
}
