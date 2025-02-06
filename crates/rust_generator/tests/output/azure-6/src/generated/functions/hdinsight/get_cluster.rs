pub mod get_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Specifies the name of this HDInsight Cluster.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group in which this HDInsight Cluster exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        /// The HDInsight Cluster ID.
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The version of HDInsights which is used on this HDInsight Cluster.
        pub cluster_version: pulumi_gestalt_rust::Output<String>,
        /// A map of versions of software used on this HDInsights Cluster.
        pub component_versions: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The SSH Endpoint of the Edge Node for this HDInsight Cluster, if an Edge Node exists.
        pub edge_ssh_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A `gateway` block as defined below.
        pub gateways: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::hdinsight::GetClusterGateway>,
        >,
        /// The HTTPS Endpoint for this HDInsight Cluster.
        pub https_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Kafka Rest Proxy Endpoint for this HDInsight Cluster.
        pub kafka_rest_proxy_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The kind of HDInsight Cluster this is, such as a Spark or Storm cluster.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region in which this HDInsight Cluster exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The HDInsight Cluster name.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SSH Endpoint for this HDInsight Cluster.
        pub ssh_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A map of tags assigned to the HDInsight Cluster.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The SKU / Tier of this HDInsight Cluster.
        pub tier: pulumi_gestalt_rust::Output<String>,
        /// The minimal supported TLS version.
        pub tls_min_version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClusterResult {
            cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterId"),
            ),
            cluster_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterVersion"),
            ),
            component_versions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("componentVersions"),
            ),
            edge_ssh_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("edgeSshEndpoint"),
            ),
            gateways: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gateways"),
            ),
            https_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpsEndpoint"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kafka_rest_proxy_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kafkaRestProxyEndpoint"),
            ),
            kind: pulumi_gestalt_rust::__private::into_domain(o.extract_field("kind")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            ssh_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sshEndpoint"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tier: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tier")),
            tls_min_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tlsMinVersion"),
            ),
        }
    }
}
