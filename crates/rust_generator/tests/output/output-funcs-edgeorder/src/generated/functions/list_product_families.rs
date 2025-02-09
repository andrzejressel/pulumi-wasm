#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod list_product_families {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListProductFamiliesArgs {
        /// Customer subscription properties. Clients can display available products to unregistered customers by explicitly passing subscription details
        #[builder(into, default)]
        pub customer_subscription_details: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::CustomerSubscriptionDetails>,
        >,
        /// $expand is supported on configurations parameter for product, which provides details on the configurations for the product.
        #[builder(into, default)]
        pub expand: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Dictionary of filterable properties on product family.
        #[builder(into)]
        pub filterable_properties: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<
                String,
                Vec<super::super::types::FilterableProperty>,
            >,
        >,
        /// $skipToken is supported on list of product families, which provides the next page in the list of product families.
        #[builder(into, default)]
        pub skip_token: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ListProductFamiliesResult {
        /// Link for the next set of product families.
        pub next_link: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of product families.
        pub value: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ProductFamilyResponse>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: ListProductFamiliesArgs,
    ) -> ListProductFamiliesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let customer_subscription_details_binding = args
            .customer_subscription_details
            .get_output(context);
        let expand_binding = args.expand.get_output(context);
        let filterable_properties_binding = args
            .filterable_properties
            .get_output(context);
        let skip_token_binding = args.skip_token.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "myedgeorder::listProductFamilies".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerSubscriptionDetails".into(),
                    value: customer_subscription_details_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expand".into(),
                    value: expand_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filterableProperties".into(),
                    value: filterable_properties_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipToken".into(),
                    value: skip_token_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        ListProductFamiliesResult {
            next_link: o.get_field("nextLink"),
            value: o.get_field("value"),
        }
    }
}
