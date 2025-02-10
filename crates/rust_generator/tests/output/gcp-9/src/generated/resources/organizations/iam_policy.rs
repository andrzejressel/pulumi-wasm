/// Four different resources help you manage your IAM policy for a organization. Each of these resources serves a different use case:
///
/// * `gcp.organizations.IAMPolicy`: Authoritative. Sets the IAM policy for the organization and replaces any existing policy already attached.
/// * `gcp.organizations.IAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the organization are preserved.
/// * `gcp.organizations.IAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the organization are preserved.
/// * `gcp.organizations.IamAuditConfig`: Authoritative for a given service. Updates the IAM policy to enable audit logging for the given service.
///
///
/// > **Note:** `gcp.organizations.IAMPolicy` **cannot** be used in conjunction with `gcp.organizations.IAMBinding`, `gcp.organizations.IAMMember`, or `gcp.organizations.IamAuditConfig` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.organizations.IAMBinding` resources **can be** used in conjunction with `gcp.organizations.IAMMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.organizations.IAMPolicy
///
/// !> **Warning:** New organizations have several default policies which will,
///    without extreme caution, be **overwritten** by use of this resource.
///    The safest alternative is to use multiple `gcp.organizations.IAMBinding`
///    resources. This resource makes it easy to remove your own access to
///    an organization, which will require a call to Google Support to have
///    fixed, and can take multiple days to resolve.
///
///
///    In general, this resource should only be used with organizations
///    fully managed by this provider.I f you do use this resource,
///    the best way to be sure that you are not making dangerous changes is to start
///    by **importing** your existing policy, and examining the diff very closely.
///
/// ```yaml
/// resources:
///   organization:
///     type: gcp:organizations:IAMPolicy
///     properties:
///       orgId: '1234567890'
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
///   organization:
///     type: gcp:organizations:IAMPolicy
///     properties:
///       orgId: '1234567890'
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
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
///
/// ## gcp.organizations.IAMBinding
///
/// > **Note:** If `role` is set to `roles/owner` and you don't specify a user or service account you have access to in `members`, you can lock yourself out of your organization.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let organization = iam_binding::create(
///         "organization",
///         IamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .org_id("1234567890")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let organization = iam_binding::create(
///         "organization",
///         IamBindingArgs::builder()
///             .condition(
///                 IamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .org_id("1234567890")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.organizations.IAMMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let organization = iam_member::create(
///         "organization",
///         IamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .org_id("1234567890")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let organization = iam_member::create(
///         "organization",
///         IamMemberArgs::builder()
///             .condition(
///                 IamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .org_id("1234567890")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.organizations.IamAuditConfig
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let organization = iam_audit_config::create(
///         "organization",
///         IamAuditConfigArgs::builder()
///             .audit_log_configs(
///                 vec![
///                     IamAuditConfigAuditLogConfig::builder().logType("ADMIN_READ")
///                     .build_struct(), IamAuditConfigAuditLogConfig::builder()
///                     .exemptedMembers(vec!["user:joebloggs@example.com",])
///                     .logType("DATA_READ").build_struct(),
///                 ],
///             )
///             .org_id("1234567890")
///             .service("allServices")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.organizations.IAMBinding
///
/// > **Note:** If `role` is set to `roles/owner` and you don't specify a user or service account you have access to in `members`, you can lock yourself out of your organization.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let organization = iam_binding::create(
///         "organization",
///         IamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .org_id("1234567890")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let organization = iam_binding::create(
///         "organization",
///         IamBindingArgs::builder()
///             .condition(
///                 IamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .org_id("1234567890")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.organizations.IAMMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let organization = iam_member::create(
///         "organization",
///         IamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .org_id("1234567890")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let organization = iam_member::create(
///         "organization",
///         IamMemberArgs::builder()
///             .condition(
///                 IamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .org_id("1234567890")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.organizations.IamAuditConfig
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let organization = iam_audit_config::create(
///         "organization",
///         IamAuditConfigArgs::builder()
///             .audit_log_configs(
///                 vec![
///                     IamAuditConfigAuditLogConfig::builder().logType("ADMIN_READ")
///                     .build_struct(), IamAuditConfigAuditLogConfig::builder()
///                     .exemptedMembers(vec!["user:joebloggs@example.com",])
///                     .logType("DATA_READ").build_struct(),
///                 ],
///             )
///             .org_id("1234567890")
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
/// An audit config can be imported into a `google_organization_iam_audit_config` resource using the resource's `org_id` and the `service`, e.g:
///
/// * `"{{org_id}} foo.googleapis.com"`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import audit configs:
///
/// tf
///
/// import {
///
///   id = "{{org_id}} foo.googleapis.com"
///
///   to = google_organization_iam_audit_config.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:organizations/iAMPolicy:IAMPolicy default "{{org_id}} foo.googleapis.com"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IAMPolicyArgs {
        /// The organization id of the target organization.
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The `gcp.organizations.getIAMPolicy` data source that represents
        /// the IAM policy that will be applied to the organization. The policy will be
        /// merged with any existing policy applied to the organization.
        ///
        /// Changing this updates the policy.
        ///
        /// Deleting this removes all policies from the organization, locking out users without
        /// organization-level access.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IAMPolicyResult {
        /// (Computed) The etag of the organization's IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The organization id of the target organization.
        pub org_id: pulumi_gestalt_rust::Output<String>,
        /// The `gcp.organizations.getIAMPolicy` data source that represents
        /// the IAM policy that will be applied to the organization. The policy will be
        /// merged with any existing policy applied to the organization.
        ///
        /// Changing this updates the policy.
        ///
        /// Deleting this removes all policies from the organization, locking out users without
        /// organization-level access.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IAMPolicyArgs,
    ) -> IAMPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let org_id_binding = args.org_id.get_output(context);
        let policy_data_binding = args.policy_data.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:organizations/iAMPolicy:IAMPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgId".into(),
                    value: org_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: policy_data_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IAMPolicyResult {
            etag: o.get_field("etag"),
            org_id: o.get_field("orgId"),
            policy_data: o.get_field("policyData"),
        }
    }
}
