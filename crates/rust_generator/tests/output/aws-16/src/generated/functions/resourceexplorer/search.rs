#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod search {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SearchArgs {
        /// String that includes keywords and filters that specify the resources that you want to include in the results. For the complete syntax supported by the QueryString parameter, see Search query syntax reference for [Resource Explorer](https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html). The search is completely case insensitive. You can specify an empty string to return all results up to the limit of 1,000 total results. The operation can return only the first 1,000 results. If the resource you want is not included, then use a different value for QueryString to refine the results.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub query_string: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Amazon resource name (ARN) of the view to use for the query. If you don't specify a value for this parameter, then the operation automatically uses the default view for the AWS Region in which you called this operation. If the Region either doesn't have a default view or if you don't have permission to use the default view, then the operation fails with a `401 Unauthorized` exception.
        #[builder(into, default)]
        pub view_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SearchResult {
        /// Query String.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub query_string: pulumi_gestalt_rust::Output<String>,
        /// Number of resources that match the query. See `resource_count` below.
        pub resource_counts: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::resourceexplorer::SearchResourceCount>,
        >,
        /// List of structures that describe the resources that match the query. See `resources` below.
        pub resources: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::resourceexplorer::SearchResource>,
        >,
        pub view_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: SearchArgs,
    ) -> SearchResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let query_string_binding = args.query_string.get_output(context);
        let view_arn_binding = args.view_arn.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:resourceexplorer/search:Search".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queryString".into(),
                    value: query_string_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "viewArn".into(),
                    value: view_arn_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        SearchResult {
            id: o.get_field("id"),
            query_string: o.get_field("queryString"),
            resource_counts: o.get_field("resourceCounts"),
            resources: o.get_field("resources"),
            view_arn: o.get_field("viewArn"),
        }
    }
}
