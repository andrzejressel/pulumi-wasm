/// Resource for managing an AWS VPC Lattice Auth Policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:vpclattice:Service
///     properties:
///       name: example-vpclattice-service
///       authType: AWS_IAM
///       customDomainName: example.com
///   exampleAuthPolicy:
///     type: aws:vpclattice:AuthPolicy
///     name: example
///     properties:
///       resourceIdentifier: ${example.arn}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: '*'
///               Effect: Allow
///               Principal: '*'
///               Resource: '*'
///               Condition:
///                 StringNotEqualsIgnoreCase:
///                   aws:PrincipalType: anonymous
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Lattice Auth Policy using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:vpclattice/authPolicy:AuthPolicy example abcd-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod auth_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthPolicyArgs {
        /// The auth policy. The policy string in JSON must not contain newlines or blank lines.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID or Amazon Resource Name (ARN) of the service network or service for which the policy is created.
        #[builder(into)]
        pub resource_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The state of the auth policy. The auth policy is only active when the auth type is set to `AWS_IAM`. If you provide a policy, then authentication and authorization decisions are made based on this policy and the client's IAM policy. If the Auth type is `NONE`, then, any auth policy you provide will remain inactive.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AuthPolicyResult {
        /// The auth policy. The policy string in JSON must not contain newlines or blank lines.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The ID or Amazon Resource Name (ARN) of the service network or service for which the policy is created.
        pub resource_identifier: pulumi_gestalt_rust::Output<String>,
        /// The state of the auth policy. The auth policy is only active when the auth type is set to `AWS_IAM`. If you provide a policy, then authentication and authorization decisions are made based on this policy and the client's IAM policy. If the Auth type is `NONE`, then, any auth policy you provide will remain inactive.
        pub state: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthPolicyArgs,
    ) -> AuthPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_binding = args.policy.get_output(context);
        let resource_identifier_binding = args.resource_identifier.get_output(context);
        let state_binding = args.state.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:vpclattice/authPolicy:AuthPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceIdentifier".into(),
                    value: &resource_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: &state_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AuthPolicyResult {
            policy: o.get_field("policy"),
            resource_identifier: o.get_field("resourceIdentifier"),
            state: o.get_field("state"),
        }
    }
}
