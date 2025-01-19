/// Provides a resource to attach an AWS Organizations policy to an organization account, root, or unit.
///
/// ## Example Usage
///
/// ### Organization Account
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let account = policy_attachment::create(
///         "account",
///         PolicyAttachmentArgs::builder()
///             .policy_id("${example.id}")
///             .target_id("123456789012")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Organization Root
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let root = policy_attachment::create(
///         "root",
///         PolicyAttachmentArgs::builder()
///             .policy_id("${example.id}")
///             .target_id("${exampleAwsOrganizationsOrganization.roots[0].id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Organization Unit
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let unit = policy_attachment::create(
///         "unit",
///         PolicyAttachmentArgs::builder()
///             .policy_id("${example.id}")
///             .target_id("${exampleAwsOrganizationsOrganizationalUnit.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_organizations_policy_attachment` using the target ID and policy ID. For example:
///
/// With an account target:
///
/// ```sh
/// $ pulumi import aws:organizations/policyAttachment:PolicyAttachment account 123456789012:p-12345678
/// ```
pub mod policy_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyAttachmentArgs {
        /// The unique identifier (ID) of the policy that you want to attach to the target.
        #[builder(into)]
        pub policy_id: pulumi_wasm_rust::Output<String>,
        /// If set to `true`, destroy will **not** detach the policy and instead just remove the resource from state. This can be useful in situations where the attachment must be preserved to meet the AWS minimum requirement of 1 attached policy.
        #[builder(into, default)]
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// The unique identifier (ID) of the root, organizational unit, or account number that you want to attach the policy to.
        #[builder(into)]
        pub target_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyAttachmentResult {
        /// The unique identifier (ID) of the policy that you want to attach to the target.
        pub policy_id: pulumi_wasm_rust::Output<String>,
        /// If set to `true`, destroy will **not** detach the policy and instead just remove the resource from state. This can be useful in situations where the attachment must be preserved to meet the AWS minimum requirement of 1 attached policy.
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// The unique identifier (ID) of the root, organizational unit, or account number that you want to attach the policy to.
        pub target_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PolicyAttachmentArgs) -> PolicyAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_id_binding = args.policy_id.get_inner();
        let skip_destroy_binding = args.skip_destroy.get_inner();
        let target_id_binding = args.target_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:organizations/policyAttachment:PolicyAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policyId".into(),
                    value: &policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "targetId".into(),
                    value: &target_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "policyId".into(),
                },
                register_interface::ResultField {
                    name: "skipDestroy".into(),
                },
                register_interface::ResultField {
                    name: "targetId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PolicyAttachmentResult {
            policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyId").unwrap(),
            ),
            skip_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipDestroy").unwrap(),
            ),
            target_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetId").unwrap(),
            ),
        }
    }
}
