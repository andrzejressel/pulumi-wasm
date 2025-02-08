#[allow(clippy::doc_lazy_continuation)]
pub mod get_orderable_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrderableClusterArgs {
        /// Reshift Cluster typeE.g., `multi-node` or `single-node`
        #[builder(into, default)]
        pub cluster_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Redshift Cluster versionE.g., `1.0`
        #[builder(into, default)]
        pub cluster_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Redshift Cluster node typeE.g., `dc2.8xlarge`
        #[builder(into, default)]
        pub node_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Ordered list of preferred Redshift Cluster node types. The first match in this list will be returned. If no preferred matches are found and the original search returned more than one result, an error is returned.
        #[builder(into, default)]
        pub preferred_node_types: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetOrderableClusterResult {
        /// List of Availability Zone names where the Redshift Cluster is available.
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        pub cluster_type: pulumi_gestalt_rust::Output<String>,
        pub cluster_version: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub node_type: pulumi_gestalt_rust::Output<String>,
        pub preferred_node_types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetOrderableClusterArgs,
    ) -> GetOrderableClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_type_binding = args.cluster_type.get_output(context).get_inner();
        let cluster_version_binding = args
            .cluster_version
            .get_output(context)
            .get_inner();
        let node_type_binding = args.node_type.get_output(context).get_inner();
        let preferred_node_types_binding = args
            .preferred_node_types
            .get_output(context)
            .get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOrderableClusterResult {
            availability_zones: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZones"),
            ),
            cluster_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterType"),
            ),
            cluster_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterVersion"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            node_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeType"),
            ),
            preferred_node_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preferredNodeTypes"),
            ),
        }
    }
}
