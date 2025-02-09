/// Manages a Service Catalog Principal Portfolio Association.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod principal_portfolio_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrincipalPortfolioAssociationArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Portfolio identifier.
        #[builder(into)]
        pub portfolio_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Principal ARN.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub principal_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Principal type. Setting this argument empty (e.g., `principal_type = ""`) will result in an error. Valid values are `IAM` and `IAM_PATTERN`. Default is `IAM`.
        #[builder(into, default)]
        pub principal_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PrincipalPortfolioAssociationResult {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        pub accept_language: pulumi_gestalt_rust::Output<Option<String>>,
        /// Portfolio identifier.
        pub portfolio_id: pulumi_gestalt_rust::Output<String>,
        /// Principal ARN.
        ///
        /// The following arguments are optional:
        pub principal_arn: pulumi_gestalt_rust::Output<String>,
        /// Principal type. Setting this argument empty (e.g., `principal_type = ""`) will result in an error. Valid values are `IAM` and `IAM_PATTERN`. Default is `IAM`.
        pub principal_type: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PrincipalPortfolioAssociationArgs,
    ) -> PrincipalPortfolioAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let accept_language_binding_1 = args.accept_language.get_output(context);
        let accept_language_binding = accept_language_binding_1.get_inner();
        let portfolio_id_binding_1 = args.portfolio_id.get_output(context);
        let portfolio_id_binding = portfolio_id_binding_1.get_inner();
        let principal_arn_binding_1 = args.principal_arn.get_output(context);
        let principal_arn_binding = principal_arn_binding_1.get_inner();
        let principal_type_binding_1 = args.principal_type.get_output(context);
        let principal_type_binding = principal_type_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicecatalog/principalPortfolioAssociation:PrincipalPortfolioAssociation"
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
                    name: "principalArn".into(),
                    value: &principal_arn_binding,
                },
                register_interface::ObjectField {
                    name: "principalType".into(),
                    value: &principal_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PrincipalPortfolioAssociationResult {
            accept_language: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceptLanguage"),
            ),
            portfolio_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("portfolioId"),
            ),
            principal_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principalArn"),
            ),
            principal_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principalType"),
            ),
        }
    }
}
