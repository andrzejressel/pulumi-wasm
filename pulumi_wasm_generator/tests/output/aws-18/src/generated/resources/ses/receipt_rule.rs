/// Provides an SES receipt rule resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let store = receipt_rule::create(
///         "store",
///         ReceiptRuleArgs::builder()
///             .add_header_actions(
///                 vec![
///                     ReceiptRuleAddHeaderAction::builder().headerName("Custom-Header")
///                     .headerValue("Added by SES").position(1).build_struct(),
///                 ],
///             )
///             .enabled(true)
///             .name("store")
///             .recipients(vec!["karen@example.com",])
///             .rule_set_name("default-rule-set")
///             .s_3_actions(
///                 vec![
///                     ReceiptRuleS3Action::builder().bucketName("emails").position(2)
///                     .build_struct(),
///                 ],
///             )
///             .scan_enabled(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SES receipt rules using the ruleset name and rule name separated by `:`. For example:
///
/// ```sh
/// $ pulumi import aws:ses/receiptRule:ReceiptRule my_rule my_rule_set:my_rule
/// ```
pub mod receipt_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReceiptRuleArgs {
        /// A list of Add Header Action blocks. Documented below.
        #[builder(into, default)]
        pub add_header_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleAddHeaderAction>>,
        >,
        /// The name of the rule to place this rule after
        #[builder(into, default)]
        pub after: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of Bounce Action blocks. Documented below.
        #[builder(into, default)]
        pub bounce_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleBounceAction>>,
        >,
        /// If true, the rule will be enabled
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of Lambda Action blocks. Documented below.
        #[builder(into, default)]
        pub lambda_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleLambdaAction>>,
        >,
        /// The name of the rule
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of email addresses
        #[builder(into, default)]
        pub recipients: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the rule set
        #[builder(into)]
        pub rule_set_name: pulumi_wasm_rust::Output<String>,
        /// A list of S3 Action blocks. Documented below.
        #[builder(into, default)]
        pub s3_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleS3Action>>,
        >,
        /// If true, incoming emails will be scanned for spam and viruses
        #[builder(into, default)]
        pub scan_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of SNS Action blocks. Documented below.
        #[builder(into, default)]
        pub sns_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleSnsAction>>,
        >,
        /// A list of Stop Action blocks. Documented below.
        #[builder(into, default)]
        pub stop_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleStopAction>>,
        >,
        /// `Require` or `Optional`
        #[builder(into, default)]
        pub tls_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of WorkMail Action blocks. Documented below.
        #[builder(into, default)]
        pub workmail_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleWorkmailAction>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReceiptRuleResult {
        /// A list of Add Header Action blocks. Documented below.
        pub add_header_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleAddHeaderAction>>,
        >,
        /// The name of the rule to place this rule after
        pub after: pulumi_wasm_rust::Output<Option<String>>,
        /// The SES receipt rule ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A list of Bounce Action blocks. Documented below.
        pub bounce_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleBounceAction>>,
        >,
        /// If true, the rule will be enabled
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of Lambda Action blocks. Documented below.
        pub lambda_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleLambdaAction>>,
        >,
        /// The name of the rule
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of email addresses
        pub recipients: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the rule set
        pub rule_set_name: pulumi_wasm_rust::Output<String>,
        /// A list of S3 Action blocks. Documented below.
        pub s3_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleS3Action>>,
        >,
        /// If true, incoming emails will be scanned for spam and viruses
        pub scan_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of SNS Action blocks. Documented below.
        pub sns_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleSnsAction>>,
        >,
        /// A list of Stop Action blocks. Documented below.
        pub stop_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleStopAction>>,
        >,
        /// `Require` or `Optional`
        pub tls_policy: pulumi_wasm_rust::Output<String>,
        /// A list of WorkMail Action blocks. Documented below.
        pub workmail_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleWorkmailAction>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ReceiptRuleArgs) -> ReceiptRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let add_header_actions_binding = args.add_header_actions.get_inner();
        let after_binding = args.after.get_inner();
        let bounce_actions_binding = args.bounce_actions.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let lambda_actions_binding = args.lambda_actions.get_inner();
        let name_binding = args.name.get_inner();
        let recipients_binding = args.recipients.get_inner();
        let rule_set_name_binding = args.rule_set_name.get_inner();
        let s3_actions_binding = args.s3_actions.get_inner();
        let scan_enabled_binding = args.scan_enabled.get_inner();
        let sns_actions_binding = args.sns_actions.get_inner();
        let stop_actions_binding = args.stop_actions.get_inner();
        let tls_policy_binding = args.tls_policy.get_inner();
        let workmail_actions_binding = args.workmail_actions.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ses/receiptRule:ReceiptRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addHeaderActions".into(),
                    value: &add_header_actions_binding,
                },
                register_interface::ObjectField {
                    name: "after".into(),
                    value: &after_binding,
                },
                register_interface::ObjectField {
                    name: "bounceActions".into(),
                    value: &bounce_actions_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "lambdaActions".into(),
                    value: &lambda_actions_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recipients".into(),
                    value: &recipients_binding,
                },
                register_interface::ObjectField {
                    name: "ruleSetName".into(),
                    value: &rule_set_name_binding,
                },
                register_interface::ObjectField {
                    name: "s3Actions".into(),
                    value: &s3_actions_binding,
                },
                register_interface::ObjectField {
                    name: "scanEnabled".into(),
                    value: &scan_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "snsActions".into(),
                    value: &sns_actions_binding,
                },
                register_interface::ObjectField {
                    name: "stopActions".into(),
                    value: &stop_actions_binding,
                },
                register_interface::ObjectField {
                    name: "tlsPolicy".into(),
                    value: &tls_policy_binding,
                },
                register_interface::ObjectField {
                    name: "workmailActions".into(),
                    value: &workmail_actions_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addHeaderActions".into(),
                },
                register_interface::ResultField {
                    name: "after".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "bounceActions".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "lambdaActions".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "recipients".into(),
                },
                register_interface::ResultField {
                    name: "ruleSetName".into(),
                },
                register_interface::ResultField {
                    name: "s3Actions".into(),
                },
                register_interface::ResultField {
                    name: "scanEnabled".into(),
                },
                register_interface::ResultField {
                    name: "snsActions".into(),
                },
                register_interface::ResultField {
                    name: "stopActions".into(),
                },
                register_interface::ResultField {
                    name: "tlsPolicy".into(),
                },
                register_interface::ResultField {
                    name: "workmailActions".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReceiptRuleResult {
            add_header_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addHeaderActions").unwrap(),
            ),
            after: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("after").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bounce_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bounceActions").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            lambda_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lambdaActions").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            recipients: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recipients").unwrap(),
            ),
            rule_set_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleSetName").unwrap(),
            ),
            s3_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3Actions").unwrap(),
            ),
            scan_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scanEnabled").unwrap(),
            ),
            sns_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snsActions").unwrap(),
            ),
            stop_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stopActions").unwrap(),
            ),
            tls_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tlsPolicy").unwrap(),
            ),
            workmail_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workmailActions").unwrap(),
            ),
        }
    }
}
