/// When managing IAM roles, you can treat a service account either as a resource or as an identity. This resource is to add iam policy bindings to a service account resource, such as allowing the members to run operations as or modify the service account. To configure permissions for a service account on other GCP resources, use the google_project_iam set of resources.
///
/// Three different resources help you manage your IAM policy for a service account. Each of these resources serves a different use case:
///
/// * `gcp.serviceaccount.IAMPolicy`: Authoritative. Sets the IAM policy for the service account and replaces any existing policy already attached.
/// * `gcp.serviceaccount.IAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the service account are preserved.
/// * `gcp.serviceaccount.IAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the service account are preserved.
///
/// > **Note:** `gcp.serviceaccount.IAMPolicy` **cannot** be used in conjunction with `gcp.serviceaccount.IAMBinding` and `gcp.serviceaccount.IAMMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.serviceaccount.IAMBinding` resources **can be** used in conjunction with `gcp.serviceaccount.IAMMember` resources **only if** they do not grant privilege to the same role.
///
/// ## Example Usage
///
/// ### Service Account IAM Policy
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that only Jane can interact with
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMPolicy
///     properties:
///       serviceAccountId: ${sa.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iam.serviceAccountUser
///             members:
///               - user:jane@example.com
/// ```
///
/// ### Service Account IAM Binding
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that only Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMBinding
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       members:
///         - user:jane@example.com
/// ```
///
/// ### Service Account IAM Binding With IAM Conditions:
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that only Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMBinding
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       members:
///         - user:jane@example.com
///       condition:
///         title: expires_after_2019_12_31
///         description: Expiring at midnight of 2019-12-31
///         expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
///
/// ### Service Account IAM Member
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMMember
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       member: user:jane@example.com
///   # Allow SA service account use the default GCE account
///   gce-default-account-iam:
///     type: gcp:serviceaccount:IAMMember
///     properties:
///       serviceAccountId: ${default.name}
///       role: roles/iam.serviceAccountUser
///       member: serviceAccount:${sa.email}
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:compute:getDefaultServiceAccount
///       arguments: {}
/// ```
///
/// ### Service Account IAM Member With IAM Conditions:
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMMember
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       member: user:jane@example.com
///       condition:
///         title: expires_after_2019_12_31
///         description: Expiring at midnight of 2019-12-31
///         expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
///
///
/// ### Additional Examples
///
/// ### Service Account IAM Policy
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that only Jane can interact with
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMPolicy
///     properties:
///       serviceAccountId: ${sa.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iam.serviceAccountUser
///             members:
///               - user:jane@example.com
/// ```
///
/// ### Service Account IAM Binding
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that only Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMBinding
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       members:
///         - user:jane@example.com
/// ```
///
/// ### Service Account IAM Binding With IAM Conditions:
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that only Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMBinding
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       members:
///         - user:jane@example.com
///       condition:
///         title: expires_after_2019_12_31
///         description: Expiring at midnight of 2019-12-31
///         expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
///
/// ### Service Account IAM Member
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMMember
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       member: user:jane@example.com
///   # Allow SA service account use the default GCE account
///   gce-default-account-iam:
///     type: gcp:serviceaccount:IAMMember
///     properties:
///       serviceAccountId: ${default.name}
///       role: roles/iam.serviceAccountUser
///       member: serviceAccount:${sa.email}
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:compute:getDefaultServiceAccount
///       arguments: {}
/// ```
///
/// ### Service Account IAM Member With IAM Conditions:
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMMember
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       member: user:jane@example.com
///       condition:
///         title: expires_after_2019_12_31
///         description: Expiring at midnight of 2019-12-31
///         expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
///
/// ## Import
///
/// ### Importing with conditions:
///
/// Here are examples of importing IAM memberships and bindings that include conditions:
///
/// ```sh
/// $ pulumi import gcp:serviceaccount/iAMMember:IAMMember admin-account-iam "projects/{your-project-id}/serviceAccounts/{your-service-account-email} roles/iam.serviceAccountUser expires_after_2019_12_31"
/// ```
///
/// ```sh
/// $ pulumi import gcp:serviceaccount/iAMMember:IAMMember admin-account-iam "projects/{your-project-id}/serviceAccounts/{your-service-account-email} roles/iam.serviceAccountUser user:foo@example.com expires_after_2019_12_31"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod iam_member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IAMMemberArgs {
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::serviceaccount::IamMemberCondition>,
        >,
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
        /// The role that should be applied. Only one
        /// `gcp.serviceaccount.IAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The fully-qualified name of the service account to apply policy to.
        #[builder(into)]
        pub service_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IAMMemberResult {
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::serviceaccount::IamMemberCondition>,
        >,
        /// (Computed) The etag of the service account IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        pub member: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.serviceaccount.IAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_gestalt_rust::Output<String>,
        /// The fully-qualified name of the service account to apply policy to.
        pub service_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: IAMMemberArgs,
    ) -> IAMMemberResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_output(context).get_inner();
        let member_binding = args.member.get_output(context).get_inner();
        let role_binding = args.role.get_output(context).get_inner();
        let service_account_id_binding = args
            .service_account_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:serviceaccount/iAMMember:IAMMember".into(),
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
                    name: "role".into(),
                    value: &role_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccountId".into(),
                    value: &service_account_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IAMMemberResult {
            condition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("condition"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            member: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("member"),
            ),
            role: pulumi_gestalt_rust::__private::into_domain(o.extract_field("role")),
            service_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceAccountId"),
            ),
        }
    }
}
