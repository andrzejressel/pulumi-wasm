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
/// $ pulumi import gcp:accesscontextmanager/accessPolicyIamPolicy:AccessPolicyIamPolicy editor "accessPolicies/{{access_policy}} roles/accesscontextmanager.policyAdmin user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/accessPolicyIamPolicy:AccessPolicyIamPolicy editor "accessPolicies/{{access_policy}} roles/accesscontextmanager.policyAdmin"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/accessPolicyIamPolicy:AccessPolicyIamPolicy editor accessPolicies/{{access_policy}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_policy_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessPolicyIamPolicyArgs {
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccessPolicyIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessPolicyIamPolicyArgs,
    ) -> AccessPolicyIamPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let policy_data_binding = args.policy_data.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/accessPolicyIamPolicy:AccessPolicyIamPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: policy_data_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccessPolicyIamPolicyResult {
            etag: o.get_field("etag"),
            name: o.get_field("name"),
            policy_data: o.get_field("policyData"),
        }
    }
}
