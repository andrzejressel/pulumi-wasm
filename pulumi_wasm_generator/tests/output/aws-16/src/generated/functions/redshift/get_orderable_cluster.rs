pub mod get_orderable_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrderableClusterArgs {
        /// Reshift Cluster typeE.g., `multi-node` or `single-node`
        #[builder(into, default)]
        pub cluster_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Redshift Cluster versionE.g., `1.0`
        #[builder(into, default)]
        pub cluster_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Redshift Cluster node typeE.g., `dc2.8xlarge`
        #[builder(into, default)]
        pub node_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Ordered list of preferred Redshift Cluster node types. The first match in this list will be returned. If no preferred matches are found and the original search returned more than one result, an error is returned.
        #[builder(into, default)]
        pub preferred_node_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GetOrderableClusterResult {
        /// List of Availability Zone names where the Redshift Cluster is available.
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        pub cluster_type: pulumi_wasm_rust::Output<String>,
        pub cluster_version: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub node_type: pulumi_wasm_rust::Output<String>,
        pub preferred_node_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetOrderableClusterArgs) -> GetOrderableClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_type_binding = args.cluster_type.get_inner();
        let cluster_version_binding = args.cluster_version.get_inner();
        let node_type_binding = args.node_type.get_inner();
        let preferred_node_types_binding = args.preferred_node_types.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:redshift/getOrderableCluster:getOrderableCluster".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterType".into(),
                    value: &cluster_type_binding,
                },
                register_interface::ObjectField {
                    name: "clusterVersion".into(),
                    value: &cluster_version_binding,
                },
                register_interface::ObjectField {
                    name: "nodeType".into(),
                    value: &node_type_binding,
                },
                register_interface::ObjectField {
                    name: "preferredNodeTypes".into(),
                    value: &preferred_node_types_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "availabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "clusterType".into(),
                },
                register_interface::ResultField {
                    name: "clusterVersion".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "nodeType".into(),
                },
                register_interface::ResultField {
                    name: "preferredNodeTypes".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOrderableClusterResult {
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZones").unwrap(),
            ),
            cluster_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterType").unwrap(),
            ),
            cluster_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterVersion").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            node_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeType").unwrap(),
            ),
            preferred_node_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredNodeTypes").unwrap(),
            ),
        }
    }
}
