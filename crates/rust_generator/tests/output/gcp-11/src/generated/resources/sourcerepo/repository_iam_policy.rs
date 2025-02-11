/// Three different resources help you manage your IAM policy for Cloud Source Repositories Repository. Each of these resources serves a different use case:
///
/// * `gcp.sourcerepo.RepositoryIamPolicy`: Authoritative. Sets the IAM policy for the repository and replaces any existing policy already attached.
/// * `gcp.sourcerepo.RepositoryIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the repository are preserved.
/// * `gcp.sourcerepo.RepositoryIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the repository are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.sourcerepo.RepositoryIamPolicy`: Retrieves the IAM policy for the repository
///
/// > **Note:** `gcp.sourcerepo.RepositoryIamPolicy` **cannot** be used in conjunction with `gcp.sourcerepo.RepositoryIamBinding` and `gcp.sourcerepo.RepositoryIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.sourcerepo.RepositoryIamBinding` resources **can be** used in conjunction with `gcp.sourcerepo.RepositoryIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.sourcerepo.RepositoryIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:sourcerepo:RepositoryIamPolicy
///     properties:
///       project: ${["my-repo"].project}
///       repository: ${["my-repo"].name}
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
/// ## gcp.sourcerepo.RepositoryIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = repository_iam_binding::create(
///         "binding",
///         RepositoryIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"my-repo\"].project}")
///             .repository("${[\"my-repo\"].name}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.sourcerepo.RepositoryIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = repository_iam_member::create(
///         "member",
///         RepositoryIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("${[\"my-repo\"].project}")
///             .repository("${[\"my-repo\"].name}")
///             .role("roles/viewer")
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
/// # IAM policy for Cloud Source Repositories Repository
/// Three different resources help you manage your IAM policy for Cloud Source Repositories Repository. Each of these resources serves a different use case:
///
/// * `gcp.sourcerepo.RepositoryIamPolicy`: Authoritative. Sets the IAM policy for the repository and replaces any existing policy already attached.
/// * `gcp.sourcerepo.RepositoryIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the repository are preserved.
/// * `gcp.sourcerepo.RepositoryIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the repository are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.sourcerepo.RepositoryIamPolicy`: Retrieves the IAM policy for the repository
///
/// > **Note:** `gcp.sourcerepo.RepositoryIamPolicy` **cannot** be used in conjunction with `gcp.sourcerepo.RepositoryIamBinding` and `gcp.sourcerepo.RepositoryIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.sourcerepo.RepositoryIamBinding` resources **can be** used in conjunction with `gcp.sourcerepo.RepositoryIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.sourcerepo.RepositoryIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:sourcerepo:RepositoryIamPolicy
///     properties:
///       project: ${["my-repo"].project}
///       repository: ${["my-repo"].name}
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
/// ## gcp.sourcerepo.RepositoryIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = repository_iam_binding::create(
///         "binding",
///         RepositoryIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"my-repo\"].project}")
///             .repository("${[\"my-repo\"].name}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.sourcerepo.RepositoryIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = repository_iam_member::create(
///         "member",
///         RepositoryIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("${[\"my-repo\"].project}")
///             .repository("${[\"my-repo\"].name}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/repos/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Cloud Source Repositories repository IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:sourcerepo/repositoryIamPolicy:RepositoryIamPolicy editor "projects/{{project}}/repos/{{repository}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:sourcerepo/repositoryIamPolicy:RepositoryIamPolicy editor "projects/{{project}}/repos/{{repository}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:sourcerepo/repositoryIamPolicy:RepositoryIamPolicy editor projects/{{project}}/repos/{{repository}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod repository_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryIamPolicyArgs {
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub repository: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub repository: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryIamPolicyArgs,
    ) -> RepositoryIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_data_binding = args.policy_data.get_output(context);
        let project_binding = args.project.get_output(context);
        let repository_binding = args.repository.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:sourcerepo/repositoryIamPolicy:RepositoryIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repository".into(),
                    value: &repository_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RepositoryIamPolicyResult {
            etag: o.get_field("etag"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
            repository: o.get_field("repository"),
        }
    }
}
