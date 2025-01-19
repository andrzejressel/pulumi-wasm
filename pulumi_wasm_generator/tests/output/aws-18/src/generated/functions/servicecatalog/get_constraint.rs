pub mod get_constraint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConstraintArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the constraint.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Constraint identifier.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetConstraintResult {
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the constraint.
        pub description: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// Owner of the constraint.
        pub owner: pulumi_wasm_rust::Output<String>,
        /// Constraint parameters in JSON format.
        pub parameters: pulumi_wasm_rust::Output<String>,
        /// Portfolio identifier.
        pub portfolio_id: pulumi_wasm_rust::Output<String>,
        /// Product identifier.
        pub product_id: pulumi_wasm_rust::Output<String>,
        /// Constraint status.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Type of constraint. Valid values are `LAUNCH`, `NOTIFICATION`, `RESOURCE_UPDATE`, `STACKSET`, and `TEMPLATE`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetConstraintArgs) -> GetConstraintResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accept_language_binding = args.accept_language.get_inner();
        let description_binding = args.description.get_inner();
        let id_binding = args.id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:servicecatalog/getConstraint:getConstraint".into(),
            version: super::super::super::get_version(),
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
                    name: "id".into(),
                    value: &id_binding,
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
                    name: "id".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetConstraintResult {
            accept_language: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceptLanguage").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
