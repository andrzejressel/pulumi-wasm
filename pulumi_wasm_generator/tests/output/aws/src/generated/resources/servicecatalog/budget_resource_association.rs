/// Manages a Service Catalog Budget Resource Association.
///
/// > **Tip:** A "resource" is either a Service Catalog portfolio or product.
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
///     let example = budget_resource_association::create(
///         "example",
///         BudgetResourceAssociationArgs::builder()
///             .budget_name("budget-pjtvyakdlyo3m")
///             .resource_id("prod-dnigbtea24ste")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicecatalog_budget_resource_association` using the budget name and resource ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/budgetResourceAssociation:BudgetResourceAssociation example budget-pjtvyakdlyo3m:prod-dnigbtea24ste
/// ```
pub mod budget_resource_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BudgetResourceAssociationArgs {
        /// Budget name.
        #[builder(into)]
        pub budget_name: pulumi_wasm_rust::Output<String>,
        /// Resource identifier.
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct BudgetResourceAssociationResult {
        /// Budget name.
        pub budget_name: pulumi_wasm_rust::Output<String>,
        /// Resource identifier.
        pub resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BudgetResourceAssociationArgs,
    ) -> BudgetResourceAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let budget_name_binding = args.budget_name.get_inner();
        let resource_id_binding = args.resource_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicecatalog/budgetResourceAssociation:BudgetResourceAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "budgetName".into(),
                    value: &budget_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "budgetName".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BudgetResourceAssociationResult {
            budget_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("budgetName").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
        }
    }
}