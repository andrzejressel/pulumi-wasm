pub mod get_tags {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTagsArgs {
        /// Configuration block for the `Expression` object used to categorize costs. See `filter` block below for details.
        #[builder(into, default)]
        pub filter: pulumi_wasm_rust::Output<
            Option<super::super::super::types::costexplorer::GetTagsFilter>,
        >,
        /// Value that you want to search for.
        #[builder(into, default)]
        pub search_string: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for the value by which you want to sort the data. `sort_by` block below for details.
        #[builder(into, default)]
        pub sort_bies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::costexplorer::GetTagsSortBy>>,
        >,
        /// Key of the tag that you want to return values for.
        #[builder(into, default)]
        pub tag_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for the start and end dates for retrieving the dimension values. See `time_period` block below for details.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub time_period: pulumi_wasm_rust::Output<
            super::super::super::types::costexplorer::GetTagsTimePeriod,
        >,
    }
    #[allow(dead_code)]
    pub struct GetTagsResult {
        pub filter: pulumi_wasm_rust::Output<
            Option<super::super::super::types::costexplorer::GetTagsFilter>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub search_string: pulumi_wasm_rust::Output<Option<String>>,
        pub sort_bies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::costexplorer::GetTagsSortBy>>,
        >,
        pub tag_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Tags that match your request.
        pub tags: pulumi_wasm_rust::Output<Vec<String>>,
        pub time_period: pulumi_wasm_rust::Output<
            super::super::super::types::costexplorer::GetTagsTimePeriod,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetTagsArgs) -> GetTagsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filter_binding = args.filter.get_inner();
        let search_string_binding = args.search_string.get_inner();
        let sort_bies_binding = args.sort_bies.get_inner();
        let tag_key_binding = args.tag_key.get_inner();
        let time_period_binding = args.time_period.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "searchString".into(),
                },
                register_interface::ResultField {
                    name: "sortBies".into(),
                },
                register_interface::ResultField {
                    name: "tagKey".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "timePeriod".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTagsResult {
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            search_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("searchString").unwrap(),
            ),
            sort_bies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sortBies").unwrap(),
            ),
            tag_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagKey").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            time_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timePeriod").unwrap(),
            ),
        }
    }
}
