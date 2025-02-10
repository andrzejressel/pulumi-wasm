/// Three different resources help you manage your IAM policy for Tags TagValue. Each of these resources serves a different use case:
///
/// * `gcp.tags.TagValueIamPolicy`: Authoritative. Sets the IAM policy for the tagvalue and replaces any existing policy already attached.
/// * `gcp.tags.TagValueIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the tagvalue are preserved.
/// * `gcp.tags.TagValueIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the tagvalue are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.tags.TagValueIamPolicy`: Retrieves the IAM policy for the tagvalue
///
/// > **Note:** `gcp.tags.TagValueIamPolicy` **cannot** be used in conjunction with `gcp.tags.TagValueIamBinding` and `gcp.tags.TagValueIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.tags.TagValueIamBinding` resources **can be** used in conjunction with `gcp.tags.TagValueIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.tags.TagValueIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:tags:TagValueIamPolicy
///     properties:
///       tagValue: ${value.name}
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
/// ## gcp.tags.TagValueIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = tag_value_iam_binding::create(
///         "binding",
///         TagValueIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .role("roles/viewer")
///             .tag_value("${value.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.tags.TagValueIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = tag_value_iam_member::create(
///         "member",
///         TagValueIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .role("roles/viewer")
///             .tag_value("${value.name}")
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
/// # IAM policy for Tags TagValue
/// Three different resources help you manage your IAM policy for Tags TagValue. Each of these resources serves a different use case:
///
/// * `gcp.tags.TagValueIamPolicy`: Authoritative. Sets the IAM policy for the tagvalue and replaces any existing policy already attached.
/// * `gcp.tags.TagValueIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the tagvalue are preserved.
/// * `gcp.tags.TagValueIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the tagvalue are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.tags.TagValueIamPolicy`: Retrieves the IAM policy for the tagvalue
///
/// > **Note:** `gcp.tags.TagValueIamPolicy` **cannot** be used in conjunction with `gcp.tags.TagValueIamBinding` and `gcp.tags.TagValueIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.tags.TagValueIamBinding` resources **can be** used in conjunction with `gcp.tags.TagValueIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.tags.TagValueIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:tags:TagValueIamPolicy
///     properties:
///       tagValue: ${value.name}
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
/// ## gcp.tags.TagValueIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = tag_value_iam_binding::create(
///         "binding",
///         TagValueIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .role("roles/viewer")
///             .tag_value("${value.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.tags.TagValueIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = tag_value_iam_member::create(
///         "member",
///         TagValueIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .role("roles/viewer")
///             .tag_value("${value.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * tagValues/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Tags tagvalue IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:tags/tagValueIamPolicy:TagValueIamPolicy editor "tagValues/{{tag_value}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:tags/tagValueIamPolicy:TagValueIamPolicy editor "tagValues/{{tag_value}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:tags/tagValueIamPolicy:TagValueIamPolicy editor tagValues/{{tag_value}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tag_value_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagValueIamPolicyArgs {
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub tag_value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TagValueIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub tag_value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TagValueIamPolicyArgs,
    ) -> TagValueIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_data_binding = args.policy_data.get_output(context);
        let tag_value_binding = args.tag_value.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:tags/tagValueIamPolicy:TagValueIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: policy_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tagValue".into(),
                    value: tag_value_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TagValueIamPolicyResult {
            etag: o.get_field("etag"),
            policy_data: o.get_field("policyData"),
            tag_value: o.get_field("tagValue"),
        }
    }
}
