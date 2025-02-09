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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod secret_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecretPolicyArgs {
        /// Makes an optional API call to Zelkova to validate the Resource Policy to prevent broad access to your secret.
        #[builder(into, default)]
        pub block_public_policy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Valid JSON document representing a [resource policy](https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_resource-based-policies.html). Unlike `aws.secretsmanager.Secret`, where `policy` can be set to `"{}"` to delete the policy, `"{}"` is not a valid policy since `policy` is required.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Secret ARN.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub secret_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SecretPolicyResult {
        /// Makes an optional API call to Zelkova to validate the Resource Policy to prevent broad access to your secret.
        pub block_public_policy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Valid JSON document representing a [resource policy](https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_resource-based-policies.html). Unlike `aws.secretsmanager.Secret`, where `policy` can be set to `"{}"` to delete the policy, `"{}"` is not a valid policy since `policy` is required.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// Secret ARN.
        ///
        /// The following arguments are optional:
        pub secret_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecretPolicyArgs,
    ) -> SecretPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let block_public_policy_binding = args.block_public_policy.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let secret_arn_binding = args.secret_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:secretsmanager/secretPolicy:SecretPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockPublicPolicy".into(),
                    value: block_public_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretArn".into(),
                    value: secret_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecretPolicyResult {
            block_public_policy: o.get_field("blockPublicPolicy"),
            policy: o.get_field("policy"),
            secret_arn: o.get_field("secretArn"),
        }
    }
}
