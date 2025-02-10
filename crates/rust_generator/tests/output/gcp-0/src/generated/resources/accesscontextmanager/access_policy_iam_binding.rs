/// Three different resources help you manage your IAM policy for Access Context Manager (VPC Service Controls) AccessPolicy. Each of these resources serves a different use case:
///
/// * `gcp.accesscontextmanager.AccessPolicyIamPolicy`: Authoritative. Sets the IAM policy for the accesspolicy and replaces any existing policy already attached.
/// * `gcp.accesscontextmanager.AccessPolicyIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the accesspolicy are preserved.
/// * `gcp.accesscontextmanager.AccessPolicyIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the accesspolicy are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.accesscontextmanager.AccessPolicyIamPolicy`: Retrieves the IAM policy for the accesspolicy
///
/// > **Note:** `gcp.accesscontextmanager.AccessPolicyIamPolicy` **cannot** be used in conjunction with `gcp.accesscontextmanager.AccessPolicyIamBinding` and `gcp.accesscontextmanager.AccessPolicyIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.accesscontextmanager.AccessPolicyIamBinding` resources **can be** used in conjunction with `gcp.accesscontextmanager.AccessPolicyIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.accesscontextmanager.AccessPolicyIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:accesscontextmanager:AccessPolicyIamPolicy
///     properties:
///       name: ${["access-policy"].name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/accesscontextmanager.policyAdmin
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.accesscontextmanager.AccessPolicyIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = access_policy_iam_binding::create(
///         "binding",
///         AccessPolicyIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .name("${[\"access-policy\"].name}")
///             .role("roles/accesscontextmanager.policyAdmin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.accesscontextmanager.AccessPolicyIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = access_policy_iam_member::create(
///         "member",
///         AccessPolicyIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .name("${[\"access-policy\"].name}")
///             .role("roles/accesscontextmanager.policyAdmin")
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
/// # IAM policy for Access Context Manager (VPC Service Controls) AccessPolicy
/// Three different resources help you manage your IAM policy for Access Context Manager (VPC Service Controls) AccessPolicy. Each of these resources serves a different use case:
///
/// * `gcp.accesscontextmanager.AccessPolicyIamPolicy`: Authoritative. Sets the IAM policy for the accesspolicy and replaces any existing policy already attached.
/// * `gcp.accesscontextmanager.AccessPolicyIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the accesspolicy are preserved.
/// * `gcp.accesscontextmanager.AccessPolicyIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the accesspolicy are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.accesscontextmanager.AccessPolicyIamPolicy`: Retrieves the IAM policy for the accesspolicy
///
/// > **Note:** `gcp.accesscontextmanager.AccessPolicyIamPolicy` **cannot** be used in conjunction with `gcp.accesscontextmanager.AccessPolicyIamBinding` and `gcp.accesscontextmanager.AccessPolicyIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.accesscontextmanager.AccessPolicyIamBinding` resources **can be** used in conjunction with `gcp.accesscontextmanager.AccessPolicyIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.accesscontextmanager.AccessPolicyIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:accesscontextmanager:AccessPolicyIamPolicy
///     properties:
///       name: ${["access-policy"].name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/accesscontextmanager.policyAdmin
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.accesscontextmanager.AccessPolicyIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = access_policy_iam_binding::create(
///         "binding",
///         AccessPolicyIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .name("${[\"access-policy\"].name}")
///             .role("roles/accesscontextmanager.policyAdmin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.accesscontextmanager.AccessPolicyIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = access_policy_iam_member::create(
///         "member",
///         AccessPolicyIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .name("${[\"access-policy\"].name}")
///             .role("roles/accesscontextmanager.policyAdmin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * accessPolicies/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Access Context Manager (VPC Service Controls) accesspolicy IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/accessPolicyIamBinding:AccessPolicyIamBinding editor "accessPolicies/{{access_policy}} roles/accesscontextmanager.policyAdmin user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/accessPolicyIamBinding:AccessPolicyIamBinding editor "accessPolicies/{{access_policy}} roles/accesscontextmanager.policyAdmin"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/accessPolicyIamBinding:AccessPolicyIamBinding editor accessPolicies/{{access_policy}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_policy_iam_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessPolicyIamBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::accesscontextmanager::AccessPolicyIamBindingCondition,
            >,
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
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.accesscontextmanager.AccessPolicyIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccessPolicyIamBindingResult {
        pub condition: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::accesscontextmanager::AccessPolicyIamBindingCondition,
            >,
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
        /// Used to find the parent resource to bind the IAM policy to
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.accesscontextmanager.AccessPolicyIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessPolicyIamBindingArgs,
    ) -> AccessPolicyIamBindingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let members_binding = args.members.get_output(context);
        let name_binding = args.name.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/accessPolicyIamBinding:AccessPolicyIamBinding"
                .into(),
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
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: role_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccessPolicyIamBindingResult {
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            members: o.get_field("members"),
            name: o.get_field("name"),
            role: o.get_field("role"),
        }
    }
}
