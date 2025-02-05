/// Manages a Service Catalog Portfolio Share. Shares the specified portfolio with the specified account or organization node. You can share portfolios to an organization, an organizational unit, or a specific account.
///
/// If the portfolio share with the specified account or organization node already exists, using this resource to re-create the share will have no effect and will not return an error. You can then use this resource to update the share.
///
/// > **NOTE:** Shares to an organization node can only be created by the management account of an organization or by a delegated administrator. If a delegated admin is de-registered, they can no longer create portfolio shares.
///
/// > **NOTE:** AWSOrganizationsAccess must be enabled in order to create a portfolio share to an organization node.
///
/// > **NOTE:** You can't share a shared resource, including portfolios that contain a shared product.
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
///     let example = portfolio_share::create(
///         "example",
///         PortfolioShareArgs::builder()
///             .portfolio_id("${exampleAwsServicecatalogPortfolio.id}")
///             .principal_id("012128675309")
///             .type_("ACCOUNT")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicecatalog_portfolio_share` using the portfolio share ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/portfolioShare:PortfolioShare example port-12344321:ACCOUNT:123456789012
/// ```
pub mod portfolio_share {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PortfolioShareArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Portfolio identifier.
        #[builder(into)]
        pub portfolio_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Identifier of the principal with whom you will share the portfolio. Valid values AWS account IDs and ARNs of AWS Organizations and organizational units.
        #[builder(into)]
        pub principal_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Enables or disables Principal sharing when creating the portfolio share. If this flag is not provided, principal sharing is disabled.
        #[builder(into, default)]
        pub share_principals: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether to enable sharing of `aws.servicecatalog.TagOption` resources when creating the portfolio share.
        #[builder(into, default)]
        pub share_tag_options: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Type of portfolio share. Valid values are `ACCOUNT` (an external account), `ORGANIZATION` (a share to every account in an organization), `ORGANIZATIONAL_UNIT`, `ORGANIZATION_MEMBER_ACCOUNT` (a share to an account in an organization).
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether to wait (up to the timeout) for the share to be accepted. Organizational shares are automatically accepted.
        #[builder(into, default)]
        pub wait_for_acceptance: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct PortfolioShareResult {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the shared portfolio is imported by the recipient account. If the recipient is organizational, the share is automatically imported, and the field is always set to true.
        pub accepted: pulumi_wasm_rust::Output<bool>,
        /// Portfolio identifier.
        pub portfolio_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the principal with whom you will share the portfolio. Valid values AWS account IDs and ARNs of AWS Organizations and organizational units.
        pub principal_id: pulumi_wasm_rust::Output<String>,
        /// Enables or disables Principal sharing when creating the portfolio share. If this flag is not provided, principal sharing is disabled.
        pub share_principals: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to enable sharing of `aws.servicecatalog.TagOption` resources when creating the portfolio share.
        pub share_tag_options: pulumi_wasm_rust::Output<Option<bool>>,
        /// Type of portfolio share. Valid values are `ACCOUNT` (an external account), `ORGANIZATION` (a share to every account in an organization), `ORGANIZATIONAL_UNIT`, `ORGANIZATION_MEMBER_ACCOUNT` (a share to an account in an organization).
        ///
        /// The following arguments are optional:
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Whether to wait (up to the timeout) for the share to be accepted. Organizational shares are automatically accepted.
        pub wait_for_acceptance: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PortfolioShareArgs,
    ) -> PortfolioShareResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accept_language_binding = args
            .accept_language
            .get_output(context)
            .get_inner();
        let portfolio_id_binding = args.portfolio_id.get_output(context).get_inner();
        let principal_id_binding = args.principal_id.get_output(context).get_inner();
        let share_principals_binding = args
            .share_principals
            .get_output(context)
            .get_inner();
        let share_tag_options_binding = args
            .share_tag_options
            .get_output(context)
            .get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let wait_for_acceptance_binding = args
            .wait_for_acceptance
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicecatalog/portfolioShare:PortfolioShare".into(),
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
                    name: "principalId".into(),
                    value: &principal_id_binding,
                },
                register_interface::ObjectField {
                    name: "sharePrincipals".into(),
                    value: &share_principals_binding,
                },
                register_interface::ObjectField {
                    name: "shareTagOptions".into(),
                    value: &share_tag_options_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "waitForAcceptance".into(),
                    value: &wait_for_acceptance_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PortfolioShareResult {
            accept_language: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("acceptLanguage"),
            ),
            accepted: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accepted"),
            ),
            portfolio_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("portfolioId"),
            ),
            principal_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("principalId"),
            ),
            share_principals: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sharePrincipals"),
            ),
            share_tag_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("shareTagOptions"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            wait_for_acceptance: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("waitForAcceptance"),
            ),
        }
    }
}
