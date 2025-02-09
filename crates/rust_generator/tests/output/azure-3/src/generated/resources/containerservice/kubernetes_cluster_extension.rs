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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod kubernetes_cluster_extension {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KubernetesClusterExtensionArgs {
        /// Specifies the Cluster ID. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into)]
        pub cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration settings that are sensitive, as name-value pairs for configuring this extension.
        #[builder(into, default)]
        pub configuration_protected_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration settings, as name-value pairs for configuring this extension.
        #[builder(into, default)]
        pub configuration_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the type of extension. It must be one of the extension types registered with Microsoft.KubernetesConfiguration by the Extension publisher. For more information, please refer to [Available Extensions for AKS](https://learn.microsoft.com/en-us/azure/aks/cluster-extensions?tabs=azure-cli#currently-available-extensions). Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into)]
        pub extension_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name which should be used for this Kubernetes Cluster Extension. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `plan` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::KubernetesClusterExtensionPlan>,
        >,
        /// Namespace where the extension release must be placed for a cluster scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub release_namespace: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The release train used by this extension. Possible values include but are not limited to `Stable`, `Preview`. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub release_train: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Namespace where the extension will be created for a namespace scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub target_namespace: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-specified version that the extension should pin to. If it is not set, Azure will use the latest version and auto upgrade it. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KubernetesClusterExtensionResult {
        /// An `aks_assigned_identity` block as defined below.
        pub aks_assigned_identities: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::containerservice::KubernetesClusterExtensionAksAssignedIdentity,
            >,
        >,
        /// Specifies the Cluster ID. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// Configuration settings that are sensitive, as name-value pairs for configuring this extension.
        pub configuration_protected_settings: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration settings, as name-value pairs for configuring this extension.
        pub configuration_settings: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The current version of the extension.
        pub current_version: pulumi_gestalt_rust::Output<String>,
        /// Specifies the type of extension. It must be one of the extension types registered with Microsoft.KubernetesConfiguration by the Extension publisher. For more information, please refer to [Available Extensions for AKS](https://learn.microsoft.com/en-us/azure/aks/cluster-extensions?tabs=azure-cli#currently-available-extensions). Changing this forces a new Kubernetes Cluster Extension to be created.
        pub extension_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Kubernetes Cluster Extension. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `plan` block as defined below. Changing this forces a new resource to be created.
        pub plan: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::KubernetesClusterExtensionPlan>,
        >,
        /// Namespace where the extension release must be placed for a cluster scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub release_namespace: pulumi_gestalt_rust::Output<String>,
        /// The release train used by this extension. Possible values include but are not limited to `Stable`, `Preview`. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub release_train: pulumi_gestalt_rust::Output<String>,
        /// Namespace where the extension will be created for a namespace scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub target_namespace: pulumi_gestalt_rust::Output<String>,
        /// User-specified version that the extension should pin to. If it is not set, Azure will use the latest version and auto upgrade it. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: KubernetesClusterExtensionArgs,
    ) -> KubernetesClusterExtensionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_id_binding_1 = args.cluster_id.get_output(context);
        let cluster_id_binding = cluster_id_binding_1.get_inner();
        let configuration_protected_settings_binding_1 = args
            .configuration_protected_settings
            .get_output(context);
        let configuration_protected_settings_binding = configuration_protected_settings_binding_1
            .get_inner();
        let configuration_settings_binding_1 = args
            .configuration_settings
            .get_output(context);
        let configuration_settings_binding = configuration_settings_binding_1
            .get_inner();
        let extension_type_binding_1 = args.extension_type.get_output(context);
        let extension_type_binding = extension_type_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let plan_binding_1 = args.plan.get_output(context);
        let plan_binding = plan_binding_1.get_inner();
        let release_namespace_binding_1 = args.release_namespace.get_output(context);
        let release_namespace_binding = release_namespace_binding_1.get_inner();
        let release_train_binding_1 = args.release_train.get_output(context);
        let release_train_binding = release_train_binding_1.get_inner();
        let target_namespace_binding_1 = args.target_namespace.get_output(context);
        let target_namespace_binding = target_namespace_binding_1.get_inner();
        let version_binding_1 = args.version.get_output(context);
        let version_binding = version_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/kubernetesClusterExtension:KubernetesClusterExtension"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        KubernetesClusterExtensionResult {
            aks_assigned_identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("aksAssignedIdentities"),
            ),
            cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterId"),
            ),
            configuration_protected_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurationProtectedSettings"),
            ),
            configuration_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurationSettings"),
            ),
            current_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("currentVersion"),
            ),
            extension_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("extensionType"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            plan: pulumi_gestalt_rust::__private::into_domain(o.extract_field("plan")),
            release_namespace: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("releaseNamespace"),
            ),
            release_train: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("releaseTrain"),
            ),
            target_namespace: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetNamespace"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
