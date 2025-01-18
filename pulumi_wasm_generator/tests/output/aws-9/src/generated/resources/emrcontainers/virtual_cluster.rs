/// Manages an EMR Containers (EMR on EKS) Virtual Cluster.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:emrcontainers:VirtualCluster
///     properties:
///       containerProvider:
///         id: ${exampleAwsEksCluster.name}
///         type: EKS
///         info:
///           eksInfo:
///             namespace: default
///       name: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EKS Clusters using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:emrcontainers/virtualCluster:VirtualCluster example a1b2c3d4e5f6g7h8i9j10k11l
/// ```
pub mod virtual_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualClusterArgs {
        /// Configuration block for the container provider associated with your cluster.
        #[builder(into)]
        pub container_provider: pulumi_wasm_rust::Output<
            super::super::types::emrcontainers::VirtualClusterContainerProvider,
        >,
        /// Name of the virtual cluster.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualClusterResult {
        /// ARN of the cluster.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the container provider associated with your cluster.
        pub container_provider: pulumi_wasm_rust::Output<
            super::super::types::emrcontainers::VirtualClusterContainerProvider,
        >,
        /// Name of the virtual cluster.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VirtualClusterArgs) -> VirtualClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_provider_binding = args.container_provider.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:emrcontainers/virtualCluster:VirtualCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerProvider".into(),
                    value: &container_provider_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "containerProvider".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualClusterResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            container_provider: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerProvider").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
