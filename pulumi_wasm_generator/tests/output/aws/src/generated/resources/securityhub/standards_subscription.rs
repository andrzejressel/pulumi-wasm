/// Subscribes to a Security Hub standard.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:securityhub:Account
///   cis:
///     type: aws:securityhub:StandardsSubscription
///     properties:
///       standardsArn: arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0
///     options:
///       dependsOn:
///         - ${example}
///   pci321:
///     type: aws:securityhub:StandardsSubscription
///     name: pci_321
///     properties:
///       standardsArn: arn:aws:securityhub:${current.name}::standards/pci-dss/v/3.2.1
///     options:
///       dependsOn:
///         - ${example}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Security Hub standards subscriptions using the standards subscription ARN. For example:
///
/// ```sh
/// $ pulumi import aws:securityhub/standardsSubscription:StandardsSubscription cis arn:aws:securityhub:eu-west-1:123456789012:subscription/cis-aws-foundations-benchmark/v/1.2.0
/// ```
/// ```sh
/// $ pulumi import aws:securityhub/standardsSubscription:StandardsSubscription pci_321 arn:aws:securityhub:eu-west-1:123456789012:subscription/pci-dss/v/3.2.1
/// ```
/// ```sh
/// $ pulumi import aws:securityhub/standardsSubscription:StandardsSubscription nist_800_53_rev_5 arn:aws:securityhub:eu-west-1:123456789012:subscription/nist-800-53/v/5.0.0
/// ```
pub mod standards_subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StandardsSubscriptionArgs {
        /// The ARN of a standard - see below.
        ///
        /// Currently available standards (remember to replace `${var.partition}` and `${var.region}` as appropriate):
        ///
        /// | Name                                     | ARN                                                                                                          |
        /// |------------------------------------------|--------------------------------------------------------------------------------------------------------------|
        /// | AWS Foundational Security Best Practices | `arn:${var.partition}:securityhub:${var.region}::standards/aws-foundational-security-best-practices/v/1.0.0` |
        /// | AWS Resource Tagging Standard            | `arn:${var.partition}:securityhub:${var.region}::standards/aws-resource-tagging-standard/v/1.0.0`            |
        /// | CIS AWS Foundations Benchmark v1.2.0     | `arn:${var.partition}:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0`                           |
        /// | CIS AWS Foundations Benchmark v1.4.0     | `arn:${var.partition}:securityhub:${var.region}::standards/cis-aws-foundations-benchmark/v/1.4.0`            |
        /// | CIS AWS Foundations Benchmark v3.0.0     | `arn:${var.partition}:securityhub:${var.region}::standards/cis-aws-foundations-benchmark/v/3.0.0`            |
        /// | NIST SP 800-53 Rev. 5                    | `arn:${var.partition}:securityhub:${var.region}::standards/nist-800-53/v/5.0.0`                              |
        /// | PCI DSS                                  | `arn:${var.partition}:securityhub:${var.region}::standards/pci-dss/v/3.2.1`                                  |
        #[builder(into)]
        pub standards_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct StandardsSubscriptionResult {
        /// The ARN of a standard - see below.
        ///
        /// Currently available standards (remember to replace `${var.partition}` and `${var.region}` as appropriate):
        ///
        /// | Name                                     | ARN                                                                                                          |
        /// |------------------------------------------|--------------------------------------------------------------------------------------------------------------|
        /// | AWS Foundational Security Best Practices | `arn:${var.partition}:securityhub:${var.region}::standards/aws-foundational-security-best-practices/v/1.0.0` |
        /// | AWS Resource Tagging Standard            | `arn:${var.partition}:securityhub:${var.region}::standards/aws-resource-tagging-standard/v/1.0.0`            |
        /// | CIS AWS Foundations Benchmark v1.2.0     | `arn:${var.partition}:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0`                           |
        /// | CIS AWS Foundations Benchmark v1.4.0     | `arn:${var.partition}:securityhub:${var.region}::standards/cis-aws-foundations-benchmark/v/1.4.0`            |
        /// | CIS AWS Foundations Benchmark v3.0.0     | `arn:${var.partition}:securityhub:${var.region}::standards/cis-aws-foundations-benchmark/v/3.0.0`            |
        /// | NIST SP 800-53 Rev. 5                    | `arn:${var.partition}:securityhub:${var.region}::standards/nist-800-53/v/5.0.0`                              |
        /// | PCI DSS                                  | `arn:${var.partition}:securityhub:${var.region}::standards/pci-dss/v/3.2.1`                                  |
        pub standards_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: StandardsSubscriptionArgs,
    ) -> StandardsSubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let standards_arn_binding = args.standards_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securityhub/standardsSubscription:StandardsSubscription".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "standardsArn".into(),
                    value: &standards_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "standardsArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StandardsSubscriptionResult {
            standards_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("standardsArn").unwrap(),
            ),
        }
    }
}
