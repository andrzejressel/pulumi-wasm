/// Manages an Azure Active Directory Diagnostic Setting for Azure Monitor.
///
/// !> **Authentication** The API for this resource does not support service principal authentication. This resource can only be used with Azure CLI authentication.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod aad_diagnostic_setting {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AadDiagnosticSettingArgs {
        /// One or more `enabled_log` blocks as defined below.
        #[builder(into, default)]
        pub enabled_logs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::AadDiagnosticSettingEnabledLog>>,
        >,
        /// Specifies the ID of an Event Hub Namespace Authorization Rule used to send Diagnostics Data. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This can be sourced from the `azure.eventhub.EventHubNamespaceAuthorizationRule` resource and is different from a `azure.eventhub.AuthorizationRule` resource.
        #[builder(into, default)]
        pub eventhub_authorization_rule_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Event Hub where Diagnostics Data should be sent. If not specified, the default Event Hub will be used. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub eventhub_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of a Log Analytics Workspace where Diagnostics Data should be sent.
        #[builder(into, default)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Monitor Azure Active Directory Diagnostic Setting. Changing this forces a new Monitor Azure Active Directory Diagnostic Setting to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Storage Account where logs should be sent. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One of `eventhub_authorization_rule_id`, `log_analytics_workspace_id` and `storage_account_id` must be specified.
        #[builder(into, default)]
        pub storage_account_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AadDiagnosticSettingResult {
        /// One or more `enabled_log` blocks as defined below.
        pub enabled_logs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::AadDiagnosticSettingEnabledLog>>,
        >,
        /// Specifies the ID of an Event Hub Namespace Authorization Rule used to send Diagnostics Data. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This can be sourced from the `azure.eventhub.EventHubNamespaceAuthorizationRule` resource and is different from a `azure.eventhub.AuthorizationRule` resource.
        pub eventhub_authorization_rule_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Event Hub where Diagnostics Data should be sent. If not specified, the default Event Hub will be used. Changing this forces a new resource to be created.
        pub eventhub_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of a Log Analytics Workspace where Diagnostics Data should be sent.
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Monitor Azure Active Directory Diagnostic Setting. Changing this forces a new Monitor Azure Active Directory Diagnostic Setting to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Storage Account where logs should be sent. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One of `eventhub_authorization_rule_id`, `log_analytics_workspace_id` and `storage_account_id` must be specified.
        pub storage_account_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AadDiagnosticSettingArgs,
    ) -> AadDiagnosticSettingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enabled_logs_binding = args.enabled_logs.get_inner();
        let eventhub_authorization_rule_id_binding = args
            .eventhub_authorization_rule_id
            .get_inner();
        let eventhub_name_binding = args.eventhub_name.get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/aadDiagnosticSetting:AadDiagnosticSetting".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabledLogs".into(),
                    value: &enabled_logs_binding,
                },
                register_interface::ObjectField {
                    name: "eventhubAuthorizationRuleId".into(),
                    value: &eventhub_authorization_rule_id_binding,
                },
                register_interface::ObjectField {
                    name: "eventhubName".into(),
                    value: &eventhub_name_binding,
                },
                register_interface::ObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enabledLogs".into(),
                },
                register_interface::ResultField {
                    name: "eventhubAuthorizationRuleId".into(),
                },
                register_interface::ResultField {
                    name: "eventhubName".into(),
                },
                register_interface::ResultField {
                    name: "logAnalyticsWorkspaceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AadDiagnosticSettingResult {
            enabled_logs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabledLogs").unwrap(),
            ),
            eventhub_authorization_rule_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventhubAuthorizationRuleId").unwrap(),
            ),
            eventhub_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventhubName").unwrap(),
            ),
            log_analytics_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logAnalyticsWorkspaceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
        }
    }
}