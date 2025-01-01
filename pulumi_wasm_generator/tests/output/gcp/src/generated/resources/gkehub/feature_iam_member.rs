/// Three different resources help you manage your IAM policy for GKEHub Feature. Each of these resources serves a different use case:
///
/// * `gcp.gkehub.FeatureIamPolicy`: Authoritative. Sets the IAM policy for the feature and replaces any existing policy already attached.
/// * `gcp.gkehub.FeatureIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the feature are preserved.
/// * `gcp.gkehub.FeatureIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the feature are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.gkehub.FeatureIamPolicy`: Retrieves the IAM policy for the feature
///
/// > **Note:** `gcp.gkehub.FeatureIamPolicy` **cannot** be used in conjunction with `gcp.gkehub.FeatureIamBinding` and `gcp.gkehub.FeatureIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.gkehub.FeatureIamBinding` resources **can be** used in conjunction with `gcp.gkehub.FeatureIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.gkehub.FeatureIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:gkehub:FeatureIamPolicy
///     properties:
///       project: ${feature.project}
///       location: ${feature.location}
///       name: ${feature.name}
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
/// ## gcp.gkehub.FeatureIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = feature_iam_binding::create(
///         "binding",
///         FeatureIamBindingArgs::builder()
///             .location("${feature.location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${feature.name}")
///             .project("${feature.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.gkehub.FeatureIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = feature_iam_member::create(
///         "member",
///         FeatureIamMemberArgs::builder()
///             .location("${feature.location}")
///             .member("user:jane@example.com")
///             .name("${feature.name}")
///             .project("${feature.project}")
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
/// # IAM policy for GKEHub Feature
/// Three different resources help you manage your IAM policy for GKEHub Feature. Each of these resources serves a different use case:
///
/// * `gcp.gkehub.FeatureIamPolicy`: Authoritative. Sets the IAM policy for the feature and replaces any existing policy already attached.
/// * `gcp.gkehub.FeatureIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the feature are preserved.
/// * `gcp.gkehub.FeatureIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the feature are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.gkehub.FeatureIamPolicy`: Retrieves the IAM policy for the feature
///
/// > **Note:** `gcp.gkehub.FeatureIamPolicy` **cannot** be used in conjunction with `gcp.gkehub.FeatureIamBinding` and `gcp.gkehub.FeatureIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.gkehub.FeatureIamBinding` resources **can be** used in conjunction with `gcp.gkehub.FeatureIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.gkehub.FeatureIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:gkehub:FeatureIamPolicy
///     properties:
///       project: ${feature.project}
///       location: ${feature.location}
///       name: ${feature.name}
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
/// ## gcp.gkehub.FeatureIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = feature_iam_binding::create(
///         "binding",
///         FeatureIamBindingArgs::builder()
///             .location("${feature.location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${feature.name}")
///             .project("${feature.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.gkehub.FeatureIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = feature_iam_member::create(
///         "member",
///         FeatureIamMemberArgs::builder()
///             .location("${feature.location}")
///             .member("user:jane@example.com")
///             .name("${feature.name}")
///             .project("${feature.project}")
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
/// * projects/{{project}}/locations/{{location}}/features/{{name}}
///
/// * {{project}}/{{location}}/{{name}}
///
/// * {{location}}/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// GKEHub feature IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:gkehub/featureIamMember:FeatureIamMember editor "projects/{{project}}/locations/{{location}}/features/{{feature}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:gkehub/featureIamMember:FeatureIamMember editor "projects/{{project}}/locations/{{location}}/features/{{feature}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:gkehub/featureIamMember:FeatureIamMember editor projects/{{project}}/locations/{{location}}/features/{{feature}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod feature_iam_member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FeatureIamMemberArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::gkehub::FeatureIamMemberCondition>,
        >,
        /// The location for the resource Used to find the parent resource to bind the IAM policy to. If not specified,
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
        pub member: pulumi_wasm_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.gkehub.FeatureIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FeatureIamMemberResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::gkehub::FeatureIamMemberCondition>,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The location for the resource Used to find the parent resource to bind the IAM policy to. If not specified,
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
        pub member: pulumi_wasm_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.gkehub.FeatureIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FeatureIamMemberArgs) -> FeatureIamMemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let location_binding = args.location.get_inner();
        let member_binding = args.member.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let role_binding = args.role.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gkehub/featureIamMember:FeatureIamMember".into(),
            name: name.to_string(),
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
                    name: "member".into(),
                    value: &member_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
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
                    name: "member".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
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
        FeatureIamMemberResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            member: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("member").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
        }
    }
}
