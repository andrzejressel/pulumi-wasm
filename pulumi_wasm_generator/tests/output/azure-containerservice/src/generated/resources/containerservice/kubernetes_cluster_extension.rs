/// Manages a Kubernetes Cluster Extension.
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
///   exampleKubernetesCluster:
///     type: azure:containerservice:KubernetesCluster
///     name: example
///     properties:
///       name: example-aks
///       location: West Europe
///       resourceGroupName: ${example.name}
///       dnsPrefix: example-aks
///       defaultNodePool:
///         name: default
///         nodeCount: 1
///         vmSize: Standard_DS2_v2
///       identity:
///         type: SystemAssigned
///   exampleKubernetesClusterExtension:
///     type: azure:containerservice:KubernetesClusterExtension
///     name: example
///     properties:
///       name: example-ext
///       clusterId: ${exampleKubernetesCluster.id}
///       extensionType: microsoft.flux
/// ```
///
/// ## Import
///
/// Kubernetes Cluster Extension can be imported using the `resource id` for different `cluster_resource_name`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/kubernetesClusterExtension:KubernetesClusterExtension example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.ContainerService/managedClusters/cluster1/providers/Microsoft.KubernetesConfiguration/extensions/extension1
/// ```
///
pub mod kubernetes_cluster_extension {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KubernetesClusterExtensionArgs {
        /// Specifies the Cluster ID. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into)]
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// Configuration settings that are sensitive, as name-value pairs for configuring this extension.
        #[builder(into, default)]
        pub configuration_protected_settings: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration settings, as name-value pairs for configuring this extension.
        #[builder(into, default)]
        pub configuration_settings: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the type of extension. It must be one of the extension types registered with Microsoft.KubernetesConfiguration by the Extension publisher. For more information, please refer to [Available Extensions for AKS](https://learn.microsoft.com/en-us/azure/aks/cluster-extensions?tabs=azure-cli#currently-available-extensions). Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into)]
        pub extension_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Kubernetes Cluster Extension. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `plan` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub plan: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::KubernetesClusterExtensionPlan>,
        >,
        /// Namespace where the extension release must be placed for a cluster scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub release_namespace: pulumi_wasm_rust::Output<Option<String>>,
        /// The release train used by this extension. Possible values include but are not limited to `Stable`, `Preview`. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub release_train: pulumi_wasm_rust::Output<Option<String>>,
        /// Namespace where the extension will be created for a namespace scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub target_namespace: pulumi_wasm_rust::Output<Option<String>>,
        /// User-specified version that the extension should pin to. If it is not set, Azure will use the latest version and auto upgrade it. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KubernetesClusterExtensionResult {
        /// An `aks_assigned_identity` block as defined below.
        pub aks_assigned_identities: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::containerservice::KubernetesClusterExtensionAksAssignedIdentity,
            >,
        >,
        /// Specifies the Cluster ID. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// Configuration settings that are sensitive, as name-value pairs for configuring this extension.
        pub configuration_protected_settings: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration settings, as name-value pairs for configuring this extension.
        pub configuration_settings: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The current version of the extension.
        pub current_version: pulumi_wasm_rust::Output<String>,
        /// Specifies the type of extension. It must be one of the extension types registered with Microsoft.KubernetesConfiguration by the Extension publisher. For more information, please refer to [Available Extensions for AKS](https://learn.microsoft.com/en-us/azure/aks/cluster-extensions?tabs=azure-cli#currently-available-extensions). Changing this forces a new Kubernetes Cluster Extension to be created.
        pub extension_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Kubernetes Cluster Extension. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `plan` block as defined below. Changing this forces a new resource to be created.
        pub plan: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::KubernetesClusterExtensionPlan>,
        >,
        /// Namespace where the extension release must be placed for a cluster scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub release_namespace: pulumi_wasm_rust::Output<String>,
        /// The release train used by this extension. Possible values include but are not limited to `Stable`, `Preview`. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub release_train: pulumi_wasm_rust::Output<String>,
        /// Namespace where the extension will be created for a namespace scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub target_namespace: pulumi_wasm_rust::Output<String>,
        /// User-specified version that the extension should pin to. If it is not set, Azure will use the latest version and auto upgrade it. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: KubernetesClusterExtensionArgs,
    ) -> KubernetesClusterExtensionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_id_binding = args.cluster_id.get_inner();
        let configuration_protected_settings_binding = args
            .configuration_protected_settings
            .get_inner();
        let configuration_settings_binding = args.configuration_settings.get_inner();
        let extension_type_binding = args.extension_type.get_inner();
        let name_binding = args.name.get_inner();
        let plan_binding = args.plan.get_inner();
        let release_namespace_binding = args.release_namespace.get_inner();
        let release_train_binding = args.release_train.get_inner();
        let target_namespace_binding = args.target_namespace.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/kubernetesClusterExtension:KubernetesClusterExtension"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "configurationProtectedSettings".into(),
                    value: &configuration_protected_settings_binding,
                },
                register_interface::ObjectField {
                    name: "configurationSettings".into(),
                    value: &configuration_settings_binding,
                },
                register_interface::ObjectField {
                    name: "extensionType".into(),
                    value: &extension_type_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "plan".into(),
                    value: &plan_binding,
                },
                register_interface::ObjectField {
                    name: "releaseNamespace".into(),
                    value: &release_namespace_binding,
                },
                register_interface::ObjectField {
                    name: "releaseTrain".into(),
                    value: &release_train_binding,
                },
                register_interface::ObjectField {
                    name: "targetNamespace".into(),
                    value: &target_namespace_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "aksAssignedIdentities".into(),
                },
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "configurationProtectedSettings".into(),
                },
                register_interface::ResultField {
                    name: "configurationSettings".into(),
                },
                register_interface::ResultField {
                    name: "currentVersion".into(),
                },
                register_interface::ResultField {
                    name: "extensionType".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "plan".into(),
                },
                register_interface::ResultField {
                    name: "releaseNamespace".into(),
                },
                register_interface::ResultField {
                    name: "releaseTrain".into(),
                },
                register_interface::ResultField {
                    name: "targetNamespace".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        KubernetesClusterExtensionResult {
            aks_assigned_identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aksAssignedIdentities").unwrap(),
            ),
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            configuration_protected_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationProtectedSettings").unwrap(),
            ),
            configuration_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationSettings").unwrap(),
            ),
            current_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("currentVersion").unwrap(),
            ),
            extension_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extensionType").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            plan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("plan").unwrap(),
            ),
            release_namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseNamespace").unwrap(),
            ),
            release_train: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseTrain").unwrap(),
            ),
            target_namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetNamespace").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
