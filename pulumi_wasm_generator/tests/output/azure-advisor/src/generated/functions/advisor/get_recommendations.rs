pub mod get_recommendations {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRecommendationsArgs {
        /// Specifies a list of categories in which the Advisor Recommendations will be listed. Possible values are `HighAvailability`, `Security`, `Performance`, `Cost` and `OperationalExcellence`.
        #[builder(into, default)]
        pub filter_by_categories: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies a list of resource groups about which the Advisor Recommendations will be listed.
        #[builder(into, default)]
        pub filter_by_resource_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GetRecommendationsResult {
        pub filter_by_categories: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub filter_by_resource_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// One or more `recommendations` blocks as defined below.
        pub recommendations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::advisor::GetRecommendationsRecommendation>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRecommendationsArgs) -> GetRecommendationsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filter_by_categories_binding = args.filter_by_categories.get_inner();
        let filter_by_resource_groups_binding = args
            .filter_by_resource_groups
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:advisor/getRecommendations:getRecommendations".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filterByCategories".into(),
                    value: &filter_by_categories_binding,
                },
                register_interface::ObjectField {
                    name: "filterByResourceGroups".into(),
                    value: &filter_by_resource_groups_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "filterByCategories".into(),
                },
                register_interface::ResultField {
                    name: "filterByResourceGroups".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "recommendations".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRecommendationsResult {
            filter_by_categories: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filterByCategories").unwrap(),
            ),
            filter_by_resource_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filterByResourceGroups").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            recommendations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recommendations").unwrap(),
            ),
        }
    }
}
