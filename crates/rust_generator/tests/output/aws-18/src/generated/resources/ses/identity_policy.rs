/// Manages a SES Identity Policy. More information about SES Sending Authorization Policies can be found in the [SES Developer Guide](https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-policies.html).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleDomainIdentity:
///     type: aws:ses:DomainIdentity
///     name: example
///     properties:
///       domain: example.com
///   exampleIdentityPolicy:
///     type: aws:ses:IdentityPolicy
///     name: example
///     properties:
///       identity: ${exampleDomainIdentity.arn}
///       name: example
///       policy: ${example.json}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - SES:SendEmail
///               - SES:SendRawEmail
///             resources:
///               - ${exampleDomainIdentity.arn}
///             principals:
///               - identifiers:
///                   - '*'
///                 type: AWS
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SES Identity Policies using the identity and policy name, separated by a pipe character (`|`). For example:
///
/// ```sh
/// $ pulumi import aws:ses/identityPolicy:IdentityPolicy example 'example.com|example'
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod identity_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityPolicyArgs {
        /// Name or Amazon Resource Name (ARN) of the SES Identity.
        #[builder(into)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// JSON string of the policy.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IdentityPolicyResult {
        /// Name or Amazon Resource Name (ARN) of the SES Identity.
        pub identity: pulumi_gestalt_rust::Output<String>,
        /// Name of the policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// JSON string of the policy.
        pub policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IdentityPolicyArgs,
    ) -> IdentityPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identity_binding = args.identity.get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ses/identityPolicy:IdentityPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IdentityPolicyResult {
            identity: o.get_field("identity"),
            name: o.get_field("name"),
            policy: o.get_field("policy"),
        }
    }
}
