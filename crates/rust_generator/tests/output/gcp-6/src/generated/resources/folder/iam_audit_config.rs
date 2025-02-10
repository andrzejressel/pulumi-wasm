/// Four different resources help you manage your IAM policy for a folder. Each of these resources serves a different use case:
///
/// * `gcp.folder.IAMPolicy`: Authoritative. Sets the IAM policy for the folder and replaces any existing policy already attached.
/// * `gcp.folder.IAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the folder are preserved.
/// * `gcp.folder.IAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the folder are preserved.
/// * `gcp.folder.IamAuditConfig`: Authoritative for a given service. Updates the IAM policy to enable audit logging for the given service.
///
///
/// > **Note:** `gcp.folder.IAMPolicy` **cannot** be used in conjunction with `gcp.folder.IAMBinding`, `gcp.folder.IAMMember`, or `gcp.folder.IamAuditConfig` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.folder.IAMBinding` resources **can be** used in conjunction with `gcp.folder.IAMMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:** The underlying API method `projects.setIamPolicy` has constraints which are documented [here](https://cloud.google.com/resource-manager/reference/rest/v1/projects/setIamPolicy). In addition to these constraints,
///    IAM Conditions cannot be used with Basic Roles such as Owner. Violating these constraints will result in the API returning a 400 error code so please review these if you encounter errors with this resource.
///
/// ## gcp.folder.IAMPolicy
///
/// !> **Be careful!** You can accidentally lock yourself out of your folder
///    using this resource. Deleting a `gcp.folder.IAMPolicy` removes access
///    from anyone without permissions on its parent folder/organization. Proceed with caution.
///    It's not recommended to use `gcp.folder.IAMPolicy` with your provider folder
///    to avoid locking yourself out, and it should generally only be used with folders
///    fully managed by this provider. If you do use this resource, it is recommended to **import** the policy before
///    applying the change.
///
/// ```yaml
/// resources:
///   folder:
///     type: gcp:folder:IAMPolicy
///     properties:
///       folder: folders/1234567
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
///   folder:
///     type: gcp:folder:IAMPolicy
///     properties:
///       folder: folders/1234567
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
/// ## gcp.folder.IAMBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let folder = iam_binding::create(
///         "folder",
///         IamBindingArgs::builder()
///             .folder("folders/1234567")
///             .members(vec!["user:jane@example.com",])
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
///     let folder = iam_binding::create(
///         "folder",
///         IamBindingArgs::builder()
///             .condition(
///                 IamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .folder("folders/1234567")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/container.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.folder.IAMMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let folder = iam_member::create(
///         "folder",
///         IamMemberArgs::builder()
///             .folder("folders/1234567")
///             .member("user:jane@example.com")
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
///     let folder = iam_member::create(
///         "folder",
///         IamMemberArgs::builder()
///             .condition(
///                 IamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .folder("folders/1234567")
///             .member("user:jane@example.com")
///             .role("roles/firebase.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.folder.IamAuditConfig
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let folder = iam_audit_config::create(
///         "folder",
///         IamAuditConfigArgs::builder()
///             .audit_log_configs(
///                 vec![
///                     IamAuditConfigAuditLogConfig::builder().logType("ADMIN_READ")
///                     .build_struct(), IamAuditConfigAuditLogConfig::builder()
///                     .exemptedMembers(vec!["user:joebloggs@example.com",])
///                     .logType("DATA_READ").build_struct(),
///                 ],
///             )
///             .folder("folders/1234567")
///             .service("allServices")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.folder.IAMBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let folder = iam_binding::create(
///         "folder",
///         IamBindingArgs::builder()
///             .folder("folders/1234567")
///             .members(vec!["user:jane@example.com",])
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
///     let folder = iam_binding::create(
///         "folder",
///         IamBindingArgs::builder()
///             .condition(
///                 IamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .folder("folders/1234567")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/container.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.folder.IAMMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let folder = iam_member::create(
///         "folder",
///         IamMemberArgs::builder()
///             .folder("folders/1234567")
///             .member("user:jane@example.com")
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
///     let folder = iam_member::create(
///         "folder",
///         IamMemberArgs::builder()
///             .condition(
///                 IamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .folder("folders/1234567")
///             .member("user:jane@example.com")
///             .role("roles/firebase.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.folder.IamAuditConfig
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let folder = iam_audit_config::create(
///         "folder",
///         IamAuditConfigArgs::builder()
///             .audit_log_configs(
///                 vec![
///                     IamAuditConfigAuditLogConfig::builder().logType("ADMIN_READ")
///                     .build_struct(), IamAuditConfigAuditLogConfig::builder()
///                     .exemptedMembers(vec!["user:joebloggs@example.com",])
///                     .logType("DATA_READ").build_struct(),
///                 ],
///             )
///             .folder("folders/1234567")
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
/// An audit config can be imported into a `google_folder_iam_audit_config` resource using the resource's `folder_id` and the `service`, e.g:
///
/// * `"folder/{{folder_id}} foo.googleapis.com"`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import audit configs:
///
/// tf
///
/// import {
///
///   id = "folder/{{folder_id}} foo.googleapis.com"
///
///   to = google_folder_iam_audit_config.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:folder/iamAuditConfig:IamAuditConfig default "folder/{{folder_id}} foo.googleapis.com"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod iam_audit_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IamAuditConfigArgs {
        /// The configuration for logging of each type of permission.  This can be specified multiple times.  Structure is documented below.
        #[builder(into)]
        pub audit_log_configs: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::folder::IamAuditConfigAuditLogConfig>,
        >,
        /// The resource name of the folder the policy is attached to. Its format is folders/{folder_id}.
        #[builder(into)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Service which will be enabled for audit logging.  The special value `allServices` covers all services.  Note that if there are gcp.folder.IamAuditConfig resources covering both `allServices` and a specific service then the union of the two AuditConfigs is used for that service: the `log_types` specified in each `audit_log_config` are enabled, and the `exempted_members` in each `audit_log_config` are exempted.
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IamAuditConfigResult {
        /// The configuration for logging of each type of permission.  This can be specified multiple times.  Structure is documented below.
        pub audit_log_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::folder::IamAuditConfigAuditLogConfig>,
        >,
        /// (Computed) The etag of the folder's IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the folder the policy is attached to. Its format is folders/{folder_id}.
        pub folder: pulumi_gestalt_rust::Output<String>,
        /// Service which will be enabled for audit logging.  The special value `allServices` covers all services.  Note that if there are gcp.folder.IamAuditConfig resources covering both `allServices` and a specific service then the union of the two AuditConfigs is used for that service: the `log_types` specified in each `audit_log_config` are enabled, and the `exempted_members` in each `audit_log_config` are exempted.
        pub service: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IamAuditConfigArgs,
    ) -> IamAuditConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let audit_log_configs_binding = args.audit_log_configs.get_output(context);
        let folder_binding = args.folder.get_output(context);
        let service_binding = args.service.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:folder/iamAuditConfig:IamAuditConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "auditLogConfigs".into(),
                    value: audit_log_configs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folder".into(),
                    value: folder_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "service".into(),
                    value: service_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IamAuditConfigResult {
            audit_log_configs: o.get_field("auditLogConfigs"),
            etag: o.get_field("etag"),
            folder: o.get_field("folder"),
            service: o.get_field("service"),
        }
    }
}
