/// Provides a resource to attach an AWS Organizations policy to an organization account, root, or unit.
///
/// ## Example Usage
///
/// ### Organization Account
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyAttachmentArgs {
        /// The unique identifier (ID) of the policy that you want to attach to the target.
        #[builder(into)]
        pub policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If set to `true`, destroy will **not** detach the policy and instead just remove the resource from state. This can be useful in situations where the attachment must be preserved to meet the AWS minimum requirement of 1 attached policy.
        #[builder(into, default)]
        pub skip_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The unique identifier (ID) of the root, organizational unit, or account number that you want to attach the policy to.
        #[builder(into)]
        pub target_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyAttachmentResult {
        /// The unique identifier (ID) of the policy that you want to attach to the target.
        pub policy_id: pulumi_gestalt_rust::Output<String>,
        /// If set to `true`, destroy will **not** detach the policy and instead just remove the resource from state. This can be useful in situations where the attachment must be preserved to meet the AWS minimum requirement of 1 attached policy.
        pub skip_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The unique identifier (ID) of the root, organizational unit, or account number that you want to attach the policy to.
        pub target_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyAttachmentArgs,
    ) -> PolicyAttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_id_binding = args.policy_id.get_output(context);
        let skip_destroy_binding = args.skip_destroy.get_output(context);
        let target_id_binding = args.target_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:organizations/policyAttachment:PolicyAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyId".into(),
                    value: policy_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipDestroy".into(),
                    value: skip_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetId".into(),
                    value: target_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyAttachmentResult {
            policy_id: o.get_field("policyId"),
            skip_destroy: o.get_field("skipDestroy"),
            target_id: o.get_field("targetId"),
        }
    }
}
