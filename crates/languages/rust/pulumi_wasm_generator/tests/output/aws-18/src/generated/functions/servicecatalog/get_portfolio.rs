pub mod get_portfolio {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPortfolioArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Portfolio identifier.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Tags applied to the portfolio.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPortfolioResult {
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// Portfolio ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Time the portfolio was created.
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// Description of the portfolio
        pub description: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// Portfolio name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Name of the person or organization who owns the portfolio.
        pub provider_name: pulumi_wasm_rust::Output<String>,
        /// Tags applied to the portfolio.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPortfolioArgs,
    ) -> GetPortfolioResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accept_language_binding = args
            .accept_language
            .get_output(context)
            .get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:servicecatalog/getPortfolio:getPortfolio".into(),
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
        GetPortfolioResult {
            accept_language: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("acceptLanguage"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            created_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            provider_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("providerName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
