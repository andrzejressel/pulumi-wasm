/// Three different resources help you manage your IAM policy for Artifact Registry Repository. Each of these resources serves a different use case:
///
/// * `gcp.artifactregistry.RepositoryIamPolicy`: Authoritative. Sets the IAM policy for the repository and replaces any existing policy already attached.
/// * `gcp.artifactregistry.RepositoryIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the repository are preserved.
/// * `gcp.artifactregistry.RepositoryIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the repository are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.artifactregistry.RepositoryIamPolicy`: Retrieves the IAM policy for the repository
///
/// > **Note:** `gcp.artifactregistry.RepositoryIamPolicy` **cannot** be used in conjunction with `gcp.artifactregistry.RepositoryIamBinding` and `gcp.artifactregistry.RepositoryIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.artifactregistry.RepositoryIamBinding` resources **can be** used in conjunction with `gcp.artifactregistry.RepositoryIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.artifactregistry.RepositoryIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:artifactregistry:RepositoryIamPolicy
///     properties:
///       project: ${["my-repo"].project}
///       location: ${["my-repo"].location}
///       repository: ${["my-repo"].name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/artifactregistry.reader
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.artifactregistry.RepositoryIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = repository_iam_binding::create(
///         "binding",
///         RepositoryIamBindingArgs::builder()
///             .location("${[\"my-repo\"].location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"my-repo\"].project}")
///             .repository("${[\"my-repo\"].name}")
///             .role("roles/artifactregistry.reader")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.artifactregistry.RepositoryIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = repository_iam_member::create(
///         "member",
///         RepositoryIamMemberArgs::builder()
///             .location("${[\"my-repo\"].location}")
///             .member("user:jane@example.com")
///             .project("${[\"my-repo\"].project}")
///             .repository("${[\"my-repo\"].name}")
///             .role("roles/artifactregistry.reader")
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
/// # IAM policy for Artifact Registry Repository
/// Three different resources help you manage your IAM policy for Artifact Registry Repository. Each of these resources serves a different use case:
///
/// * `gcp.artifactregistry.RepositoryIamPolicy`: Authoritative. Sets the IAM policy for the repository and replaces any existing policy already attached.
/// * `gcp.artifactregistry.RepositoryIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the repository are preserved.
/// * `gcp.artifactregistry.RepositoryIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the repository are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.artifactregistry.RepositoryIamPolicy`: Retrieves the IAM policy for the repository
///
/// > **Note:** `gcp.artifactregistry.RepositoryIamPolicy` **cannot** be used in conjunction with `gcp.artifactregistry.RepositoryIamBinding` and `gcp.artifactregistry.RepositoryIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.artifactregistry.RepositoryIamBinding` resources **can be** used in conjunction with `gcp.artifactregistry.RepositoryIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.artifactregistry.RepositoryIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:artifactregistry:RepositoryIamPolicy
///     properties:
///       project: ${["my-repo"].project}
///       location: ${["my-repo"].location}
///       repository: ${["my-repo"].name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/artifactregistry.reader
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.artifactregistry.RepositoryIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = repository_iam_binding::create(
///         "binding",
///         RepositoryIamBindingArgs::builder()
///             .location("${[\"my-repo\"].location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"my-repo\"].project}")
///             .repository("${[\"my-repo\"].name}")
///             .role("roles/artifactregistry.reader")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.artifactregistry.RepositoryIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = repository_iam_member::create(
///         "member",
///         RepositoryIamMemberArgs::builder()
///             .location("${[\"my-repo\"].location}")
///             .member("user:jane@example.com")
///             .project("${[\"my-repo\"].project}")
///             .repository("${[\"my-repo\"].name}")
///             .role("roles/artifactregistry.reader")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/locations/{{location}}/repositories/{{repository}}
///
/// * {{project}}/{{location}}/{{repository}}
///
/// * {{location}}/{{repository}}
///
/// * {{repository}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Artifact Registry repository IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:artifactregistry/repositoryIamBinding:RepositoryIamBinding editor "projects/{{project}}/locations/{{location}}/repositories/{{repository}} roles/artifactregistry.reader user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:artifactregistry/repositoryIamBinding:RepositoryIamBinding editor "projects/{{project}}/locations/{{location}}/repositories/{{repository}} roles/artifactregistry.reader"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:artifactregistry/repositoryIamBinding:RepositoryIamBinding editor projects/{{project}}/locations/{{location}}/repositories/{{repository}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod repository_iam_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryIamBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::artifactregistry::RepositoryIamBindingCondition>,
        >,
        /// The name of the repository's location. In addition to specific regions,
        /// special values for multi-region locations are `asia`, `europe`, and `us`.
        /// See [here](https://cloud.google.com/artifact-registry/docs/repositories/repo-locations),
        /// or use the
        /// gcp.artifactregistry.getLocations
        /// data source for possible values. Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
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
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub repository: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.artifactregistry.RepositoryIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryIamBindingResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::artifactregistry::RepositoryIamBindingCondition>,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The name of the repository's location. In addition to specific regions,
        /// special values for multi-region locations are `asia`, `europe`, and `us`.
        /// See [here](https://cloud.google.com/artifact-registry/docs/repositories/repo-locations),
        /// or use the
        /// gcp.artifactregistry.getLocations
        /// data source for possible values. Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        pub location: pulumi_wasm_rust::Output<String>,
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
        /// Used to find the parent resource to bind the IAM policy to
        pub repository: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.artifactregistry.RepositoryIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RepositoryIamBindingArgs,
    ) -> RepositoryIamBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let location_binding = args.location.get_inner();
        let members_binding = args.members.get_inner();
        let project_binding = args.project.get_inner();
        let repository_binding = args.repository.get_inner();
        let role_binding = args.role.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:artifactregistry/repositoryIamBinding:RepositoryIamBinding"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "repository".into(),
                    value: &repository_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
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
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "members".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "repository".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RepositoryIamBindingResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("members").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            repository: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repository").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
        }
    }
}
