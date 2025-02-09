/// Manages a Service Catalog Product Portfolio Association.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = product_portfolio_association::create(
///         "example",
///         ProductPortfolioAssociationArgs::builder()
///             .portfolio_id("port-68656c6c6f")
///             .product_id("prod-dnigbtea24ste")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicecatalog_product_portfolio_association` using the accept language, portfolio ID, and product ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/productPortfolioAssociation:ProductPortfolioAssociation example en:port-68656c6c6f:prod-dnigbtea24ste
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod product_portfolio_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProductPortfolioAssociationArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Portfolio identifier.
        #[builder(into)]
        pub portfolio_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Product identifier.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub product_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of the source portfolio.
        #[builder(into, default)]
        pub source_portfolio_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProductPortfolioAssociationResult {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        pub accept_language: pulumi_gestalt_rust::Output<Option<String>>,
        /// Portfolio identifier.
        pub portfolio_id: pulumi_gestalt_rust::Output<String>,
        /// Product identifier.
        ///
        /// The following arguments are optional:
        pub product_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the source portfolio.
        pub source_portfolio_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ProductPortfolioAssociationArgs,
    ) -> ProductPortfolioAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let accept_language_binding_1 = args.accept_language.get_output(context);
        let accept_language_binding = accept_language_binding_1.get_inner();
        let portfolio_id_binding_1 = args.portfolio_id.get_output(context);
        let portfolio_id_binding = portfolio_id_binding_1.get_inner();
        let product_id_binding_1 = args.product_id.get_output(context);
        let product_id_binding = product_id_binding_1.get_inner();
        let source_portfolio_id_binding_1 = args.source_portfolio_id.get_output(context);
        let source_portfolio_id_binding = source_portfolio_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicecatalog/productPortfolioAssociation:ProductPortfolioAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                register_interface::ObjectField {
                    name: "sourcePortfolioId".into(),
                    value: &source_portfolio_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProductPortfolioAssociationResult {
            accept_language: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceptLanguage"),
            ),
            portfolio_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("portfolioId"),
            ),
            product_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("productId"),
            ),
            source_portfolio_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourcePortfolioId"),
            ),
        }
    }
}
