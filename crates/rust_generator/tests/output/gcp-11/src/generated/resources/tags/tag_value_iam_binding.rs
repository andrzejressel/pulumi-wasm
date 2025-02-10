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
/// $ pulumi import gcp:tags/tagValueIamBinding:TagValueIamBinding editor "tagValues/{{tag_value}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:tags/tagValueIamBinding:TagValueIamBinding editor "tagValues/{{tag_value}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:tags/tagValueIamBinding:TagValueIamBinding editor tagValues/{{tag_value}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tag_value_iam_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagValueIamBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::tags::TagValueIamBindingCondition>,
        >,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        #[builder(into)]
        pub members: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The role that should be applied. Only one
        /// `gcp.tags.TagValueIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub tag_value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TagValueIamBindingResult {
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::tags::TagValueIamBindingCondition>,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        pub members: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The role that should be applied. Only one
        /// `gcp.tags.TagValueIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_gestalt_rust::Output<String>,
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
        args: TagValueIamBindingArgs,
    ) -> TagValueIamBindingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let members_binding = args.members.get_output(context);
        let role_binding = args.role.get_output(context);
        let tag_value_binding = args.tag_value.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:tags/tagValueIamBinding:TagValueIamBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: condition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "members".into(),
                    value: members_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tagValue".into(),
                    value: tag_value_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TagValueIamBindingResult {
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            members: o.get_field("members"),
            role: o.get_field("role"),
            tag_value: o.get_field("tagValue"),
        }
    }
}
