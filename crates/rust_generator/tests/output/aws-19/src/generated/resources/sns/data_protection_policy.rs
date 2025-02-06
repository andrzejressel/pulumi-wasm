/// Provides an SNS data protection topic policy resource
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:sns:Topic
///     properties:
///       name: example
///   exampleDataProtectionPolicy:
///     type: aws:sns:DataProtectionPolicy
///     name: example
///     properties:
///       arn: ${example.arn}
///       policy:
///         fn::toJSON:
///           Description: Example data protection policy
///           Name: __example_data_protection_policy
///           Statement:
///             - DataDirection: Inbound
///               DataIdentifier:
///                 - arn:aws:dataprotection::aws:data-identifier/EmailAddress
///               Operation:
///                 Deny: {}
///               Principal:
///                 - '*'
///               Sid: __deny_statement_11ba9d96
///           Version: 2021-06-01
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SNS Data Protection Topic Policy using the topic ARN. For example:
///
/// ```sh
/// $ pulumi import aws:sns/dataProtectionPolicy:DataProtectionPolicy example arn:aws:sns:us-west-2:123456789012:example
/// ```
pub mod data_protection_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataProtectionPolicyArgs {
        /// The ARN of the SNS topic
        #[builder(into)]
        pub arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The fully-formed AWS policy as JSON. For more information about building AWS IAM policy documents with this provider, see the AWS IAM Policy Document Guide.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DataProtectionPolicyResult {
        /// The ARN of the SNS topic
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The fully-formed AWS policy as JSON. For more information about building AWS IAM policy documents with this provider, see the AWS IAM Policy Document Guide.
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DataProtectionPolicyArgs,
    ) -> DataProtectionPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let policy_binding = args.policy.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sns/dataProtectionPolicy:DataProtectionPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DataProtectionPolicyResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            policy: pulumi_wasm_rust::__private::into_domain(o.extract_field("policy")),
        }
    }
}
