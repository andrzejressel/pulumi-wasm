/// Three different resources help you manage your IAM policy for Security Command Center (SCC)v2 API OrganizationSource. Each of these resources serves a different use case:
///
/// * `gcp.securitycenter.V2OrganizationSourceIamPolicy`: Authoritative. Sets the IAM policy for the organizationsource and replaces any existing policy already attached.
/// * `gcp.securitycenter.V2OrganizationSourceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the organizationsource are preserved.
/// * `gcp.securitycenter.V2OrganizationSourceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the organizationsource are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.securitycenter.V2OrganizationSourceIamPolicy`: Retrieves the IAM policy for the organizationsource
///
/// > **Note:** `gcp.securitycenter.V2OrganizationSourceIamPolicy` **cannot** be used in conjunction with `gcp.securitycenter.V2OrganizationSourceIamBinding` and `gcp.securitycenter.V2OrganizationSourceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.securitycenter.V2OrganizationSourceIamBinding` resources **can be** used in conjunction with `gcp.securitycenter.V2OrganizationSourceIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.securitycenter.V2OrganizationSourceIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:securitycenter:V2OrganizationSourceIamPolicy
///     properties:
///       source: ${customSource.name}
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
/// ## gcp.securitycenter.V2OrganizationSourceIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = v_2_organization_source_iam_binding::create(
///         "binding",
///         V2OrganizationSourceIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .role("roles/viewer")
///             .source("${customSource.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.securitycenter.V2OrganizationSourceIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = v_2_organization_source_iam_member::create(
///         "member",
///         V2OrganizationSourceIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .role("roles/viewer")
///             .source("${customSource.name}")
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
/// # IAM policy for Security Command Center (SCC)v2 API OrganizationSource
/// Three different resources help you manage your IAM policy for Security Command Center (SCC)v2 API OrganizationSource. Each of these resources serves a different use case:
///
/// * `gcp.securitycenter.V2OrganizationSourceIamPolicy`: Authoritative. Sets the IAM policy for the organizationsource and replaces any existing policy already attached.
/// * `gcp.securitycenter.V2OrganizationSourceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the organizationsource are preserved.
/// * `gcp.securitycenter.V2OrganizationSourceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the organizationsource are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.securitycenter.V2OrganizationSourceIamPolicy`: Retrieves the IAM policy for the organizationsource
///
/// > **Note:** `gcp.securitycenter.V2OrganizationSourceIamPolicy` **cannot** be used in conjunction with `gcp.securitycenter.V2OrganizationSourceIamBinding` and `gcp.securitycenter.V2OrganizationSourceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.securitycenter.V2OrganizationSourceIamBinding` resources **can be** used in conjunction with `gcp.securitycenter.V2OrganizationSourceIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.securitycenter.V2OrganizationSourceIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:securitycenter:V2OrganizationSourceIamPolicy
///     properties:
///       source: ${customSource.name}
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
/// ## gcp.securitycenter.V2OrganizationSourceIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = v_2_organization_source_iam_binding::create(
///         "binding",
///         V2OrganizationSourceIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .role("roles/viewer")
///             .source("${customSource.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.securitycenter.V2OrganizationSourceIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = v_2_organization_source_iam_member::create(
///         "member",
///         V2OrganizationSourceIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .role("roles/viewer")
///             .source("${customSource.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * organizations/{{organization}}/sources/{{source}}
///
/// * {{organization}}/{{source}}
///
/// * {{source}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Security Command Center (SCC)v2 API organizationsource IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2OrganizationSourceIamMember:V2OrganizationSourceIamMember editor "organizations/{{organization}}/sources/{{source}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2OrganizationSourceIamMember:V2OrganizationSourceIamMember editor "organizations/{{organization}}/sources/{{source}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2OrganizationSourceIamMember:V2OrganizationSourceIamMember editor organizations/{{organization}}/sources/{{source}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod v_2_organization_source_iam_member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2OrganizationSourceIamMemberArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<
                super::super::types::securitycenter::V2OrganizationSourceIamMemberCondition,
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
        pub member: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub organization: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.securitycenter.V2OrganizationSourceIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub source: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct V2OrganizationSourceIamMemberResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<
                super::super::types::securitycenter::V2OrganizationSourceIamMemberCondition,
            >,
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
        pub member: pulumi_wasm_rust::Output<String>,
        pub organization: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.securitycenter.V2OrganizationSourceIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub source: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: V2OrganizationSourceIamMemberArgs,
    ) -> V2OrganizationSourceIamMemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let member_binding = args.member.get_inner();
        let organization_binding = args.organization.get_inner();
        let role_binding = args.role.get_inner();
        let source_binding = args.source.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securitycenter/v2OrganizationSourceIamMember:V2OrganizationSourceIamMember"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "member".into(),
                    value: &member_binding,
                },
                register_interface::ObjectField {
                    name: "organization".into(),
                    value: &organization_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
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
                    name: "member".into(),
                },
                register_interface::ResultField {
                    name: "organization".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
                },
                register_interface::ResultField {
                    name: "source".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        V2OrganizationSourceIamMemberResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            member: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("member").unwrap(),
            ),
            organization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organization").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
        }
    }
}
