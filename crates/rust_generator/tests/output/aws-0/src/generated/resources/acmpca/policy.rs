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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// JSON-formatted IAM policy to attach to the specified private CA resource.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the private CA to associate with the policy.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// JSON-formatted IAM policy to attach to the specified private CA resource.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// ARN of the private CA to associate with the policy.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyArgs,
    ) -> PolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_binding = args.policy.get_output(context);
        let resource_arn_binding = args.resource_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:acmpca/policy:Policy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyResult {
            policy: o.get_field("policy"),
            resource_arn: o.get_field("resourceArn"),
        }
    }
}
