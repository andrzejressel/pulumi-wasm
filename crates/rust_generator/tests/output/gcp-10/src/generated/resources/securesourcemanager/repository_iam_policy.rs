/// Three different resources help you manage your IAM policy for Secure Source Manager Repository. Each of these resources serves a different use case:
///
/// * `gcp.securesourcemanager.RepositoryIamPolicy`: Authoritative. Sets the IAM policy for the repository and replaces any existing policy already attached.
/// * `gcp.securesourcemanager.RepositoryIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the repository are preserved.
/// * `gcp.securesourcemanager.RepositoryIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the repository are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.securesourcemanager.RepositoryIamPolicy`: Retrieves the IAM policy for the repository
///
/// > **Note:** `gcp.securesourcemanager.RepositoryIamPolicy` **cannot** be used in conjunction with `gcp.securesourcemanager.RepositoryIamBinding` and `gcp.securesourcemanager.RepositoryIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.securesourcemanager.RepositoryIamBinding` resources **can be** used in conjunction with `gcp.securesourcemanager.RepositoryIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.securesourcemanager.RepositoryIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:securesourcemanager:RepositoryIamPolicy
///     properties:
///       project: ${default.project}
///       location: ${default.location}
///       repositoryId: ${default.repositoryId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/securesourcemanager.repoAdmin
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.securesourcemanager.RepositoryIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = repository_iam_binding::create(
///         "binding",
///         RepositoryIamBindingArgs::builder()
///             .location("${default.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${default.project}")
///             .repository_id("${default.repositoryId}")
///             .role("roles/securesourcemanager.repoAdmin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.securesourcemanager.RepositoryIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = repository_iam_member::create(
///         "member",
///         RepositoryIamMemberArgs::builder()
///             .location("${default.location}")
///             .member("user:jane@example.com")
///             .project("${default.project}")
///             .repository_id("${default.repositoryId}")
///             .role("roles/securesourcemanager.repoAdmin")
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
/// # IAM policy for Secure Source Manager Repository
/// Three different resources help you manage your IAM policy for Secure Source Manager Repository. Each of these resources serves a different use case:
///
/// * `gcp.securesourcemanager.RepositoryIamPolicy`: Authoritative. Sets the IAM policy for the repository and replaces any existing policy already attached.
/// * `gcp.securesourcemanager.RepositoryIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the repository are preserved.
/// * `gcp.securesourcemanager.RepositoryIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the repository are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.securesourcemanager.RepositoryIamPolicy`: Retrieves the IAM policy for the repository
///
/// > **Note:** `gcp.securesourcemanager.RepositoryIamPolicy` **cannot** be used in conjunction with `gcp.securesourcemanager.RepositoryIamBinding` and `gcp.securesourcemanager.RepositoryIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.securesourcemanager.RepositoryIamBinding` resources **can be** used in conjunction with `gcp.securesourcemanager.RepositoryIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.securesourcemanager.RepositoryIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:securesourcemanager:RepositoryIamPolicy
///     properties:
///       project: ${default.project}
///       location: ${default.location}
///       repositoryId: ${default.repositoryId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/securesourcemanager.repoAdmin
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.securesourcemanager.RepositoryIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = repository_iam_binding::create(
///         "binding",
///         RepositoryIamBindingArgs::builder()
///             .location("${default.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${default.project}")
///             .repository_id("${default.repositoryId}")
///             .role("roles/securesourcemanager.repoAdmin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.securesourcemanager.RepositoryIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = repository_iam_member::create(
///         "member",
///         RepositoryIamMemberArgs::builder()
///             .location("${default.location}")
///             .member("user:jane@example.com")
///             .project("${default.project}")
///             .repository_id("${default.repositoryId}")
///             .role("roles/securesourcemanager.repoAdmin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/locations/{{location}}/repositories/{{repository_id}}
///
/// * {{project}}/{{location}}/{{repository_id}}
///
/// * {{location}}/{{repository_id}}
///
/// * {{repository_id}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Secure Source Manager repository IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/repositoryIamPolicy:RepositoryIamPolicy editor "projects/{{project}}/locations/{{location}}/repositories/{{repository_id}} roles/securesourcemanager.repoAdmin user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/repositoryIamPolicy:RepositoryIamPolicy editor "projects/{{project}}/locations/{{location}}/repositories/{{repository_id}} roles/securesourcemanager.repoAdmin"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/repositoryIamPolicy:RepositoryIamPolicy editor projects/{{project}}/locations/{{location}}/repositories/{{repository_id}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod repository_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryIamPolicyArgs {
        /// The location for the Repository.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID for the Repository.
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub repository_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The location for the Repository.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The ID for the Repository.
        /// Used to find the parent resource to bind the IAM policy to
        pub repository_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RepositoryIamPolicyArgs,
    ) -> RepositoryIamPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let policy_data_binding = args.policy_data.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let repository_id_binding = args.repository_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securesourcemanager/repositoryIamPolicy:RepositoryIamPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "repositoryId".into(),
                    value: &repository_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RepositoryIamPolicyResult {
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            policy_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyData"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            repository_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repositoryId"),
            ),
        }
    }
}
