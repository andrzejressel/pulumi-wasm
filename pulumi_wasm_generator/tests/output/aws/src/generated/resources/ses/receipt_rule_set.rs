/// Provides an SES receipt rule set resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = receipt_rule_set::create(
///         "main",
///         ReceiptRuleSetArgs::builder().rule_set_name("primary-rules").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SES receipt rule sets using the rule set name. For example:
///
/// ```sh
/// $ pulumi import aws:ses/receiptRuleSet:ReceiptRuleSet my_rule_set my_rule_set_name
/// ```
pub mod receipt_rule_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReceiptRuleSetArgs {
        /// Name of the rule set.
        #[builder(into)]
        pub rule_set_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ReceiptRuleSetResult {
        /// SES receipt rule set ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of the rule set.
        pub rule_set_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ReceiptRuleSetArgs) -> ReceiptRuleSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let rule_set_name_binding = args.rule_set_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ses/receiptRuleSet:ReceiptRuleSet".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ruleSetName".into(),
                    value: &rule_set_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "ruleSetName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReceiptRuleSetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            rule_set_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleSetName").unwrap(),
            ),
        }
    }
}