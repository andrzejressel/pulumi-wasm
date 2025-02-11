/// Manages an Azure Active Directory Diagnostic Setting for Azure Monitor.
///
/// !> **Authentication** The API for this resource does not support service principal authentication. This resource can only be used with Azure CLI authentication.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("west europe")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAadDiagnosticSetting = aad_diagnostic_setting::create(
///         "exampleAadDiagnosticSetting",
///         AadDiagnosticSettingArgs::builder()
///             .enabled_logs(
///                 vec![
///                     AadDiagnosticSettingEnabledLog::builder().category("SignInLogs")
///                     .build_struct(), AadDiagnosticSettingEnabledLog::builder()
///                     .category("AuditLogs").build_struct(),
///                     AadDiagnosticSettingEnabledLog::builder()
///                     .category("NonInteractiveUserSignInLogs").build_struct(),
///                     AadDiagnosticSettingEnabledLog::builder()
///                     .category("ServicePrincipalSignInLogs").build_struct(),
///                 ],
///             )
///             .name("setting1")
///             .storage_account_id("${exampleAccount.id}")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_kind("StorageV2")
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("examplestorageaccount")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Monitor Azure Active Directory Diagnostic Settings can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/aadDiagnosticSetting:AadDiagnosticSetting example /providers/Microsoft.AADIAM/diagnosticSettings/setting1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod aad_diagnostic_setting {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AadDiagnosticSettingArgs {
        /// One or more `enabled_log` blocks as defined below.
        #[builder(into, default)]
        pub enabled_logs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::AadDiagnosticSettingEnabledLog>>,
        >,
        /// Specifies the ID of an Event Hub Namespace Authorization Rule used to send Diagnostics Data. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This can be sourced from the `azure.eventhub.EventHubNamespaceAuthorizationRule` resource and is different from a `azure.eventhub.AuthorizationRule` resource.
        #[builder(into, default)]
        pub eventhub_authorization_rule_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the Event Hub where Diagnostics Data should be sent. If not specified, the default Event Hub will be used. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub eventhub_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of a Log Analytics Workspace where Diagnostics Data should be sent.
        #[builder(into, default)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name which should be used for this Monitor Azure Active Directory Diagnostic Setting. Changing this forces a new Monitor Azure Active Directory Diagnostic Setting to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Storage Account where logs should be sent. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One of `eventhub_authorization_rule_id`, `log_analytics_workspace_id` and `storage_account_id` must be specified.
        #[builder(into, default)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AadDiagnosticSettingResult {
        /// One or more `enabled_log` blocks as defined below.
        pub enabled_logs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::AadDiagnosticSettingEnabledLog>>,
        >,
        /// Specifies the ID of an Event Hub Namespace Authorization Rule used to send Diagnostics Data. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This can be sourced from the `azure.eventhub.EventHubNamespaceAuthorizationRule` resource and is different from a `azure.eventhub.AuthorizationRule` resource.
        pub eventhub_authorization_rule_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Event Hub where Diagnostics Data should be sent. If not specified, the default Event Hub will be used. Changing this forces a new resource to be created.
        pub eventhub_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the ID of a Log Analytics Workspace where Diagnostics Data should be sent.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Monitor Azure Active Directory Diagnostic Setting. Changing this forces a new Monitor Azure Active Directory Diagnostic Setting to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Storage Account where logs should be sent. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One of `eventhub_authorization_rule_id`, `log_analytics_workspace_id` and `storage_account_id` must be specified.
        pub storage_account_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AadDiagnosticSettingArgs,
    ) -> AadDiagnosticSettingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enabled_logs_binding = args.enabled_logs.get_output(context);
        let eventhub_authorization_rule_id_binding = args
            .eventhub_authorization_rule_id
            .get_output(context);
        let eventhub_name_binding = args.eventhub_name.get_output(context);
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:monitoring/aadDiagnosticSetting:AadDiagnosticSetting".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabledLogs".into(),
                    value: &enabled_logs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubAuthorizationRuleId".into(),
                    value: &eventhub_authorization_rule_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubName".into(),
                    value: &eventhub_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AadDiagnosticSettingResult {
            enabled_logs: o.get_field("enabledLogs"),
            eventhub_authorization_rule_id: o.get_field("eventhubAuthorizationRuleId"),
            eventhub_name: o.get_field("eventhubName"),
            log_analytics_workspace_id: o.get_field("logAnalyticsWorkspaceId"),
            name: o.get_field("name"),
            storage_account_id: o.get_field("storageAccountId"),
        }
    }
}
