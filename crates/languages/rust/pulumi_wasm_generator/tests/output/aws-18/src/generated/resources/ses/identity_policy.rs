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
pub mod identity_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityPolicyArgs {
        /// Name or Amazon Resource Name (ARN) of the SES Identity.
        #[builder(into)]
        pub identity: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the policy.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// JSON string of the policy.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IdentityPolicyResult {
        /// Name or Amazon Resource Name (ARN) of the SES Identity.
        pub identity: pulumi_wasm_rust::Output<String>,
        /// Name of the policy.
        pub name: pulumi_wasm_rust::Output<String>,
        /// JSON string of the policy.
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IdentityPolicyArgs,
    ) -> IdentityPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_binding = args.identity.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let policy_binding = args.policy.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ses/identityPolicy:IdentityPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IdentityPolicyResult {
            identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            policy: pulumi_wasm_rust::__private::into_domain(o.extract_field("policy")),
        }
    }
}
