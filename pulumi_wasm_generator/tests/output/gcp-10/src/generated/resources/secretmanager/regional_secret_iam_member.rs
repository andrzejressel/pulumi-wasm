/// Three different resources help you manage your IAM policy for Secret Manager RegionalSecret. Each of these resources serves a different use case:
///
/// * `gcp.secretmanager.RegionalSecretIamPolicy`: Authoritative. Sets the IAM policy for the regionalsecret and replaces any existing policy already attached.
/// * `gcp.secretmanager.RegionalSecretIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the regionalsecret are preserved.
/// * `gcp.secretmanager.RegionalSecretIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the regionalsecret are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.secretmanager.RegionalSecretIamPolicy`: Retrieves the IAM policy for the regionalsecret
///
/// > **Note:** `gcp.secretmanager.RegionalSecretIamPolicy` **cannot** be used in conjunction with `gcp.secretmanager.RegionalSecretIamBinding` and `gcp.secretmanager.RegionalSecretIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.secretmanager.RegionalSecretIamBinding` resources **can be** used in conjunction with `gcp.secretmanager.RegionalSecretIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.secretmanager.RegionalSecretIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:secretmanager:RegionalSecretIamPolicy
///     properties:
///       project: ${["regional-secret-basic"].project}
///       location: ${["regional-secret-basic"].location}
///       secretId: ${["regional-secret-basic"].secretId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/secretmanager.secretAccessor
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:secretmanager:RegionalSecretIamPolicy
///     properties:
///       project: ${["regional-secret-basic"].project}
///       location: ${["regional-secret-basic"].location}
///       secretId: ${["regional-secret-basic"].secretId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/secretmanager.secretAccessor
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.secretmanager.RegionalSecretIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = regional_secret_iam_binding::create(
///         "binding",
///         RegionalSecretIamBindingArgs::builder()
///             .location("${[\"regional-secret-basic\"].location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"regional-secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"regional-secret-basic\"].secretId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = regional_secret_iam_binding::create(
///         "binding",
///         RegionalSecretIamBindingArgs::builder()
///             .condition(
///                 RegionalSecretIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .location("${[\"regional-secret-basic\"].location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"regional-secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"regional-secret-basic\"].secretId}")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.secretmanager.RegionalSecretIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = regional_secret_iam_member::create(
///         "member",
///         RegionalSecretIamMemberArgs::builder()
///             .location("${[\"regional-secret-basic\"].location}")
///             .member("user:jane@example.com")
///             .project("${[\"regional-secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"regional-secret-basic\"].secretId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = regional_secret_iam_member::create(
///         "member",
///         RegionalSecretIamMemberArgs::builder()
///             .condition(
///                 RegionalSecretIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .location("${[\"regional-secret-basic\"].location}")
///             .member("user:jane@example.com")
///             .project("${[\"regional-secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"regional-secret-basic\"].secretId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for Secret Manager RegionalSecret
/// Three different resources help you manage your IAM policy for Secret Manager RegionalSecret. Each of these resources serves a different use case:
///
/// * `gcp.secretmanager.RegionalSecretIamPolicy`: Authoritative. Sets the IAM policy for the regionalsecret and replaces any existing policy already attached.
/// * `gcp.secretmanager.RegionalSecretIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the regionalsecret are preserved.
/// * `gcp.secretmanager.RegionalSecretIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the regionalsecret are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.secretmanager.RegionalSecretIamPolicy`: Retrieves the IAM policy for the regionalsecret
///
/// > **Note:** `gcp.secretmanager.RegionalSecretIamPolicy` **cannot** be used in conjunction with `gcp.secretmanager.RegionalSecretIamBinding` and `gcp.secretmanager.RegionalSecretIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.secretmanager.RegionalSecretIamBinding` resources **can be** used in conjunction with `gcp.secretmanager.RegionalSecretIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.secretmanager.RegionalSecretIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:secretmanager:RegionalSecretIamPolicy
///     properties:
///       project: ${["regional-secret-basic"].project}
///       location: ${["regional-secret-basic"].location}
///       secretId: ${["regional-secret-basic"].secretId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/secretmanager.secretAccessor
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:secretmanager:RegionalSecretIamPolicy
///     properties:
///       project: ${["regional-secret-basic"].project}
///       location: ${["regional-secret-basic"].location}
///       secretId: ${["regional-secret-basic"].secretId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/secretmanager.secretAccessor
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.secretmanager.RegionalSecretIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = regional_secret_iam_binding::create(
///         "binding",
///         RegionalSecretIamBindingArgs::builder()
///             .location("${[\"regional-secret-basic\"].location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"regional-secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"regional-secret-basic\"].secretId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = regional_secret_iam_binding::create(
///         "binding",
///         RegionalSecretIamBindingArgs::builder()
///             .condition(
///                 RegionalSecretIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .location("${[\"regional-secret-basic\"].location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"regional-secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"regional-secret-basic\"].secretId}")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.secretmanager.RegionalSecretIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = regional_secret_iam_member::create(
///         "member",
///         RegionalSecretIamMemberArgs::builder()
///             .location("${[\"regional-secret-basic\"].location}")
///             .member("user:jane@example.com")
///             .project("${[\"regional-secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"regional-secret-basic\"].secretId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = regional_secret_iam_member::create(
///         "member",
///         RegionalSecretIamMemberArgs::builder()
///             .condition(
///                 RegionalSecretIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .location("${[\"regional-secret-basic\"].location}")
///             .member("user:jane@example.com")
///             .project("${[\"regional-secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"regional-secret-basic\"].secretId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}
///
/// * {{project}}/{{location}}/{{secret_id}}
///
/// * {{location}}/{{secret_id}}
///
/// * {{secret_id}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Secret Manager regionalsecret IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:secretmanager/regionalSecretIamMember:RegionalSecretIamMember editor "projects/{{project}}/locations/{{location}}/secrets/{{secret_id}} roles/secretmanager.secretAccessor user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:secretmanager/regionalSecretIamMember:RegionalSecretIamMember editor "projects/{{project}}/locations/{{location}}/secrets/{{secret_id}} roles/secretmanager.secretAccessor"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:secretmanager/regionalSecretIamMember:RegionalSecretIamMember editor projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod regional_secret_iam_member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionalSecretIamMemberArgs {
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::secretmanager::RegionalSecretIamMemberCondition>,
        >,
        /// The location of the regional secret. eg us-central1
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
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
        /// `gcp.secretmanager.RegionalSecretIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub secret_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RegionalSecretIamMemberResult {
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::secretmanager::RegionalSecretIamMemberCondition>,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The location of the regional secret. eg us-central1
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
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
        /// `gcp.secretmanager.RegionalSecretIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
        pub secret_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RegionalSecretIamMemberArgs,
    ) -> RegionalSecretIamMemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let location_binding = args.location.get_inner();
        let member_binding = args.member.get_inner();
        let project_binding = args.project.get_inner();
        let role_binding = args.role.get_inner();
        let secret_id_binding = args.secret_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:secretmanager/regionalSecretIamMember:RegionalSecretIamMember"
                .into(),
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
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
                register_interface::ObjectField {
                    name: "secretId".into(),
                    value: &secret_id_binding,
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
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
                },
                register_interface::ResultField {
                    name: "secretId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegionalSecretIamMemberResult {
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
            secret_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretId").unwrap(),
            ),
        }
    }
}
