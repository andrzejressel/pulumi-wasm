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
pub mod flux_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FluxConfigurationArgs {
        /// An `blob_storage` block as defined below.
        #[builder(into, default)]
        pub blob_storage: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::FluxConfigurationBlobStorage>,
        >,
        /// A `bucket` block as defined below.
        #[builder(into, default)]
        pub bucket: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::FluxConfigurationBucket>,
        >,
        /// Specifies the Cluster ID. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into)]
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// Whether the configuration will keep its reconciliation of its kustomizations and sources with the repository. Defaults to `true`.
        #[builder(into, default)]
        pub continuous_reconciliation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `git_repository` block as defined below.
        #[builder(into, default)]
        pub git_repository: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::FluxConfigurationGitRepository>,
        >,
        /// A `kustomizations` block as defined below.
        #[builder(into)]
        pub kustomizations: pulumi_wasm_rust::Output<
            Vec<super::super::types::containerservice::FluxConfigurationKustomization>,
        >,
        /// Specifies the name which should be used for this Kubernetes Flux Configuration. Changing this forces a new Kubernetes Flux Configuration to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the namespace to which this configuration is installed to. Changing this forces a new Kubernetes Flux Configuration to be created.
        #[builder(into)]
        pub namespace: pulumi_wasm_rust::Output<String>,
        /// Specifies the scope at which the operator will be installed. Possible values are `cluster` and `namespace`. Defaults to `namespace`. Changing this forces a new Kubernetes Flux Configuration to be created.
        #[builder(into, default)]
        pub scope: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FluxConfigurationResult {
        /// An `blob_storage` block as defined below.
        pub blob_storage: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::FluxConfigurationBlobStorage>,
        >,
        /// A `bucket` block as defined below.
        pub bucket: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::FluxConfigurationBucket>,
        >,
        /// Specifies the Cluster ID. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// Whether the configuration will keep its reconciliation of its kustomizations and sources with the repository. Defaults to `true`.
        pub continuous_reconciliation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `git_repository` block as defined below.
        pub git_repository: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::FluxConfigurationGitRepository>,
        >,
        /// A `kustomizations` block as defined below.
        pub kustomizations: pulumi_wasm_rust::Output<
            Vec<super::super::types::containerservice::FluxConfigurationKustomization>,
        >,
        /// Specifies the name which should be used for this Kubernetes Flux Configuration. Changing this forces a new Kubernetes Flux Configuration to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the namespace to which this configuration is installed to. Changing this forces a new Kubernetes Flux Configuration to be created.
        pub namespace: pulumi_wasm_rust::Output<String>,
        /// Specifies the scope at which the operator will be installed. Possible values are `cluster` and `namespace`. Defaults to `namespace`. Changing this forces a new Kubernetes Flux Configuration to be created.
        pub scope: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FluxConfigurationArgs) -> FluxConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let blob_storage_binding = args.blob_storage.get_inner();
        let bucket_binding = args.bucket.get_inner();
        let cluster_id_binding = args.cluster_id.get_inner();
        let continuous_reconciliation_enabled_binding = args
            .continuous_reconciliation_enabled
            .get_inner();
        let git_repository_binding = args.git_repository.get_inner();
        let kustomizations_binding = args.kustomizations.get_inner();
        let name_binding = args.name.get_inner();
        let namespace_binding = args.namespace.get_inner();
        let scope_binding = args.scope.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/fluxConfiguration:FluxConfiguration".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "blobStorage".into(),
                    value: &blob_storage_binding,
                },
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "continuousReconciliationEnabled".into(),
                    value: &continuous_reconciliation_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "gitRepository".into(),
                    value: &git_repository_binding,
                },
                register_interface::ObjectField {
                    name: "kustomizations".into(),
                    value: &kustomizations_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "blobStorage".into(),
                },
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "continuousReconciliationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "gitRepository".into(),
                },
                register_interface::ResultField {
                    name: "kustomizations".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namespace".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FluxConfigurationResult {
            blob_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blobStorage").unwrap(),
            ),
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            continuous_reconciliation_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("continuousReconciliationEnabled").unwrap(),
            ),
            git_repository: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gitRepository").unwrap(),
            ),
            kustomizations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kustomizations").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespace").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
        }
    }
}