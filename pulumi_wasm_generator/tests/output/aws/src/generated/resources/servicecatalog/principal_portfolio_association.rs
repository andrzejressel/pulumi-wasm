/// Manages a Service Catalog Principal Portfolio Association.
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
///     let example = principal_portfolio_association::create(
///         "example",
///         PrincipalPortfolioAssociationArgs::builder()
///             .portfolio_id("port-68656c6c6f")
///             .principal_arn("arn:aws:iam::123456789012:user/Eleanor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicecatalog_principal_portfolio_association` using `accept_language`, `principal_arn`, `portfolio_id`, and `principal_type` separated by a comma. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/principalPortfolioAssociation:PrincipalPortfolioAssociation example en,arn:aws:iam::123456789012:user/Eleanor,port-68656c6c6f,IAM
/// ```
pub mod principal_portfolio_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrincipalPortfolioAssociationArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// Portfolio identifier.
        #[builder(into)]
        pub portfolio_id: pulumi_wasm_rust::Output<String>,
        /// Principal ARN.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub principal_arn: pulumi_wasm_rust::Output<String>,
        /// Principal type. Setting this argument empty (e.g., `principal_type = ""`) will result in an error. Valid values are `IAM` and `IAM_PATTERN`. Default is `IAM`.
        #[builder(into, default)]
        pub principal_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PrincipalPortfolioAssociationResult {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// Portfolio identifier.
        pub portfolio_id: pulumi_wasm_rust::Output<String>,
        /// Principal ARN.
        ///
        /// The following arguments are optional:
        pub principal_arn: pulumi_wasm_rust::Output<String>,
        /// Principal type. Setting this argument empty (e.g., `principal_type = ""`) will result in an error. Valid values are `IAM` and `IAM_PATTERN`. Default is `IAM`.
        pub principal_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: PrincipalPortfolioAssociationArgs,
    ) -> PrincipalPortfolioAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accept_language_binding = args.accept_language.get_inner();
        let portfolio_id_binding = args.portfolio_id.get_inner();
        let principal_arn_binding = args.principal_arn.get_inner();
        let principal_type_binding = args.principal_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicecatalog/principalPortfolioAssociation:PrincipalPortfolioAssociation"
                .into(),
            name: name.to_string(),
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
                    name: "principalArn".into(),
                    value: &principal_arn_binding,
                },
                register_interface::ObjectField {
                    name: "principalType".into(),
                    value: &principal_type_binding,
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
                    name: "principalArn".into(),
                },
                register_interface::ResultField {
                    name: "principalType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PrincipalPortfolioAssociationResult {
            accept_language: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceptLanguage").unwrap(),
            ),
            portfolio_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("portfolioId").unwrap(),
            ),
            principal_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalArn").unwrap(),
            ),
            principal_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalType").unwrap(),
            ),
        }
    }
}