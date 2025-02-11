#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetClustersArgs,
    ) -> GetClustersResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:rds/getClusters:getClusters".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetClustersResult {
            cluster_arns: o.get_field("clusterArns"),
            cluster_identifiers: o.get_field("clusterIdentifiers"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
        }
    }
}
