/// <!-- Note: This documentation is generated. Any manual changes will be overwritten -->
///
/// Manages a Kubernetes Cluster Trusted Access Role Binding
/// > **Note:** This Resource is in **Preview** to use this you must be opted into the Preview. You can do this by running `az feature register --namespace Microsoft.ContainerService --name TrustedAccessPreview` and then `az provider register -n Microsoft.ContainerService`
/// .
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:appinsights:Insights
///     properties:
///       name: example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       applicationType: example-value
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       tenantId: ${exampleAzurermClientConfig.tenantId}
///       skuName: example-value
///       softDeleteRetentionDays: example-value
///   exampleAccessPolicy:
///     type: azure:keyvault:AccessPolicy
///     name: example
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${exampleAzurermClientConfig.tenantId}
///       objectId: ${exampleAzurermClientConfig.objectId}
///       keyPermissions: example-value
///   exampleKubernetesCluster:
///     type: azure:containerservice:KubernetesCluster
///     name: example
///     properties:
///       name: example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       dnsPrefix: acctestaksexample
///       defaultNodePool:
///         name: example-value
///         nodeCount: example-value
///         vmSize: example-value
///         upgradeSettings:
///           maxSurge: example-value
///       identity:
///         type: example-value
///   exampleWorkspace:
///     type: azure:machinelearning:Workspace
///     name: example
///     properties:
///       name: example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       keyVaultId: ${exampleKeyVault.id}
///       storageAccountId: ${exampleAccount.id}
///       applicationInsightsId: ${example.id}
///       identity:
///         type: example-value
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       accountTier: example-value
///       accountReplicationType: example-value
///   exampleClusterTrustedAccessRoleBinding:
///     type: azure:containerservice:ClusterTrustedAccessRoleBinding
///     name: example
///     properties:
///       kubernetesClusterId: ${exampleKubernetesCluster.id}
///       name: example
///       roles: example-value
///       sourceResourceId: ${exampleWorkspace.id}
/// variables:
///   test:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// An existing Kubernetes Cluster Trusted Access Role Binding can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/clusterTrustedAccessRoleBinding:ClusterTrustedAccessRoleBinding example /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerService/managedClusters/{managedClusterName}/trustedAccessRoleBindings/{trustedAccessRoleBindingName}
/// ```
///
/// * Where `{subscriptionId}` is the ID of the Azure Subscription where the Kubernetes Cluster Trusted Access Role Binding exists. For example `12345678-1234-9876-4563-123456789012`.
///
/// * Where `{resourceGroupName}` is the name of Resource Group where this Kubernetes Cluster Trusted Access Role Binding exists. For example `example-resource-group`.
///
/// * Where `{managedClusterName}` is the name of the Managed Cluster. For example `managedClusterValue`.
///
/// * Where `{trustedAccessRoleBindingName}` is the name of the Trusted Access Role Binding. For example `trustedAccessRoleBindingValue`.
///
pub mod cluster_trusted_access_role_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterTrustedAccessRoleBindingArgs {
        /// Specifies the Kubernetes Cluster Id within which this Kubernetes Cluster Trusted Access Role Binding should exist. Changing this forces a new Kubernetes Cluster Trusted Access Role Binding to be created.
        #[builder(into)]
        pub kubernetes_cluster_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Kubernetes Cluster Trusted Access Role Binding. Changing this forces a new Kubernetes Cluster Trusted Access Role Binding to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of roles to bind, each item is a resource type qualified role name.
        #[builder(into)]
        pub roles: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ARM resource ID of source resource that trusted access is configured for. Changing this forces a new Kubernetes Cluster Trusted Access Role Binding to be created.
        #[builder(into)]
        pub source_resource_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ClusterTrustedAccessRoleBindingResult {
        /// Specifies the Kubernetes Cluster Id within which this Kubernetes Cluster Trusted Access Role Binding should exist. Changing this forces a new Kubernetes Cluster Trusted Access Role Binding to be created.
        pub kubernetes_cluster_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Kubernetes Cluster Trusted Access Role Binding. Changing this forces a new Kubernetes Cluster Trusted Access Role Binding to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of roles to bind, each item is a resource type qualified role name.
        pub roles: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ARM resource ID of source resource that trusted access is configured for. Changing this forces a new Kubernetes Cluster Trusted Access Role Binding to be created.
        pub source_resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ClusterTrustedAccessRoleBindingArgs,
    ) -> ClusterTrustedAccessRoleBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let kubernetes_cluster_id_binding = args.kubernetes_cluster_id.get_inner();
        let name_binding = args.name.get_inner();
        let roles_binding = args.roles.get_inner();
        let source_resource_id_binding = args.source_resource_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/clusterTrustedAccessRoleBinding:ClusterTrustedAccessRoleBinding"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "kubernetesClusterId".into(),
                    value: &kubernetes_cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "roles".into(),
                    value: &roles_binding,
                },
                register_interface::ObjectField {
                    name: "sourceResourceId".into(),
                    value: &source_resource_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "kubernetesClusterId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "roles".into(),
                },
                register_interface::ResultField {
                    name: "sourceResourceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterTrustedAccessRoleBindingResult {
            kubernetes_cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubernetesClusterId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            roles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roles").unwrap(),
            ),
            source_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceResourceId").unwrap(),
            ),
        }
    }
}