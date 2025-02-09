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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ActiveReceiptRuleSetArgs,
    ) -> ActiveReceiptRuleSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let rule_set_name_binding = args.rule_set_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ses/activeReceiptRuleSet:ActiveReceiptRuleSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleSetName".into(),
                    value: rule_set_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ActiveReceiptRuleSetResult {
            arn: o.get_field("arn"),
            rule_set_name: o.get_field("ruleSetName"),
        }
    }
}
