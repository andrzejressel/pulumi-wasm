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
pub mod auth_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthPolicyArgs {
        /// The auth policy. The policy string in JSON must not contain newlines or blank lines.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The ID or Amazon Resource Name (ARN) of the service network or service for which the policy is created.
        #[builder(into)]
        pub resource_identifier: pulumi_wasm_rust::Output<String>,
        /// The state of the auth policy. The auth policy is only active when the auth type is set to `AWS_IAM`. If you provide a policy, then authentication and authorization decisions are made based on this policy and the client's IAM policy. If the Auth type is `NONE`, then, any auth policy you provide will remain inactive.
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AuthPolicyResult {
        /// The auth policy. The policy string in JSON must not contain newlines or blank lines.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The ID or Amazon Resource Name (ARN) of the service network or service for which the policy is created.
        pub resource_identifier: pulumi_wasm_rust::Output<String>,
        /// The state of the auth policy. The auth policy is only active when the auth type is set to `AWS_IAM`. If you provide a policy, then authentication and authorization decisions are made based on this policy and the client's IAM policy. If the Auth type is `NONE`, then, any auth policy you provide will remain inactive.
        pub state: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AuthPolicyArgs) -> AuthPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_binding = args.policy.get_inner();
        let resource_identifier_binding = args.resource_identifier.get_inner();
        let state_binding = args.state.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:vpclattice/authPolicy:AuthPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "resourceIdentifier".into(),
                    value: &resource_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "resourceIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AuthPolicyResult {
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            resource_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceIdentifier").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
        }
    }
}