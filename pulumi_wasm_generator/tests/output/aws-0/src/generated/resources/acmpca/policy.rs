/// Attaches a resource based policy to a private CA.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   examplePolicy:
///     type: aws:acmpca:Policy
///     name: example
///     properties:
///       resourceArn: ${exampleAwsAcmpcaCertificateAuthority.arn}
///       policy: ${example.json}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: '1'
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - ${current.accountId}
///             actions:
///               - acm-pca:DescribeCertificateAuthority
///               - acm-pca:GetCertificate
///               - acm-pca:GetCertificateAuthorityCertificate
///               - acm-pca:ListPermissions
///               - acm-pca:ListTags
///             resources:
///               - ${exampleAwsAcmpcaCertificateAuthority.arn}
///           - sid: '2'
///             effect: ${allow}
///             principals:
///               - type: AWS
///                 identifiers:
///                   - ${current.accountId}
///             actions:
///               - acm-pca:IssueCertificate
///             resources:
///               - ${exampleAwsAcmpcaCertificateAuthority.arn}
///             conditions:
///               - test: StringEquals
///                 variable: acm-pca:TemplateArn
///                 values:
///                   - arn:aws:acm-pca:::template/EndEntityCertificate/V1
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_acmpca_policy` using the `resource_arn` value. For example:
///
/// ```sh
/// $ pulumi import aws:acmpca/policy:Policy example arn:aws:acm-pca:us-east-1:123456789012:certificate-authority/12345678-1234-1234-1234-123456789012
/// ```
pub mod policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// JSON-formatted IAM policy to attach to the specified private CA resource.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::InputOrOutput<String>,
        /// ARN of the private CA to associate with the policy.
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// JSON-formatted IAM policy to attach to the specified private CA resource.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// ARN of the private CA to associate with the policy.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PolicyArgs,
    ) -> PolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_binding = args.policy.get_output(context).get_inner();
        let resource_arn_binding = args.resource_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:acmpca/policy:Policy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PolicyResult {
            policy: pulumi_wasm_rust::__private::into_domain(o.extract_field("policy")),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceArn"),
            ),
        }
    }
}
