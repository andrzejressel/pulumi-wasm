/// Manages a Kubernetes Flux Configuration.
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
///       clusterId: ${test.id}
///       extensionType: microsoft.flux
///   exampleFluxConfiguration:
///     type: azure:containerservice:FluxConfiguration
///     name: example
///     properties:
///       name: example-fc
///       clusterId: ${test.id}
///       namespace: flux
///       gitRepository:
///         url: https://github.com/Azure/arc-k8s-demo
///         referenceType: branch
///         referenceValue: main
///       kustomizations:
///         - name: kustomization-1
///     options:
///       dependsOn:
///         - ${exampleKubernetesClusterExtension}
/// ```
///
/// ## Import
///
/// Kubernetes Flux Configuration can be imported using the `resource id` for different `cluster_resource_name`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/fluxConfiguration:FluxConfiguration example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.ContainerService/managedClusters/cluster1/providers/Microsoft.KubernetesConfiguration/fluxConfigurations/fluxConfiguration1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod flux_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FluxConfigurationArgs {
        /// An `blob_storage` block as defined below.
        #[builder(into, default)]
        pub blob_storage: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::FluxConfigurationBlobStorage>,
        >,
        /// A `bucket` block as defined below.
        #[builder(into, default)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::FluxConfigurationBucket>,
        >,
        /// Specifies the Cluster ID. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into)]
        pub cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the configuration will keep its reconciliation of its kustomizations and sources with the repository. Defaults to `true`.
        #[builder(into, default)]
        pub continuous_reconciliation_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A `git_repository` block as defined below.
        #[builder(into, default)]
        pub git_repository: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::FluxConfigurationGitRepository>,
        >,
        /// A `kustomizations` block as defined below.
        #[builder(into)]
        pub kustomizations: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::containerservice::FluxConfigurationKustomization>,
        >,
        /// Specifies the name which should be used for this Kubernetes Flux Configuration. Changing this forces a new Kubernetes Flux Configuration to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the namespace to which this configuration is installed to. Changing this forces a new Kubernetes Flux Configuration to be created.
        #[builder(into)]
        pub namespace: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the scope at which the operator will be installed. Possible values are `cluster` and `namespace`. Defaults to `namespace`. Changing this forces a new Kubernetes Flux Configuration to be created.
        #[builder(into, default)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FluxConfigurationResult {
        /// An `blob_storage` block as defined below.
        pub blob_storage: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::FluxConfigurationBlobStorage>,
        >,
        /// A `bucket` block as defined below.
        pub bucket: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::FluxConfigurationBucket>,
        >,
        /// Specifies the Cluster ID. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// Whether the configuration will keep its reconciliation of its kustomizations and sources with the repository. Defaults to `true`.
        pub continuous_reconciliation_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `git_repository` block as defined below.
        pub git_repository: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::FluxConfigurationGitRepository>,
        >,
        /// A `kustomizations` block as defined below.
        pub kustomizations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::containerservice::FluxConfigurationKustomization>,
        >,
        /// Specifies the name which should be used for this Kubernetes Flux Configuration. Changing this forces a new Kubernetes Flux Configuration to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the namespace to which this configuration is installed to. Changing this forces a new Kubernetes Flux Configuration to be created.
        pub namespace: pulumi_gestalt_rust::Output<String>,
        /// Specifies the scope at which the operator will be installed. Possible values are `cluster` and `namespace`. Defaults to `namespace`. Changing this forces a new Kubernetes Flux Configuration to be created.
        pub scope: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FluxConfigurationArgs,
    ) -> FluxConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let blob_storage_binding = args.blob_storage.get_output(context);
        let bucket_binding = args.bucket.get_output(context);
        let cluster_id_binding = args.cluster_id.get_output(context);
        let continuous_reconciliation_enabled_binding = args
            .continuous_reconciliation_enabled
            .get_output(context);
        let git_repository_binding = args.git_repository.get_output(context);
        let kustomizations_binding = args.kustomizations.get_output(context);
        let name_binding = args.name.get_output(context);
        let namespace_binding = args.namespace.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerservice/fluxConfiguration:FluxConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blobStorage".into(),
                    value: &blob_storage_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "continuousReconciliationEnabled".into(),
                    value: &continuous_reconciliation_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gitRepository".into(),
                    value: &git_repository_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kustomizations".into(),
                    value: &kustomizations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: &scope_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FluxConfigurationResult {
            blob_storage: o.get_field("blobStorage"),
            bucket: o.get_field("bucket"),
            cluster_id: o.get_field("clusterId"),
            continuous_reconciliation_enabled: o
                .get_field("continuousReconciliationEnabled"),
            git_repository: o.get_field("gitRepository"),
            kustomizations: o.get_field("kustomizations"),
            name: o.get_field("name"),
            namespace: o.get_field("namespace"),
            scope: o.get_field("scope"),
        }
    }
}
