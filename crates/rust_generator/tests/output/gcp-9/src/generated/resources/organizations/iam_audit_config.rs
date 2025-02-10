/// Allows management of audit logging config for a given service for a Google Cloud Platform Organization.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let config = iam_audit_config::create(
///         "config",
///         IamAuditConfigArgs::builder()
///             .audit_log_configs(
///                 vec![
///                     IamAuditConfigAuditLogConfig::builder()
///                     .exemptedMembers(vec!["user:joebloggs@example.com",])
///                     .logType("DATA_READ").build_struct(),
///                 ],
///             )
///             .org_id("your-organization-id")
///             .service("allServices")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// IAM audit config imports use the identifier of the resource in question and the service, e.g.
///
/// ```sh
/// $ pulumi import gcp:organizations/iamAuditConfig:IamAuditConfig config "your-organization-id foo.googleapis.com"
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
            Vec<super::super::types::organizations::IamAuditConfigAuditLogConfig>,
        >,
        /// The numeric ID of the organization in which you want to manage the audit logging config.
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Service which will be enabled for audit logging.  The special value `allServices` covers all services.  Note that if there are google\_organization\_iam\_audit\_config resources covering both `allServices` and a specific service then the union of the two AuditConfigs is used for that service: the `log_types` specified in each `audit_log_config` are enabled, and the `exempted_members` in each `audit_log_config` are exempted.
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IamAuditConfigResult {
        /// The configuration for logging of each type of permission.  This can be specified multiple times.  Structure is documented below.
        pub audit_log_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::organizations::IamAuditConfigAuditLogConfig>,
        >,
        /// The etag of iam policy
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The numeric ID of the organization in which you want to manage the audit logging config.
        pub org_id: pulumi_gestalt_rust::Output<String>,
        /// Service which will be enabled for audit logging.  The special value `allServices` covers all services.  Note that if there are google\_organization\_iam\_audit\_config resources covering both `allServices` and a specific service then the union of the two AuditConfigs is used for that service: the `log_types` specified in each `audit_log_config` are enabled, and the `exempted_members` in each `audit_log_config` are exempted.
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
        let org_id_binding = args.org_id.get_output(context);
        let service_binding = args.service.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:organizations/iamAuditConfig:IamAuditConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "auditLogConfigs".into(),
                    value: audit_log_configs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgId".into(),
                    value: org_id_binding.get_id(),
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
            org_id: o.get_field("orgId"),
            service: o.get_field("service"),
        }
    }
}
