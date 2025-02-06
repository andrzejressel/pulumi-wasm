pub mod get_portfolio_constraints {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPortfolioConstraintsArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Portfolio identifier.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub portfolio_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Product identifier.
        #[builder(into, default)]
        pub product_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPortfolioConstraintsResult {
        pub accept_language: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of information about the constraints. See details below.
        pub details: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::servicecatalog::GetPortfolioConstraintsDetail,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the portfolio the product resides in. The constraint applies only to the instance of the product that lives within this portfolio.
        pub portfolio_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the product the constraint applies to. A constraint applies to a specific instance of a product within a certain portfolio.
        pub product_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetPortfolioConstraintsArgs,
    ) -> GetPortfolioConstraintsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let accept_language_binding = args
            .accept_language
            .get_output(context)
            .get_inner();
        let portfolio_id_binding = args.portfolio_id.get_output(context).get_inner();
        let product_id_binding = args.product_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:servicecatalog/getPortfolioConstraints:getPortfolioConstraints"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceptLanguage".into(),
                    value: &accept_language_binding,
                },
                register_interface::ObjectField {
                    name: "portfolioId".into(),
                    value: &portfolio_id_binding,
                },
                register_interface::ObjectField {
                    name: "productId".into(),
                    value: &product_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPortfolioConstraintsResult {
            accept_language: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceptLanguage"),
            ),
            details: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("details"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            portfolio_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("portfolioId"),
            ),
            product_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("productId"),
            ),
        }
    }
}
