pub mod get_product {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProductArgs {
        /// Language code. Valid values are `en` (English), `jp` (Japanese), `zh` (Chinese). The default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID of the product.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Tags applied to the product.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetProductResult {
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the product.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Time when the product was created.
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// Description of the product.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Vendor of the product.
        pub distributor: pulumi_wasm_rust::Output<String>,
        /// Whether the product has a default path.
        pub has_default_path: pulumi_wasm_rust::Output<bool>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the product.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Owner of the product.
        pub owner: pulumi_wasm_rust::Output<String>,
        /// Status of the product.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Field that provides support information about the product.
        pub support_description: pulumi_wasm_rust::Output<String>,
        /// Contact email for product support.
        pub support_email: pulumi_wasm_rust::Output<String>,
        /// Contact URL for product support.
        pub support_url: pulumi_wasm_rust::Output<String>,
        /// Tags applied to the product.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of product.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetProductArgs,
    ) -> GetProductResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceptLanguage".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "distributor".into(),
                },
                register_interface::ResultField {
                    name: "hasDefaultPath".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "supportDescription".into(),
                },
                register_interface::ResultField {
                    name: "supportEmail".into(),
                },
                register_interface::ResultField {
                    name: "supportUrl".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetProductResult {
            accept_language: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceptLanguage").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            distributor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("distributor").unwrap(),
            ),
            has_default_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasDefaultPath").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            support_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportDescription").unwrap(),
            ),
            support_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportEmail").unwrap(),
            ),
            support_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportUrl").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
