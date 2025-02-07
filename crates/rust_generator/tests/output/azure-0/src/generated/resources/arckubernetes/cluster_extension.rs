/// Manages an Arc Kubernetes Cluster Extension.
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
///   exampleClusterExtension:
///     type: azure:arckubernetes:ClusterExtension
///     name: example
///     properties:
///       name: example-ext
///       clusterId: ${exampleCluster.id}
///       extensionType: microsoft.flux
/// ```
///
/// ## Import
///
/// Arc Kubernetes Cluster Extension can be imported using the `resource id` for different `cluster_resource_name`, e.g.
///
/// ```sh
/// $ pulumi import azure:arckubernetes/clusterExtension:ClusterExtension example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Kubernetes/connectedClusters/cluster1/providers/Microsoft.KubernetesConfiguration/extensions/extension1
/// ```
///
pub mod cluster_extension {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterExtensionArgs {
        /// Specifies the Cluster ID. Changing this forces a new Arc Kubernetes Cluster Extension to be created.
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
        /// Specifies the type of extension. It must be one of the extension types registered with Microsoft.KubernetesConfiguration by the Extension publisher. For more information, please refer to [Available Extensions for Arc-enabled Kubernetes clusters](https://learn.microsoft.com/en-us/azure/azure-arc/kubernetes/extensions-release). Changing this forces a new Arc Kubernetes Cluster Extension to be created.
        #[builder(into)]
        pub extension_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `identity` block as defined below. Changing this forces a new Arc Kubernetes Cluster Extension to be created.
        #[builder(into)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::arckubernetes::ClusterExtensionIdentity,
        >,
        /// Specifies the name which should be used for this Arc Kubernetes Cluster Extension. Changing this forces a new Arc Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Namespace where the extension release must be placed for a cluster scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Arc Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub release_namespace: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The release train used by this extension. Possible values include but are not limited to `Stable`, `Preview`. Changing this forces a new Arc Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub release_train: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Namespace where the extension will be created for a namespace scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Arc Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub target_namespace: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-specified version that the extension should pin to. If it is not set, Azure will use the latest version and auto upgrade it. Changing this forces a new Arc Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ClusterExtensionResult {
        /// Specifies the Cluster ID. Changing this forces a new Arc Kubernetes Cluster Extension to be created.
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
        /// Specifies the type of extension. It must be one of the extension types registered with Microsoft.KubernetesConfiguration by the Extension publisher. For more information, please refer to [Available Extensions for Arc-enabled Kubernetes clusters](https://learn.microsoft.com/en-us/azure/azure-arc/kubernetes/extensions-release). Changing this forces a new Arc Kubernetes Cluster Extension to be created.
        pub extension_type: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below. Changing this forces a new Arc Kubernetes Cluster Extension to be created.
        pub identity: pulumi_gestalt_rust::Output<
            super::super::types::arckubernetes::ClusterExtensionIdentity,
        >,
        /// Specifies the name which should be used for this Arc Kubernetes Cluster Extension. Changing this forces a new Arc Kubernetes Cluster Extension to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Namespace where the extension release must be placed for a cluster scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Arc Kubernetes Cluster Extension to be created.
        pub release_namespace: pulumi_gestalt_rust::Output<String>,
        /// The release train used by this extension. Possible values include but are not limited to `Stable`, `Preview`. Changing this forces a new Arc Kubernetes Cluster Extension to be created.
        pub release_train: pulumi_gestalt_rust::Output<String>,
        /// Namespace where the extension will be created for a namespace scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Arc Kubernetes Cluster Extension to be created.
        pub target_namespace: pulumi_gestalt_rust::Output<String>,
        /// User-specified version that the extension should pin to. If it is not set, Azure will use the latest version and auto upgrade it. Changing this forces a new Arc Kubernetes Cluster Extension to be created.
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ClusterExtensionArgs,
    ) -> ClusterExtensionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_id_binding = args.cluster_id.get_output(context).get_inner();
        let configuration_protected_settings_binding = args
            .configuration_protected_settings
            .get_output(context)
            .get_inner();
        let configuration_settings_binding = args
            .configuration_settings
            .get_output(context)
            .get_inner();
        let extension_type_binding = args.extension_type.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let release_namespace_binding = args
            .release_namespace
            .get_output(context)
            .get_inner();
        let release_train_binding = args.release_train.get_output(context).get_inner();
        let target_namespace_binding = args
            .target_namespace
            .get_output(context)
            .get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:arckubernetes/clusterExtension:ClusterExtension".into(),
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
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
        ClusterExtensionResult {
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
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
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
