/// Manages a Service Catalog Product Portfolio Association.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod product_portfolio_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProductPortfolioAssociationArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Portfolio identifier.
        #[builder(into)]
        pub portfolio_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Product identifier.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub product_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Identifier of the source portfolio.
        #[builder(into, default)]
        pub source_portfolio_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProductPortfolioAssociationResult {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// Portfolio identifier.
        pub portfolio_id: pulumi_wasm_rust::Output<String>,
        /// Product identifier.
        ///
        /// The following arguments are optional:
        pub product_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the source portfolio.
        pub source_portfolio_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ProductPortfolioAssociationArgs,
    ) -> ProductPortfolioAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accept_language_binding = args
            .accept_language
            .get_output(context)
            .get_inner();
        let portfolio_id_binding = args.portfolio_id.get_output(context).get_inner();
        let product_id_binding = args.product_id.get_output(context).get_inner();
        let source_portfolio_id_binding = args
            .source_portfolio_id
            .get_output(context)
            .get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceptLanguage".into(),
                },
                register_interface::ResultField {
                    name: "portfolioId".into(),
                },
                register_interface::ResultField {
                    name: "productId".into(),
                },
                register_interface::ResultField {
                    name: "sourcePortfolioId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProductPortfolioAssociationResult {
            accept_language: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceptLanguage").unwrap(),
            ),
            portfolio_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("portfolioId").unwrap(),
            ),
            product_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productId").unwrap(),
            ),
            source_portfolio_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourcePortfolioId").unwrap(),
            ),
        }
    }
}
