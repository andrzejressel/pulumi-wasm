/// Manages a SES Identity Policy. More information about SES Sending Authorization Policies can be found in the [SES Developer Guide](https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-policies.html).
///
/// ## Example Usage
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
///                     GetPolicyDocumentStatement::builder().actions(vec!["SES:SendEmail",
///                     "SES:SendRawEmail",])
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["*",]). type ("AWS").build_struct(),])
///                     .resources(vec!["${exampleDomainIdentity.arn}",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleDomainIdentity = domain_identity::create(
///         "exampleDomainIdentity",
///         DomainIdentityArgs::builder().domain("example.com").build_struct(),
///     );
///     let exampleIdentityPolicy = identity_policy::create(
///         "exampleIdentityPolicy",
///         IdentityPolicyArgs::builder()
///             .identity("${exampleDomainIdentity.arn}")
///             .name("example")
///             .policy("${example.json}")
///             .build_struct(),
///     );
/// }
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityPolicyArgs {
        /// Name or Amazon Resource Name (ARN) of the SES Identity.
        #[builder(into)]
        pub identity: pulumi_wasm_rust::Output<String>,
        /// Name of the policy.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// JSON string of the policy.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
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
    pub fn create(name: &str, args: IdentityPolicyArgs) -> IdentityPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_binding = args.identity.get_inner();
        let name_binding = args.name.get_inner();
        let policy_binding = args.policy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ses/identityPolicy:IdentityPolicy".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IdentityPolicyResult {
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
        }
    }
}