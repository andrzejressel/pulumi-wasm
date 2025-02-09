/// Three different resources help you manage your IAM policy for a Spanner instance. Each of these resources serves a different use case:
///
/// * `gcp.spanner.InstanceIAMPolicy`: Authoritative. Sets the IAM policy for the instance and replaces any existing policy already attached.
///
/// > **Warning:** It's entirely possibly to lock yourself out of your instance using `gcp.spanner.InstanceIAMPolicy`. Any permissions granted by default will be removed unless you include them in your config.
///
/// * `gcp.spanner.InstanceIAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the instance are preserved.
/// * `gcp.spanner.InstanceIAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the instance are preserved.
///
/// > **Note:** `gcp.spanner.InstanceIAMPolicy` **cannot** be used in conjunction with `gcp.spanner.InstanceIAMBinding` and `gcp.spanner.InstanceIAMMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.spanner.InstanceIAMBinding` resources **can be** used in conjunction with `gcp.spanner.InstanceIAMMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.spanner.InstanceIAMPolicy
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:spanner:InstanceIAMPolicy
///     properties:
///       instance: your-instance-name
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
/// ## gcp.spanner.InstanceIAMBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = instance_iam_binding::create(
///         "instance",
///         InstanceIamBindingArgs::builder()
///             .instance("your-instance-name")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/spanner.databaseAdmin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.spanner.InstanceIAMMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = instance_iam_member::create(
///         "instance",
///         InstanceIamMemberArgs::builder()
///             .instance("your-instance-name")
///             .member("user:jane@example.com")
///             .role("roles/spanner.databaseAdmin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.spanner.InstanceIAMBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = instance_iam_binding::create(
///         "instance",
///         InstanceIamBindingArgs::builder()
///             .instance("your-instance-name")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/spanner.databaseAdmin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.spanner.InstanceIAMMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = instance_iam_member::create(
///         "instance",
///         InstanceIamMemberArgs::builder()
///             .instance("your-instance-name")
///             .member("user:jane@example.com")
///             .role("roles/spanner.databaseAdmin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Importing IAM policies
///
/// IAM policy imports use the identifier of the Spanner Instances resource . For example:
///
/// * `{{project}}/{{instance}}`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = {{project}}/{{instance}}
///
///   to = google_spanner_instance_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:spanner/instanceIAMMember:InstanceIAMMember default {{project}}/{{instance}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_iam_member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceIAMMemberArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::spanner::InstanceIamMemberCondition>,
        >,
        /// The name of the instance.
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        #[builder(into)]
        pub member: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.spanner.InstanceIAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceIAMMemberResult {
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::spanner::InstanceIamMemberCondition>,
        >,
        /// (Computed) The etag of the instance's IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The name of the instance.
        pub instance: pulumi_gestalt_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        pub member: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.spanner.InstanceIAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InstanceIAMMemberArgs,
    ) -> InstanceIAMMemberResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let condition_binding_1 = args.condition.get_output(context);
        let condition_binding = condition_binding_1.get_inner();
        let instance_binding_1 = args.instance.get_output(context);
        let instance_binding = instance_binding_1.get_inner();
        let member_binding_1 = args.member.get_output(context);
        let member_binding = member_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let role_binding_1 = args.role.get_output(context);
        let role_binding = role_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:spanner/instanceIAMMember:InstanceIAMMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceIAMMemberResult {
            condition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("condition"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            instance: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instance"),
            ),
            member: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("member"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            role: pulumi_gestalt_rust::__private::into_domain(o.extract_field("role")),
        }
    }
}
