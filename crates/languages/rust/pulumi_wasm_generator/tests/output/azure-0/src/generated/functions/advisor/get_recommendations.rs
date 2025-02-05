pub mod get_recommendations {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRecommendationsArgs {
        /// Specifies a list of categories in which the Advisor Recommendations will be listed. Possible values are `HighAvailability`, `Security`, `Performance`, `Cost` and `OperationalExcellence`.
        #[builder(into, default)]
        pub filter_by_categories: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies a list of resource groups about which the Advisor Recommendations will be listed.
        #[builder(into, default)]
        pub filter_by_resource_groups: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRecommendationsArgs,
    ) -> GetRecommendationsResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filter_by_categories_binding = args
            .filter_by_categories
            .get_output(context)
            .get_inner();
        let filter_by_resource_groups_binding = args
            .filter_by_resource_groups
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:advisor/getRecommendations:getRecommendations".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRecommendationsResult {
            filter_by_categories: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filterByCategories"),
            ),
            filter_by_resource_groups: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filterByResourceGroups"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            recommendations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("recommendations"),
            ),
        }
    }
}
