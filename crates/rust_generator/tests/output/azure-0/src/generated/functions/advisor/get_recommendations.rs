#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_recommendations {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRecommendationsArgs {
        /// Specifies a list of categories in which the Advisor Recommendations will be listed. Possible values are `HighAvailability`, `Security`, `Performance`, `Cost` and `OperationalExcellence`.
        #[builder(into, default)]
        pub filter_by_categories: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies a list of resource groups about which the Advisor Recommendations will be listed.
        #[builder(into, default)]
        pub filter_by_resource_groups: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRecommendationsResult {
        pub filter_by_categories: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub filter_by_resource_groups: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// One or more `recommendations` blocks as defined below.
        pub recommendations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::advisor::GetRecommendationsRecommendation>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRecommendationsArgs,
    ) -> GetRecommendationsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filter_by_categories_binding = args.filter_by_categories.get_output(context);
        let filter_by_resource_groups_binding = args
            .filter_by_resource_groups
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:advisor/getRecommendations:getRecommendations".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filterByCategories".into(),
                    value: &filter_by_categories_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filterByResourceGroups".into(),
                    value: &filter_by_resource_groups_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRecommendationsResult {
            filter_by_categories: o.get_field("filterByCategories"),
            filter_by_resource_groups: o.get_field("filterByResourceGroups"),
            id: o.get_field("id"),
            recommendations: o.get_field("recommendations"),
        }
    }
}
