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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// $ pulumi import gcp:gkehub/scopeIamBinding:ScopeIamBinding editor "projects/{{project}}/locations/global/scopes/{{scope_id}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:gkehub/scopeIamBinding:ScopeIamBinding editor "projects/{{project}}/locations/global/scopes/{{scope_id}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:gkehub/scopeIamBinding:ScopeIamBinding editor projects/{{project}}/locations/global/scopes/{{scope_id}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod scope_iam_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScopeIamBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::gkehub::ScopeIamBindingCondition>,
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
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.gkehub.ScopeIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub scope_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ScopeIamBindingResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::gkehub::ScopeIamBindingCondition>,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
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
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.gkehub.ScopeIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
        pub scope_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ScopeIamBindingArgs) -> ScopeIamBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let members_binding = args.members.get_inner();
        let project_binding = args.project.get_inner();
        let role_binding = args.role.get_inner();
        let scope_id_binding = args.scope_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gkehub/scopeIamBinding:ScopeIamBinding".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "members".into(),
                    value: &members_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
                register_interface::ObjectField {
                    name: "scopeId".into(),
                    value: &scope_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "condition".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "members".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
                },
                register_interface::ResultField {
                    name: "scopeId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ScopeIamBindingResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("members").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
            scope_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopeId").unwrap(),
            ),
        }
    }
}
