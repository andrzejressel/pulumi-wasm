pub mod get_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Specifies the name of this HDInsight Cluster.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group in which this HDInsight Cluster exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        /// The HDInsight Cluster ID.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// The version of HDInsights which is used on this HDInsight Cluster.
        pub cluster_version: pulumi_wasm_rust::Output<String>,
        /// A map of versions of software used on this HDInsights Cluster.
        pub component_versions: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The SSH Endpoint of the Edge Node for this HDInsight Cluster, if an Edge Node exists.
        pub edge_ssh_endpoint: pulumi_wasm_rust::Output<String>,
        /// A `gateway` block as defined below.
        pub gateways: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::hdinsight::GetClusterGateway>,
        >,
        /// The HTTPS Endpoint for this HDInsight Cluster.
        pub https_endpoint: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Kafka Rest Proxy Endpoint for this HDInsight Cluster.
        pub kafka_rest_proxy_endpoint: pulumi_wasm_rust::Output<String>,
        /// The kind of HDInsight Cluster this is, such as a Spark or Storm cluster.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The Azure Region in which this HDInsight Cluster exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The HDInsight Cluster name.
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SSH Endpoint for this HDInsight Cluster.
        pub ssh_endpoint: pulumi_wasm_rust::Output<String>,
        /// A map of tags assigned to the HDInsight Cluster.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The SKU / Tier of this HDInsight Cluster.
        pub tier: pulumi_wasm_rust::Output<String>,
        /// The minimal supported TLS version.
        pub tls_min_version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetClusterArgs) -> GetClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:hdinsight/getCluster:getCluster".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "clusterVersion".into(),
                },
                register_interface::ResultField {
                    name: "componentVersions".into(),
                },
                register_interface::ResultField {
                    name: "edgeSshEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "gateways".into(),
                },
                register_interface::ResultField {
                    name: "httpsEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kafkaRestProxyEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sshEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tier".into(),
                },
                register_interface::ResultField {
                    name: "tlsMinVersion".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetClusterResult {
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            cluster_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterVersion").unwrap(),
            ),
            component_versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("componentVersions").unwrap(),
            ),
            edge_ssh_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeSshEndpoint").unwrap(),
            ),
            gateways: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gateways").unwrap(),
            ),
            https_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsEndpoint").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kafka_rest_proxy_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kafkaRestProxyEndpoint").unwrap(),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            ssh_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sshEndpoint").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tier").unwrap(),
            ),
            tls_min_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tlsMinVersion").unwrap(),
            ),
        }
    }
}
