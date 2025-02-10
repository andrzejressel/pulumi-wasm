/// Three different resources help you manage your IAM policy for GKEHub Scope. Each of these resources serves a different use case:
///
/// * `gcp.gkehub.ScopeIamPolicy`: Authoritative. Sets the IAM policy for the scope and replaces any existing policy already attached.
/// * `gcp.gkehub.ScopeIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the scope are preserved.
/// * `gcp.gkehub.ScopeIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the scope are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.gkehub.ScopeIamPolicy`: Retrieves the IAM policy for the scope
///
/// > **Note:** `gcp.gkehub.ScopeIamPolicy` **cannot** be used in conjunction with `gcp.gkehub.ScopeIamBinding` and `gcp.gkehub.ScopeIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.gkehub.ScopeIamBinding` resources **can be** used in conjunction with `gcp.gkehub.ScopeIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.gkehub.ScopeIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:gkehub:ScopeIamPolicy
///     properties:
///       project: ${scope.project}
///       scopeId: ${scope.scopeId}
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
/// ## gcp.gkehub.ScopeIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = scope_iam_binding::create(
///         "binding",
///         ScopeIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("${scope.project}")
///             .role("roles/viewer")
///             .scope_id("${scope.scopeId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.gkehub.ScopeIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = scope_iam_member::create(
///         "member",
///         ScopeIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("${scope.project}")
///             .role("roles/viewer")
///             .scope_id("${scope.scopeId}")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for GKEHub Scope
/// Three different resources help you manage your IAM policy for GKEHub Scope. Each of these resources serves a different use case:
///
/// * `gcp.gkehub.ScopeIamPolicy`: Authoritative. Sets the IAM policy for the scope and replaces any existing policy already attached.
/// * `gcp.gkehub.ScopeIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the scope are preserved.
/// * `gcp.gkehub.ScopeIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the scope are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.gkehub.ScopeIamPolicy`: Retrieves the IAM policy for the scope
///
/// > **Note:** `gcp.gkehub.ScopeIamPolicy` **cannot** be used in conjunction with `gcp.gkehub.ScopeIamBinding` and `gcp.gkehub.ScopeIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.gkehub.ScopeIamBinding` resources **can be** used in conjunction with `gcp.gkehub.ScopeIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.gkehub.ScopeIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:gkehub:ScopeIamPolicy
///     properties:
///       project: ${scope.project}
///       scopeId: ${scope.scopeId}
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
/// ## gcp.gkehub.ScopeIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = scope_iam_binding::create(
///         "binding",
///         ScopeIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("${scope.project}")
///             .role("roles/viewer")
///             .scope_id("${scope.scopeId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.gkehub.ScopeIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = scope_iam_member::create(
///         "member",
///         ScopeIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("${scope.project}")
///             .role("roles/viewer")
///             .scope_id("${scope.scopeId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/locations/global/scopes/{{scope_id}}
///
/// * {{project}}/{{scope_id}}
///
/// * {{scope_id}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// GKEHub scope IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:gkehub/scopeIamPolicy:ScopeIamPolicy editor "projects/{{project}}/locations/global/scopes/{{scope_id}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:gkehub/scopeIamPolicy:ScopeIamPolicy editor "projects/{{project}}/locations/global/scopes/{{scope_id}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:gkehub/scopeIamPolicy:ScopeIamPolicy editor projects/{{project}}/locations/global/scopes/{{scope_id}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod scope_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScopeIamPolicyArgs {
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub scope_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ScopeIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        pub scope_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScopeIamPolicyArgs,
    ) -> ScopeIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_data_binding = args.policy_data.get_output(context);
        let project_binding = args.project.get_output(context);
        let scope_id_binding = args.scope_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:gkehub/scopeIamPolicy:ScopeIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: policy_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopeId".into(),
                    value: scope_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ScopeIamPolicyResult {
            etag: o.get_field("etag"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
            scope_id: o.get_field("scopeId"),
        }
    }
}
