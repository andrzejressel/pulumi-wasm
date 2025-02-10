/// Attaches a policy to a KMS Key.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       description: example
///   exampleKeyPolicy:
///     type: aws:kms:KeyPolicy
///     name: example
///     properties:
///       keyId: ${example.id}
///       policy:
///         fn::toJSON:
///           Id: example
///           Statement:
///             - Action: kms:*
///               Effect: Allow
///               Principal:
///                 AWS: '*'
///               Resource: '*'
///               Sid: Enable IAM User Permissions
///           Version: 2012-10-17
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import KMS Key Policies using the `key_id`. For example:
///
/// ```sh
/// $ pulumi import aws:kms/keyPolicy:KeyPolicy a 1234abcd-12ab-34cd-56ef-1234567890ab
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod key_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyPolicyArgs {
        /// A flag to indicate whether to bypass the key policy lockout safety check.
        /// Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately. If this value is set, and the resource is destroyed, a warning will be shown, and the resource will be removed from state.
        /// For more information, refer to the scenario in the [Default Key Policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam) section in the _AWS Key Management Service Developer Guide_.
        #[builder(into, default)]
        pub bypass_policy_lockout_safety_check: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The ID of the KMS Key to attach the policy.
        #[builder(into)]
        pub key_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A valid policy JSON document. Although this is a key policy, not an IAM policy, an `aws.iam.getPolicyDocument`, in the form that designates a principal, can be used. For more information about building policy documents, see the AWS IAM Policy Document Guide.
        ///
        /// > **NOTE:** Note: All KMS keys must have a key policy. If a key policy is not specified, or this resource is destroyed, AWS gives the KMS key a [default key policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default) that gives all principals in the owning account unlimited access to all KMS operations for the key. This default key policy effectively delegates all access control to IAM policies and KMS grants.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct KeyPolicyResult {
        /// A flag to indicate whether to bypass the key policy lockout safety check.
        /// Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately. If this value is set, and the resource is destroyed, a warning will be shown, and the resource will be removed from state.
        /// For more information, refer to the scenario in the [Default Key Policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam) section in the _AWS Key Management Service Developer Guide_.
        pub bypass_policy_lockout_safety_check: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The ID of the KMS Key to attach the policy.
        pub key_id: pulumi_gestalt_rust::Output<String>,
        /// A valid policy JSON document. Although this is a key policy, not an IAM policy, an `aws.iam.getPolicyDocument`, in the form that designates a principal, can be used. For more information about building policy documents, see the AWS IAM Policy Document Guide.
        ///
        /// > **NOTE:** Note: All KMS keys must have a key policy. If a key policy is not specified, or this resource is destroyed, AWS gives the KMS key a [default key policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default) that gives all principals in the owning account unlimited access to all KMS operations for the key. This default key policy effectively delegates all access control to IAM policies and KMS grants.
        pub policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeyPolicyArgs,
    ) -> KeyPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bypass_policy_lockout_safety_check_binding = args
            .bypass_policy_lockout_safety_check
            .get_output(context);
        let key_id_binding = args.key_id.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kms/keyPolicy:KeyPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bypassPolicyLockoutSafetyCheck".into(),
                    value: bypass_policy_lockout_safety_check_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyId".into(),
                    value: key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        KeyPolicyResult {
            bypass_policy_lockout_safety_check: o
                .get_field("bypassPolicyLockoutSafetyCheck"),
            key_id: o.get_field("keyId"),
            policy: o.get_field("policy"),
        }
    }
}
