#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_tags {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTagsArgs {
        /// Configuration block for the `Expression` object used to categorize costs. See `filter` block below for details.
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::costexplorer::GetTagsFilter>,
        >,
        /// Value that you want to search for.
        #[builder(into, default)]
        pub search_string: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for the value by which you want to sort the data. `sort_by` block below for details.
        #[builder(into, default)]
        pub sort_bies: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::costexplorer::GetTagsSortBy>>,
        >,
        /// Key of the tag that you want to return values for.
        #[builder(into, default)]
        pub tag_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for the start and end dates for retrieving the dimension values. See `time_period` block below for details.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub time_period: pulumi_gestalt_rust::InputOrOutput<
            super::super::super::types::costexplorer::GetTagsTimePeriod,
        >,
    }
    #[allow(dead_code)]
    pub struct GetTagsResult {
        pub filter: pulumi_gestalt_rust::Output<
            Option<super::super::super::types::costexplorer::GetTagsFilter>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub search_string: pulumi_gestalt_rust::Output<Option<String>>,
        pub sort_bies: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::costexplorer::GetTagsSortBy>>,
        >,
        pub tag_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Tags that match your request.
        pub tags: pulumi_gestalt_rust::Output<Vec<String>>,
        pub time_period: pulumi_gestalt_rust::Output<
            super::super::super::types::costexplorer::GetTagsTimePeriod,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTagsArgs,
    ) -> GetTagsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filter_binding = args.filter.get_output(context);
        let search_string_binding = args.search_string.get_output(context);
        let sort_bies_binding = args.sort_bies.get_output(context);
        let tag_key_binding = args.tag_key.get_output(context);
        let time_period_binding = args.time_period.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:costexplorer/getTags:getTags".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "searchString".into(),
                    value: search_string_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sortBies".into(),
                    value: sort_bies_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tagKey".into(),
                    value: tag_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timePeriod".into(),
                    value: time_period_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTagsResult {
            filter: o.get_field("filter"),
            id: o.get_field("id"),
            search_string: o.get_field("searchString"),
            sort_bies: o.get_field("sortBies"),
            tag_key: o.get_field("tagKey"),
            tags: o.get_field("tags"),
            time_period: o.get_field("timePeriod"),
        }
    }
}
