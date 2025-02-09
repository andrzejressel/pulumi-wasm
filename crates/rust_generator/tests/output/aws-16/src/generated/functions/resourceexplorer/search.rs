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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: SearchArgs,
    ) -> SearchResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let query_string_binding_1 = args.query_string.get_output(context);
        let query_string_binding = query_string_binding_1.get_inner();
        let view_arn_binding_1 = args.view_arn.get_output(context);
        let view_arn_binding = view_arn_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:resourceexplorer/search:Search".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "queryString".into(),
                    value: &query_string_binding,
                },
                register_interface::ObjectField {
                    name: "viewArn".into(),
                    value: &view_arn_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        SearchResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            query_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("queryString"),
            ),
            resource_counts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceCounts"),
            ),
            resources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resources"),
            ),
            view_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("viewArn"),
            ),
        }
    }
}
