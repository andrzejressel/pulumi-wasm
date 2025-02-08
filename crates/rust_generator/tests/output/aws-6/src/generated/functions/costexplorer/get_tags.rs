#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetTagsArgs,
    ) -> GetTagsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filter_binding = args.filter.get_output(context).get_inner();
        let search_string_binding = args.search_string.get_output(context).get_inner();
        let sort_bies_binding = args.sort_bies.get_output(context).get_inner();
        let tag_key_binding = args.tag_key.get_output(context).get_inner();
        let time_period_binding = args.time_period.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:costexplorer/getTags:getTags".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "searchString".into(),
                    value: &search_string_binding,
                },
                register_interface::ObjectField {
                    name: "sortBies".into(),
                    value: &sort_bies_binding,
                },
                register_interface::ObjectField {
                    name: "tagKey".into(),
                    value: &tag_key_binding,
                },
                register_interface::ObjectField {
                    name: "timePeriod".into(),
                    value: &time_period_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTagsResult {
            filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filter"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            search_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("searchString"),
            ),
            sort_bies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sortBies"),
            ),
            tag_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagKey"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            time_period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timePeriod"),
            ),
        }
    }
}
