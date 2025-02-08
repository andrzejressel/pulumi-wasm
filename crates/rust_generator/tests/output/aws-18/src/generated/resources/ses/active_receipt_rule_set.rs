/// Provides a resource to designate the active SES receipt rule set
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = active_receipt_rule_set::create(
///         "main",
///         ActiveReceiptRuleSetArgs::builder().rule_set_name("primary-rules").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import active SES receipt rule sets using the rule set name. For example:
///
/// ```sh
/// $ pulumi import aws:ses/activeReceiptRuleSet:ActiveReceiptRuleSet my_rule_set my_rule_set_name
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod active_receipt_rule_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ActiveReceiptRuleSetArgs {
        /// The name of the rule set
        #[builder(into)]
        pub rule_set_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ActiveReceiptRuleSetResult {
        /// The SES receipt rule set ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the rule set
        pub rule_set_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ActiveReceiptRuleSetArgs,
    ) -> ActiveReceiptRuleSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let rule_set_name_binding = args.rule_set_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ses/activeReceiptRuleSet:ActiveReceiptRuleSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ruleSetName".into(),
                    value: &rule_set_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ActiveReceiptRuleSetResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            rule_set_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ruleSetName"),
            ),
        }
    }
}
