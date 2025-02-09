/// Three different resources help you manage IAM policies on billing accounts. Each of these resources serves a different use case:
///
/// * `gcp.billing.AccountIamPolicy`: Authoritative. Sets the IAM policy for the billing accounts and replaces any existing policy already attached.
/// * `gcp.billing.AccountIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the table are preserved.
/// * `gcp.billing.AccountIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role of the billing accounts are preserved.
///
/// > **Note:** `gcp.billing.AccountIamPolicy` **cannot** be used in conjunction with `gcp.billing.AccountIamBinding` and `gcp.billing.AccountIamMember` or they will fight over what your policy should be. In addition, be careful not to accidentally unset ownership of the billing account as `gcp.billing.AccountIamPolicy` replaces the entire policy.
///
/// > **Note:** `gcp.billing.AccountIamBinding` resources **can be** used in conjunction with `gcp.billing.AccountIamMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.billing.AccountIamPolicy
///
/// ```yaml
/// resources:
///   editor:
///     type: gcp:billing:AccountIamPolicy
///     properties:
///       billingAccountId: 00AA00-000AAA-00AA0A
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/billing.viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.billing.AccountIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = account_iam_binding::create(
///         "editor",
///         AccountIamBindingArgs::builder()
///             .billing_account_id("00AA00-000AAA-00AA0A")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/billing.viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.billing.AccountIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = account_iam_member::create(
///         "editor",
///         AccountIamMemberArgs::builder()
///             .billing_account_id("00AA00-000AAA-00AA0A")
///             .member("user:jane@example.com")
///             .role("roles/billing.viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.billing.AccountIamPolicy
///
/// ```yaml
/// resources:
///   editor:
///     type: gcp:billing:AccountIamPolicy
///     properties:
///       billingAccountId: 00AA00-000AAA-00AA0A
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/billing.viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.billing.AccountIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = account_iam_binding::create(
///         "editor",
///         AccountIamBindingArgs::builder()
///             .billing_account_id("00AA00-000AAA-00AA0A")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/billing.viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.billing.AccountIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = account_iam_member::create(
///         "editor",
///         AccountIamMemberArgs::builder()
///             .billing_account_id("00AA00-000AAA-00AA0A")
///             .member("user:jane@example.com")
///             .role("roles/billing.viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Importing IAM policies
///
/// IAM policy imports use the `billing_account_id` identifier of the Billing Account resource only. For example:
///
/// * `{{billing_account_id}}`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = {{billing_account_id}}
///
///   to = google_billing_account_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:billing/accountIamMember:AccountIamMember default {{billing_account_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account_iam_member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountIamMemberArgs {
        /// The billing account id.
        ///
        /// For `gcp.billing.AccountIamMember` or `gcp.billing.AccountIamBinding`:
        #[builder(into)]
        pub billing_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::billing::AccountIamMemberCondition>,
        >,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        #[builder(into)]
        pub member: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The role that should be applied. Only one
        /// `gcp.billing.AccountIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`. Read more about roles [here](https://cloud.google.com/bigtable/docs/access-control#roles).
        ///
        /// `gcp.billing.AccountIamPolicy` only:
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccountIamMemberResult {
        /// The billing account id.
        ///
        /// For `gcp.billing.AccountIamMember` or `gcp.billing.AccountIamBinding`:
        pub billing_account_id: pulumi_gestalt_rust::Output<String>,
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::billing::AccountIamMemberCondition>,
        >,
        /// (Computed) The etag of the billing account's IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        pub member: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.billing.AccountIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`. Read more about roles [here](https://cloud.google.com/bigtable/docs/access-control#roles).
        ///
        /// `gcp.billing.AccountIamPolicy` only:
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AccountIamMemberArgs,
    ) -> AccountIamMemberResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let billing_account_id_binding_1 = args.billing_account_id.get_output(context);
        let billing_account_id_binding = billing_account_id_binding_1.get_inner();
        let condition_binding_1 = args.condition.get_output(context);
        let condition_binding = condition_binding_1.get_inner();
        let member_binding_1 = args.member.get_output(context);
        let member_binding = member_binding_1.get_inner();
        let role_binding_1 = args.role.get_output(context);
        let role_binding = role_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:billing/accountIamMember:AccountIamMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "billingAccountId".into(),
                    value: &billing_account_id_binding,
                },
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
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccountIamMemberResult {
            billing_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("billingAccountId"),
            ),
            condition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("condition"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            member: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("member"),
            ),
            role: pulumi_gestalt_rust::__private::into_domain(o.extract_field("role")),
        }
    }
}
