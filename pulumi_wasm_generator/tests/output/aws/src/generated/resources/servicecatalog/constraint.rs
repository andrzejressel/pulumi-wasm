/// Manages a Service Catalog Constraint.
///
/// > **NOTE:** This resource does not associate a Service Catalog product and portfolio. However, the product and portfolio must be associated (see the `aws.servicecatalog.ProductPortfolioAssociation` resource) prior to creating a constraint or you will receive an error.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:servicecatalog:Constraint
///     properties:
///       description: Back off, man. I'm a scientist.
///       portfolioId: ${exampleAwsServicecatalogPortfolio.id}
///       productId: ${exampleAwsServicecatalogProduct.id}
///       type: LAUNCH
///       parameters:
///         fn::toJSON:
///           RoleArn: arn:aws:iam::123456789012:role/LaunchRole
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicecatalog_constraint` using the constraint ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/constraint:Constraint example cons-nmdkb6cgxfcrs
/// ```
pub mod constraint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConstraintArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the constraint.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Constraint parameters in JSON format. The syntax depends on the constraint type. See details below.
        #[builder(into)]
        pub parameters: pulumi_wasm_rust::Output<String>,
        /// Portfolio identifier.
        #[builder(into)]
        pub portfolio_id: pulumi_wasm_rust::Output<String>,
        /// Product identifier.
        #[builder(into)]
        pub product_id: pulumi_wasm_rust::Output<String>,
        /// Type of constraint. Valid values are `LAUNCH`, `NOTIFICATION`, `RESOURCE_UPDATE`, `STACKSET`, and `TEMPLATE`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ConstraintResult {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the constraint.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Owner of the constraint.
        pub owner: pulumi_wasm_rust::Output<String>,
        /// Constraint parameters in JSON format. The syntax depends on the constraint type. See details below.
        pub parameters: pulumi_wasm_rust::Output<String>,
        /// Portfolio identifier.
        pub portfolio_id: pulumi_wasm_rust::Output<String>,
        /// Product identifier.
        pub product_id: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<String>,
        /// Type of constraint. Valid values are `LAUNCH`, `NOTIFICATION`, `RESOURCE_UPDATE`, `STACKSET`, and `TEMPLATE`.
        ///
        /// The following arguments are optional:
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConstraintArgs) -> ConstraintResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accept_language_binding = args.accept_language.get_inner();
        let description_binding = args.description.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let portfolio_id_binding = args.portfolio_id.get_inner();
        let product_id_binding = args.product_id.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicecatalog/constraint:Constraint".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceptLanguage".into(),
                    value: &accept_language_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
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
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceptLanguage".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "portfolioId".into(),
                },
                register_interface::ResultField {
                    name: "productId".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConstraintResult {
            accept_language: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceptLanguage").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            portfolio_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("portfolioId").unwrap(),
            ),
            product_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productId").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}