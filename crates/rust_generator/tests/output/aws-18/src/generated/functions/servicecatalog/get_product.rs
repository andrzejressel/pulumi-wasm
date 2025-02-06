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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetProductArgs,
    ) -> GetProductResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let accept_language_binding = args
            .accept_language
            .get_output(context)
            .get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:servicecatalog/getProduct:getProduct".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceptLanguage".into(),
                    value: &accept_language_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetProductResult {
            accept_language: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceptLanguage"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            created_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            distributor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("distributor"),
            ),
            has_default_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hasDefaultPath"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner: pulumi_gestalt_rust::__private::into_domain(o.extract_field("owner")),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            support_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportDescription"),
            ),
            support_email: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportEmail"),
            ),
            support_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportUrl"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
