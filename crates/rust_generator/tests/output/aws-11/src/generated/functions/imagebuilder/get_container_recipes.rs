#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_container_recipes {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetContainerRecipesArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::imagebuilder::GetContainerRecipesFilter>,
            >,
        >,
        /// Owner of the container recipes. Valid values are `Self`, `Shared`, `Amazon` and `ThirdParty`. Defaults to `Self`.
        #[builder(into, default)]
        pub owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetContainerRecipesResult {
        /// Set of ARNs of the matched Image Builder Container Recipes.
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::imagebuilder::GetContainerRecipesFilter>,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set of names of the matched Image Builder Container Recipes.
        pub names: pulumi_gestalt_rust::Output<Vec<String>>,
        pub owner: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetContainerRecipesArgs,
    ) -> GetContainerRecipesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let owner_binding = args.owner.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:imagebuilder/getContainerRecipes:getContainerRecipes".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "owner".into(),
                    value: owner_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetContainerRecipesResult {
            arns: o.get_field("arns"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            names: o.get_field("names"),
            owner: o.get_field("owner"),
        }
    }
}
