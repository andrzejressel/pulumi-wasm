#[allow(clippy::doc_lazy_continuation)]
pub mod get_clusters {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClustersArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::rds::GetClustersFilter>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClustersResult {
        /// Set of cluster ARNs of the matched RDS clusters.
        pub cluster_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of ARNs of cluster identifiers of the matched RDS clusters.
        pub cluster_identifiers: pulumi_gestalt_rust::Output<Vec<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::rds::GetClustersFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetClustersArgs,
    ) -> GetClustersResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:rds/getClusters:getClusters".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClustersResult {
            cluster_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterArns"),
            ),
            cluster_identifiers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterIdentifiers"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
