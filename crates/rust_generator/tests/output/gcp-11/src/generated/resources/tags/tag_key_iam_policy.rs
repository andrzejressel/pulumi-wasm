/// Three different resources help you manage your IAM policy for Tags TagKey. Each of these resources serves a different use case:
///
/// * `gcp.tags.TagKeyIamPolicy`: Authoritative. Sets the IAM policy for the tagkey and replaces any existing policy already attached.
/// * `gcp.tags.TagKeyIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the tagkey are preserved.
/// * `gcp.tags.TagKeyIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the tagkey are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.tags.TagKeyIamPolicy`: Retrieves the IAM policy for the tagkey
///
/// > **Note:** `gcp.tags.TagKeyIamPolicy` **cannot** be used in conjunction with `gcp.tags.TagKeyIamBinding` and `gcp.tags.TagKeyIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.tags.TagKeyIamBinding` resources **can be** used in conjunction with `gcp.tags.TagKeyIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.tags.TagKeyIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:tags:TagKeyIamPolicy
///     properties:
///       tagKey: ${key.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.tags.TagKeyIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = tag_key_iam_binding::create(
///         "binding",
///         TagKeyIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .role("roles/viewer")
///             .tag_key("${key.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.tags.TagKeyIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = tag_key_iam_member::create(
///         "member",
///         TagKeyIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .role("roles/viewer")
///             .tag_key("${key.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ## > **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
/// -
///
/// # IAM policy for Tags TagKey
/// Three different resources help you manage your IAM policy for Tags TagKey. Each of these resources serves a different use case:
///
/// * `gcp.tags.TagKeyIamPolicy`: Authoritative. Sets the IAM policy for the tagkey and replaces any existing policy already attached.
/// * `gcp.tags.TagKeyIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the tagkey are preserved.
/// * `gcp.tags.TagKeyIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the tagkey are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.tags.TagKeyIamPolicy`: Retrieves the IAM policy for the tagkey
///
/// > **Note:** `gcp.tags.TagKeyIamPolicy` **cannot** be used in conjunction with `gcp.tags.TagKeyIamBinding` and `gcp.tags.TagKeyIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.tags.TagKeyIamBinding` resources **can be** used in conjunction with `gcp.tags.TagKeyIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.tags.TagKeyIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:tags:TagKeyIamPolicy
///     properties:
///       tagKey: ${key.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.tags.TagKeyIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = tag_key_iam_binding::create(
///         "binding",
///         TagKeyIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .role("roles/viewer")
///             .tag_key("${key.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.tags.TagKeyIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = tag_key_iam_member::create(
///         "member",
///         TagKeyIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .role("roles/viewer")
///             .tag_key("${key.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * tagKeys/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Tags tagkey IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:tags/tagKeyIamPolicy:TagKeyIamPolicy editor "tagKeys/{{tag_key}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:tags/tagKeyIamPolicy:TagKeyIamPolicy editor "tagKeys/{{tag_key}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:tags/tagKeyIamPolicy:TagKeyIamPolicy editor tagKeys/{{tag_key}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tag_key_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagKeyIamPolicyArgs {
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub tag_key: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TagKeyIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub tag_key: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TagKeyIamPolicyArgs,
    ) -> TagKeyIamPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_data_binding = args.policy_data.get_output(context);
        let tag_key_binding = args.tag_key.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:tags/tagKeyIamPolicy:TagKeyIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: policy_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tagKey".into(),
                    value: tag_key_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TagKeyIamPolicyResult {
            etag: o.get_field("etag"),
            policy_data: o.get_field("policyData"),
            tag_key: o.get_field("tagKey"),
        }
    }
}
