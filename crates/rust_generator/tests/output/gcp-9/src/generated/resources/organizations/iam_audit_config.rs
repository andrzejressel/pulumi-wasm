/// Allows management of audit logging config for a given service for a Google Cloud Platform Organization.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod iam_audit_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IamAuditConfigArgs {
        /// The configuration for logging of each type of permission.  This can be specified multiple times.  Structure is documented below.
        #[builder(into)]
        pub audit_log_configs: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::organizations::IamAuditConfigAuditLogConfig>,
        >,
        /// The numeric ID of the organization in which you want to manage the audit logging config.
        #[builder(into)]
        pub org_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Service which will be enabled for audit logging.  The special value `allServices` covers all services.  Note that if there are google\_organization\_iam\_audit\_config resources covering both `allServices` and a specific service then the union of the two AuditConfigs is used for that service: the `log_types` specified in each `audit_log_config` are enabled, and the `exempted_members` in each `audit_log_config` are exempted.
        #[builder(into)]
        pub service: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IamAuditConfigResult {
        /// The configuration for logging of each type of permission.  This can be specified multiple times.  Structure is documented below.
        pub audit_log_configs: pulumi_wasm_rust::Output<
            Vec<super::super::types::organizations::IamAuditConfigAuditLogConfig>,
        >,
        /// The etag of iam policy
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The numeric ID of the organization in which you want to manage the audit logging config.
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// Service which will be enabled for audit logging.  The special value `allServices` covers all services.  Note that if there are google\_organization\_iam\_audit\_config resources covering both `allServices` and a specific service then the union of the two AuditConfigs is used for that service: the `log_types` specified in each `audit_log_config` are enabled, and the `exempted_members` in each `audit_log_config` are exempted.
        pub service: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IamAuditConfigArgs,
    ) -> IamAuditConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let audit_log_configs_binding = args
            .audit_log_configs
            .get_output(context)
            .get_inner();
        let org_id_binding = args.org_id.get_output(context).get_inner();
        let service_binding = args.service.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:organizations/iamAuditConfig:IamAuditConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "auditLogConfigs".into(),
                    value: &audit_log_configs_binding,
                },
                register_interface::ObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IamAuditConfigResult {
            audit_log_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("auditLogConfigs"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            org_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("orgId")),
            service: pulumi_wasm_rust::__private::into_domain(o.extract_field("service")),
        }
    }
}
