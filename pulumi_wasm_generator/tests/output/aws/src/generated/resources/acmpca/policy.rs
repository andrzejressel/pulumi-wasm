/// Attaches a resource based policy to a private CA.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["acm-pca:DescribeCertificateAuthority",
///                     "acm-pca:GetCertificate",
///                     "acm-pca:GetCertificateAuthorityCertificate",
///                     "acm-pca:ListPermissions", "acm-pca:ListTags",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["${current.accountId}",]). type ("AWS")
///                     .build_struct(),])
///                     .resources(vec!["${exampleAwsAcmpcaCertificateAuthority.arn}",])
///                     .sid("1").build_struct(), GetPolicyDocumentStatement::builder()
///                     .actions(vec!["acm-pca:IssueCertificate",])
///                     .conditions(vec![GetPolicyDocumentStatementCondition::builder()
///                     .test("StringEquals")
///                     .values(vec!["arn:aws:acm-pca:::template/EndEntityCertificate/V1",])
///                     .variable("acm-pca:TemplateArn").build_struct(),]).effect("${allow}")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["${current.accountId}",]). type ("AWS")
///                     .build_struct(),])
///                     .resources(vec!["${exampleAwsAcmpcaCertificateAuthority.arn}",])
///                     .sid("2").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let examplePolicy = policy::create(
///         "examplePolicy",
///         PolicyArgs::builder()
///             .policy("${example.json}")
///             .resource_arn("${exampleAwsAcmpcaCertificateAuthority.arn}")
///             .build_struct(),
///     );
/// }
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// JSON-formatted IAM policy to attach to the specified private CA resource.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
        /// ARN of the private CA to associate with the policy.
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::Output<String>,
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
    pub fn create(name: &str, args: PolicyArgs) -> PolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_binding = args.policy.get_inner();
        let resource_arn_binding = args.resource_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:acmpca/policy:Policy".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PolicyResult {
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
        }
    }
}
