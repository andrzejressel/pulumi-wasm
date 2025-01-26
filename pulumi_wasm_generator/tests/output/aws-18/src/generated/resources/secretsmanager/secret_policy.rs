/// Provides a resource to manage AWS Secrets Manager secret policy.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   exampleSecret:
///     type: aws:secretsmanager:Secret
///     name: example
///     properties:
///       name: example
///   exampleSecretPolicy:
///     type: aws:secretsmanager:SecretPolicy
///     name: example
///     properties:
///       secretArn: ${exampleSecret.arn}
///       policy: ${example.json}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: EnableAnotherAWSAccountToReadTheSecret
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - arn:aws:iam::123456789012:root
///             actions:
///               - secretsmanager:GetSecretValue
///             resources:
///               - '*'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_secretsmanager_secret_policy` using the secret Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:secretsmanager/secretPolicy:SecretPolicy example arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456
/// ```
pub mod secret_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecretPolicyArgs {
        /// Makes an optional API call to Zelkova to validate the Resource Policy to prevent broad access to your secret.
        #[builder(into, default)]
        pub block_public_policy: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Valid JSON document representing a [resource policy](https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_resource-based-policies.html). Unlike `aws.secretsmanager.Secret`, where `policy` can be set to `"{}"` to delete the policy, `"{}"` is not a valid policy since `policy` is required.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::InputOrOutput<String>,
        /// Secret ARN.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub secret_arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SecretPolicyResult {
        /// Makes an optional API call to Zelkova to validate the Resource Policy to prevent broad access to your secret.
        pub block_public_policy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Valid JSON document representing a [resource policy](https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_resource-based-policies.html). Unlike `aws.secretsmanager.Secret`, where `policy` can be set to `"{}"` to delete the policy, `"{}"` is not a valid policy since `policy` is required.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// Secret ARN.
        ///
        /// The following arguments are optional:
        pub secret_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SecretPolicyArgs,
    ) -> SecretPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let block_public_policy_binding = args
            .block_public_policy
            .get_output(context)
            .get_inner();
        let policy_binding = args.policy.get_output(context).get_inner();
        let secret_arn_binding = args.secret_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:secretsmanager/secretPolicy:SecretPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "blockPublicPolicy".into(),
                    value: &block_public_policy_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "secretArn".into(),
                    value: &secret_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "blockPublicPolicy".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "secretArn".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SecretPolicyResult {
            block_public_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockPublicPolicy").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            secret_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretArn").unwrap(),
            ),
        }
    }
}
