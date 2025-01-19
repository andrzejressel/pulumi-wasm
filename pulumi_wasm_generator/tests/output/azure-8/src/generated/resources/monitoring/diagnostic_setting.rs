/// Manages a Diagnostic Setting for an existing Resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: storageaccountname
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: examplekeyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       softDeleteRetentionDays: 7
///       purgeProtectionEnabled: false
///       skuName: standard
///   exampleDiagnosticSetting:
///     type: azure:monitoring:DiagnosticSetting
///     name: example
///     properties:
///       name: example
///       targetResourceId: ${exampleKeyVault.id}
///       storageAccountId: ${exampleAccount.id}
///       enabledLogs:
///         - category: AuditEvent
///       metrics:
///         - category: AllMetrics
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Diagnostic Settings can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/diagnosticSetting:DiagnosticSetting example "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.KeyVault/vaults/vault1|logMonitoring1"
/// ```
///
pub mod diagnostic_setting {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DiagnosticSettingArgs {
        /// One or more `enabled_log` blocks as defined below.
        ///
        /// > **NOTE:** At least one `enabled_log` or `metric` block must be specified. At least one type of Log or Metric must be enabled.
        #[builder(into, default)]
        pub enabled_logs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::DiagnosticSettingEnabledLog>>,
        >,
        /// Specifies the ID of an Event Hub Namespace Authorization Rule used to send Diagnostics Data.
        ///
        /// > **NOTE:** This can be sourced from the `azure.eventhub.EventHubNamespaceAuthorizationRule` resource and is different from a `azure.eventhub.AuthorizationRule` resource.
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        #[builder(into, default)]
        pub eventhub_authorization_rule_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Event Hub where Diagnostics Data should be sent.
        ///
        /// > **NOTE:** If this isn't specified then the default Event Hub will be used.
        #[builder(into, default)]
        pub eventhub_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Possible values are `AzureDiagnostics` and `Dedicated`. When set to `Dedicated`, logs sent to a Log Analytics workspace will go into resource specific tables, instead of the legacy `AzureDiagnostics` table.
        ///
        /// > **NOTE:** This setting will only have an effect if a `log_analytics_workspace_id` is provided. For some target resource type (e.g., Key Vault), this field is unconfigurable. Please see [resource types](https://learn.microsoft.com/en-us/azure/azure-monitor/reference/tables/azurediagnostics#resource-types) for services that use each method. Please [see the documentation](https://docs.microsoft.com/azure/azure-monitor/platform/diagnostic-logs-stream-log-store#azure-diagnostics-vs-resource-specific) for details on the differences between destination types.
        #[builder(into, default)]
        pub log_analytics_destination_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of a Log Analytics Workspace where Diagnostics Data should be sent.
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        #[builder(into, default)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `metric` blocks as defined below.
        ///
        /// > **NOTE:** At least one `enabled_log` or `metric` block must be specified.
        #[builder(into, default)]
        pub metrics: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::DiagnosticSettingMetric>>,
        >,
        /// Specifies the name of the Diagnostic Setting. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If the name is set to 'service' it will not be possible to fully delete the diagnostic setting. This is due to legacy API support.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the market partner solution where Diagnostics Data should be sent. For potential partner integrations, [click to learn more about partner integration](https://learn.microsoft.com/en-us/azure/partner-solutions/overview).
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        #[builder(into, default)]
        pub partner_solution_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Storage Account where logs should be sent.
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        #[builder(into, default)]
        pub storage_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of an existing Resource on which to configure Diagnostic Settings. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DiagnosticSettingResult {
        /// One or more `enabled_log` blocks as defined below.
        ///
        /// > **NOTE:** At least one `enabled_log` or `metric` block must be specified. At least one type of Log or Metric must be enabled.
        pub enabled_logs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::DiagnosticSettingEnabledLog>>,
        >,
        /// Specifies the ID of an Event Hub Namespace Authorization Rule used to send Diagnostics Data.
        ///
        /// > **NOTE:** This can be sourced from the `azure.eventhub.EventHubNamespaceAuthorizationRule` resource and is different from a `azure.eventhub.AuthorizationRule` resource.
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        pub eventhub_authorization_rule_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Event Hub where Diagnostics Data should be sent.
        ///
        /// > **NOTE:** If this isn't specified then the default Event Hub will be used.
        pub eventhub_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Possible values are `AzureDiagnostics` and `Dedicated`. When set to `Dedicated`, logs sent to a Log Analytics workspace will go into resource specific tables, instead of the legacy `AzureDiagnostics` table.
        ///
        /// > **NOTE:** This setting will only have an effect if a `log_analytics_workspace_id` is provided. For some target resource type (e.g., Key Vault), this field is unconfigurable. Please see [resource types](https://learn.microsoft.com/en-us/azure/azure-monitor/reference/tables/azurediagnostics#resource-types) for services that use each method. Please [see the documentation](https://docs.microsoft.com/azure/azure-monitor/platform/diagnostic-logs-stream-log-store#azure-diagnostics-vs-resource-specific) for details on the differences between destination types.
        pub log_analytics_destination_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of a Log Analytics Workspace where Diagnostics Data should be sent.
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `metric` blocks as defined below.
        ///
        /// > **NOTE:** At least one `enabled_log` or `metric` block must be specified.
        pub metrics: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::DiagnosticSettingMetric>>,
        >,
        /// Specifies the name of the Diagnostic Setting. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If the name is set to 'service' it will not be possible to fully delete the diagnostic setting. This is due to legacy API support.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the market partner solution where Diagnostics Data should be sent. For potential partner integrations, [click to learn more about partner integration](https://learn.microsoft.com/en-us/azure/partner-solutions/overview).
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        pub partner_solution_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Storage Account where logs should be sent.
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        pub storage_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of an existing Resource on which to configure Diagnostic Settings. Changing this forces a new resource to be created.
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DiagnosticSettingArgs) -> DiagnosticSettingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enabled_logs_binding = args.enabled_logs.get_inner();
        let eventhub_authorization_rule_id_binding = args
            .eventhub_authorization_rule_id
            .get_inner();
        let eventhub_name_binding = args.eventhub_name.get_inner();
        let log_analytics_destination_type_binding = args
            .log_analytics_destination_type
            .get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_inner();
        let metrics_binding = args.metrics.get_inner();
        let name_binding = args.name.get_inner();
        let partner_solution_id_binding = args.partner_solution_id.get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let target_resource_id_binding = args.target_resource_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/diagnosticSetting:DiagnosticSetting".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "logAnalyticsDestinationType".into(),
                    value: &log_analytics_destination_type_binding,
                },
                register_interface::ObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "metrics".into(),
                    value: &metrics_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "partnerSolutionId".into(),
                    value: &partner_solution_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding,
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
                    name: "logAnalyticsDestinationType".into(),
                },
                register_interface::ResultField {
                    name: "logAnalyticsWorkspaceId".into(),
                },
                register_interface::ResultField {
                    name: "metrics".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "partnerSolutionId".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "targetResourceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DiagnosticSettingResult {
            enabled_logs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabledLogs").unwrap(),
            ),
            eventhub_authorization_rule_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventhubAuthorizationRuleId").unwrap(),
            ),
            eventhub_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventhubName").unwrap(),
            ),
            log_analytics_destination_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logAnalyticsDestinationType").unwrap(),
            ),
            log_analytics_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logAnalyticsWorkspaceId").unwrap(),
            ),
            metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metrics").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            partner_solution_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partnerSolutionId").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
            target_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetResourceId").unwrap(),
            ),
        }
    }
}
