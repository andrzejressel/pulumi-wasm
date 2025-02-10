#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:hdinsight/getCluster:getCluster".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetClusterResult {
            cluster_id: o.get_field("clusterId"),
            cluster_version: o.get_field("clusterVersion"),
            component_versions: o.get_field("componentVersions"),
            edge_ssh_endpoint: o.get_field("edgeSshEndpoint"),
            gateways: o.get_field("gateways"),
            https_endpoint: o.get_field("httpsEndpoint"),
            id: o.get_field("id"),
            kafka_rest_proxy_endpoint: o.get_field("kafkaRestProxyEndpoint"),
            kind: o.get_field("kind"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            ssh_endpoint: o.get_field("sshEndpoint"),
            tags: o.get_field("tags"),
            tier: o.get_field("tier"),
            tls_min_version: o.get_field("tlsMinVersion"),
        }
    }
}
