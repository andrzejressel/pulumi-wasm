#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_image_recipes {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageRecipesArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::imagebuilder::GetImageRecipesFilter>>,
        >,
        /// Owner of the image recipes. Valid values are `Self`, `Shared`, `Amazon` and `ThirdParty`. Defaults to `Self`.
        #[builder(into, default)]
        pub owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetImageRecipesResult {
        /// Set of ARNs of the matched Image Builder Image Recipes.
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::imagebuilder::GetImageRecipesFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set of names of the matched Image Builder Image Recipes.
        pub names: pulumi_gestalt_rust::Output<Vec<String>>,
        pub owner: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetImageRecipesArgs,
    ) -> GetImageRecipesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding_1 = args.filters.get_output(context);
        let filters_binding = filters_binding_1.get_inner();
        let owner_binding_1 = args.owner.get_output(context);
        let owner_binding = owner_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:imagebuilder/getImageRecipes:getImageRecipes".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "owner".into(),
                    value: &owner_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetImageRecipesResult {
            arns: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arns")),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            names: pulumi_gestalt_rust::__private::into_domain(o.extract_field("names")),
            owner: pulumi_gestalt_rust::__private::into_domain(o.extract_field("owner")),
        }
    }
}
