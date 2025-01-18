/// Four different resources help you manage your IAM policy for a project. Each of these resources serves a different use case:
///
/// * `gcp.projects.IAMPolicy`: Authoritative. Sets the IAM policy for the project and replaces any existing policy already attached.
/// * `gcp.projects.IAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the project are preserved.
/// * `gcp.projects.IAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the project are preserved.
/// * `gcp.projects.IAMAuditConfig`: Authoritative for a given service. Updates the IAM policy to enable audit logging for the given service.
///
/// > **Note:** `gcp.projects.IAMPolicy` **cannot** be used in conjunction with `gcp.projects.IAMBinding`, `gcp.projects.IAMMember`, or `gcp.projects.IAMAuditConfig` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.projects.IAMBinding` resources **can be** used in conjunction with `gcp.projects.IAMMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:** The underlying API method `projects.setIamPolicy` has a lot of constraints which are documented [here](https://cloud.google.com/resource-manager/reference/rest/v1/projects/setIamPolicy). In addition to these constraints,
///    IAM Conditions cannot be used with Basic Roles such as Owner. Violating these constraints will result in the API returning 400 error code so please review these if you encounter errors with this resource.
///
/// ## gcp.projects.IAMPolicy
///
/// !> **Be careful!** You can accidentally lock yourself out of your project
///    using this resource. Deleting a `gcp.projects.IAMPolicy` removes access
///    from anyone without organization-level access to the project. Proceed with caution.
///    It's not recommended to use `gcp.projects.IAMPolicy` with your provider project
///    to avoid locking yourself out, and it should generally only be used with projects
///    fully managed by this provider. If you do use this resource, it is recommended to **import** the policy before
///    applying the change.
///
/// ```yaml
/// resources:
///   project:
///     type: gcp:projects:IAMPolicy
///     properties:
///       project: your-project-id
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/editor
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   project:
///     type: gcp:projects:IAMPolicy
///     properties:
///       project: your-project-id
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/compute.admin
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
///
/// ## gcp.projects.IAMBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let project = iam_binding::create(
///         "project",
///         IamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("your-project-id")
///             .role("roles/editor")
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
///     let project = iam_binding::create(
///         "project",
///         IamBindingArgs::builder()
///             .condition(
///                 IamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .project("your-project-id")
///             .role("roles/container.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.projects.IAMMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let project = iam_member::create(
///         "project",
///         IamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("your-project-id")
///             .role("roles/editor")
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
///     let project = iam_member::create(
///         "project",
///         IamMemberArgs::builder()
///             .condition(
///                 IamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .project("your-project-id")
///             .role("roles/firebase.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.projects.IAMAuditConfig
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let project = iam_audit_config::create(
///         "project",
///         IamAuditConfigArgs::builder()
///             .audit_log_configs(
///                 vec![
///                     IamAuditConfigAuditLogConfig::builder().logType("ADMIN_READ")
///                     .build_struct(), IamAuditConfigAuditLogConfig::builder()
///                     .exemptedMembers(vec!["user:joebloggs@example.com",])
///                     .logType("DATA_READ").build_struct(),
///                 ],
///             )
///             .project("your-project-id")
///             .service("allServices")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.projects.IAMPolicy
///
/// !> **Be careful!** You can accidentally lock yourself out of your project
///    using this resource. Deleting a `gcp.projects.IAMPolicy` removes access
///    from anyone without organization-level access to the project. Proceed with caution.
///    It's not recommended to use `gcp.projects.IAMPolicy` with your provider project
///    to avoid locking yourself out, and it should generally only be used with projects
///    fully managed by this provider. If you do use this resource, it is recommended to **import** the policy before
///    applying the change.
///
/// ```yaml
/// resources:
///   project:
///     type: gcp:projects:IAMPolicy
///     properties:
///       project: your-project-id
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/editor
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   project:
///     type: gcp:projects:IAMPolicy
///     properties:
///       project: your-project-id
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/compute.admin
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
///
/// ## gcp.projects.IAMBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let project = iam_binding::create(
///         "project",
///         IamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("your-project-id")
///             .role("roles/editor")
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
///     let project = iam_binding::create(
///         "project",
///         IamBindingArgs::builder()
///             .condition(
///                 IamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .project("your-project-id")
///             .role("roles/container.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.projects.IAMMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let project = iam_member::create(
///         "project",
///         IamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("your-project-id")
///             .role("roles/editor")
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
///     let project = iam_member::create(
///         "project",
///         IamMemberArgs::builder()
///             .condition(
///                 IamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .project("your-project-id")
///             .role("roles/firebase.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.projects.IAMAuditConfig
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let project = iam_audit_config::create(
///         "project",
///         IamAuditConfigArgs::builder()
///             .audit_log_configs(
///                 vec![
///                     IamAuditConfigAuditLogConfig::builder().logType("ADMIN_READ")
///                     .build_struct(), IamAuditConfigAuditLogConfig::builder()
///                     .exemptedMembers(vec!["user:joebloggs@example.com",])
///                     .logType("DATA_READ").build_struct(),
///                 ],
///             )
///             .project("your-project-id")
///             .service("allServices")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Importing Audit Configs
///
/// An audit config can be imported into a `google_project_iam_audit_config` resource using the resource's `project_id` and the `service`, e.g:
///
/// * `"{{project_id}} foo.googleapis.com"`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import audit configs:
///
/// tf
///
/// import {
///
///   id = "{{project_id}} foo.googleapis.com"
///
///   to = google_project_iam_audit_config.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:projects/iAMMember:IAMMember default "{{project_id}} foo.googleapis.com"
/// ```
///
pub mod iam_member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IAMMemberArgs {
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::projects::IamMemberCondition>,
        >,
        /// Identities that will be granted the privilege in `role`. gcp.projects.IAMBinding expects `members` field while gcp.projects.IAMMember expects `member` field.
        /// Each entry can have one of the following values:
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        #[builder(into)]
        pub member: pulumi_wasm_rust::Output<String>,
        /// The project id of the target project. This is not
        /// inferred from the provider.
        #[builder(into)]
        pub project: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.projects.IAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct IAMMemberResult {
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::projects::IamMemberCondition>,
        >,
        /// (Computed) The etag of the project's IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`. gcp.projects.IAMBinding expects `members` field while gcp.projects.IAMMember expects `member` field.
        /// Each entry can have one of the following values:
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        pub member: pulumi_wasm_rust::Output<String>,
        /// The project id of the target project. This is not
        /// inferred from the provider.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.projects.IAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: IAMMemberArgs) -> IAMMemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let member_binding = args.member.get_inner();
        let project_binding = args.project.get_inner();
        let role_binding = args.role.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:projects/iAMMember:IAMMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        IAMMemberResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
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
