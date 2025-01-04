/// Three different resources help you manage your IAM policy for Cloud Functions (2nd gen) function. Each of these resources serves a different use case:
///
/// * `gcp.cloudfunctionsv2.FunctionIamPolicy`: Authoritative. Sets the IAM policy for the function and replaces any existing policy already attached.
/// * `gcp.cloudfunctionsv2.FunctionIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the function are preserved.
/// * `gcp.cloudfunctionsv2.FunctionIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the function are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.cloudfunctionsv2.FunctionIamPolicy`: Retrieves the IAM policy for the function
///
/// > **Note:** `gcp.cloudfunctionsv2.FunctionIamPolicy` **cannot** be used in conjunction with `gcp.cloudfunctionsv2.FunctionIamBinding` and `gcp.cloudfunctionsv2.FunctionIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.cloudfunctionsv2.FunctionIamBinding` resources **can be** used in conjunction with `gcp.cloudfunctionsv2.FunctionIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.cloudfunctionsv2.FunctionIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:cloudfunctionsv2:FunctionIamPolicy
///     properties:
///       project: ${function.project}
///       location: ${function.location}
///       cloudFunction: ${function.name}
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
/// ## gcp.cloudfunctionsv2.FunctionIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = function_iam_binding::create(
///         "binding",
///         FunctionIamBindingArgs::builder()
///             .cloud_function("${function.name}")
///             .location("${function.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${function.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.cloudfunctionsv2.FunctionIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = function_iam_member::create(
///         "member",
///         FunctionIamMemberArgs::builder()
///             .cloud_function("${function.name}")
///             .location("${function.location}")
///             .member("user:jane@example.com")
///             .project("${function.project}")
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
/// # IAM policy for Cloud Functions (2nd gen) function
/// Three different resources help you manage your IAM policy for Cloud Functions (2nd gen) function. Each of these resources serves a different use case:
///
/// * `gcp.cloudfunctionsv2.FunctionIamPolicy`: Authoritative. Sets the IAM policy for the function and replaces any existing policy already attached.
/// * `gcp.cloudfunctionsv2.FunctionIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the function are preserved.
/// * `gcp.cloudfunctionsv2.FunctionIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the function are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.cloudfunctionsv2.FunctionIamPolicy`: Retrieves the IAM policy for the function
///
/// > **Note:** `gcp.cloudfunctionsv2.FunctionIamPolicy` **cannot** be used in conjunction with `gcp.cloudfunctionsv2.FunctionIamBinding` and `gcp.cloudfunctionsv2.FunctionIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.cloudfunctionsv2.FunctionIamBinding` resources **can be** used in conjunction with `gcp.cloudfunctionsv2.FunctionIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.cloudfunctionsv2.FunctionIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:cloudfunctionsv2:FunctionIamPolicy
///     properties:
///       project: ${function.project}
///       location: ${function.location}
///       cloudFunction: ${function.name}
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
/// ## gcp.cloudfunctionsv2.FunctionIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = function_iam_binding::create(
///         "binding",
///         FunctionIamBindingArgs::builder()
///             .cloud_function("${function.name}")
///             .location("${function.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${function.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.cloudfunctionsv2.FunctionIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = function_iam_member::create(
///         "member",
///         FunctionIamMemberArgs::builder()
///             .cloud_function("${function.name}")
///             .location("${function.location}")
///             .member("user:jane@example.com")
///             .project("${function.project}")
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
/// * projects/{{project}}/locations/{{location}}/functions/{{cloud_function}}
///
/// * {{project}}/{{location}}/{{cloud_function}}
///
/// * {{location}}/{{cloud_function}}
///
/// * {{cloud_function}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Cloud Functions (2nd gen) function IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:cloudfunctionsv2/functionIamMember:FunctionIamMember editor "projects/{{project}}/locations/{{location}}/functions/{{cloud_function}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:cloudfunctionsv2/functionIamMember:FunctionIamMember editor "projects/{{project}}/locations/{{location}}/functions/{{cloud_function}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:cloudfunctionsv2/functionIamMember:FunctionIamMember editor projects/{{project}}/locations/{{location}}/functions/{{cloud_function}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod function_iam_member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionIamMemberArgs {
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub cloud_function: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudfunctionsv2::FunctionIamMemberCondition>,
        >,
        /// The location of this cloud function. Used to find the parent resource to bind the IAM policy to. If not specified,
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
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.cloudfunctionsv2.FunctionIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FunctionIamMemberResult {
        /// Used to find the parent resource to bind the IAM policy to
        pub cloud_function: pulumi_wasm_rust::Output<String>,
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudfunctionsv2::FunctionIamMemberCondition>,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The location of this cloud function. Used to find the parent resource to bind the IAM policy to. If not specified,
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
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.cloudfunctionsv2.FunctionIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FunctionIamMemberArgs) -> FunctionIamMemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloud_function_binding = args.cloud_function.get_inner();
        let condition_binding = args.condition.get_inner();
        let location_binding = args.location.get_inner();
        let member_binding = args.member.get_inner();
        let project_binding = args.project.get_inner();
        let role_binding = args.role.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:cloudfunctionsv2/functionIamMember:FunctionIamMember".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudFunction".into(),
                    value: &cloud_function_binding,
                },
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
                    name: "cloudFunction".into(),
                },
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
        FunctionIamMemberResult {
            cloud_function: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudFunction").unwrap(),
            ),
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
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
        }
    }
}
