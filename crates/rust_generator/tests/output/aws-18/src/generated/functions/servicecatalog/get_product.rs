#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_product {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProductArgs {
        /// Language code. Valid values are `en` (English), `jp` (Japanese), `zh` (Chinese). The default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the product.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tags applied to the product.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetProductResult {
        pub accept_language: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the product.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Time when the product was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// Description of the product.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Vendor of the product.
        pub distributor: pulumi_gestalt_rust::Output<String>,
        /// Whether the product has a default path.
        pub has_default_path: pulumi_gestalt_rust::Output<bool>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the product.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Owner of the product.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// Status of the product.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Field that provides support information about the product.
        pub support_description: pulumi_gestalt_rust::Output<String>,
        /// Contact email for product support.
        pub support_email: pulumi_gestalt_rust::Output<String>,
        /// Contact URL for product support.
        pub support_url: pulumi_gestalt_rust::Output<String>,
        /// Tags applied to the product.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of product.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetProductArgs,
    ) -> GetProductResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let accept_language_binding = args.accept_language.get_output(context);
        let id_binding = args.id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:servicecatalog/getProduct:getProduct".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acceptLanguage".into(),
                    value: accept_language_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetProductResult {
            accept_language: o.get_field("acceptLanguage"),
            arn: o.get_field("arn"),
            created_time: o.get_field("createdTime"),
            description: o.get_field("description"),
            distributor: o.get_field("distributor"),
            has_default_path: o.get_field("hasDefaultPath"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            owner: o.get_field("owner"),
            status: o.get_field("status"),
            support_description: o.get_field("supportDescription"),
            support_email: o.get_field("supportEmail"),
            support_url: o.get_field("supportUrl"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
        }
    }
}
