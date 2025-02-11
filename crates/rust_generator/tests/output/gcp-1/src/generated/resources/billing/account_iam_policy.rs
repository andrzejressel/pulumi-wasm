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
/// $ pulumi import gcp:billing/accountIamPolicy:AccountIamPolicy default {{billing_account_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountIamPolicyArgs {
        /// The billing account id.
        ///
        /// For `gcp.billing.AccountIamMember` or `gcp.billing.AccountIamBinding`:
        #[builder(into)]
        pub billing_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The policy data generated by a `gcp.organizations.getIAMPolicy` data source.
        ///
        /// - - -
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccountIamPolicyResult {
        /// The billing account id.
        ///
        /// For `gcp.billing.AccountIamMember` or `gcp.billing.AccountIamBinding`:
        pub billing_account_id: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the billing account's IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by a `gcp.organizations.getIAMPolicy` data source.
        ///
        /// - - -
        pub policy_data: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountIamPolicyArgs,
    ) -> AccountIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let billing_account_id_binding = args.billing_account_id.get_output(context);
        let policy_data_binding = args.policy_data.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:billing/accountIamPolicy:AccountIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "billingAccountId".into(),
                    value: &billing_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccountIamPolicyResult {
            billing_account_id: o.get_field("billingAccountId"),
            etag: o.get_field("etag"),
            policy_data: o.get_field("policyData"),
        }
    }
}
