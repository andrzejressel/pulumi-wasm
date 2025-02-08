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
#[allow(clippy::doc_lazy_continuation)]
pub mod diagnostic_setting {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DiagnosticSettingArgs {
        /// One or more `enabled_log` blocks as defined below.
        ///
        /// > **NOTE:** At least one `enabled_log` or `metric` block must be specified. At least one type of Log or Metric must be enabled.
        #[builder(into, default)]
        pub enabled_logs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::DiagnosticSettingEnabledLog>>,
        >,
        /// Specifies the ID of an Event Hub Namespace Authorization Rule used to send Diagnostics Data.
        ///
        /// > **NOTE:** This can be sourced from the `azure.eventhub.EventHubNamespaceAuthorizationRule` resource and is different from a `azure.eventhub.AuthorizationRule` resource.
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        #[builder(into, default)]
        pub eventhub_authorization_rule_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the Event Hub where Diagnostics Data should be sent.
        ///
        /// > **NOTE:** If this isn't specified then the default Event Hub will be used.
        #[builder(into, default)]
        pub eventhub_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Possible values are `AzureDiagnostics` and `Dedicated`. When set to `Dedicated`, logs sent to a Log Analytics workspace will go into resource specific tables, instead of the legacy `AzureDiagnostics` table.
        ///
        /// > **NOTE:** This setting will only have an effect if a `log_analytics_workspace_id` is provided. For some target resource type (e.g., Key Vault), this field is unconfigurable. Please see [resource types](https://learn.microsoft.com/en-us/azure/azure-monitor/reference/tables/azurediagnostics#resource-types) for services that use each method. Please [see the documentation](https://docs.microsoft.com/azure/azure-monitor/platform/diagnostic-logs-stream-log-store#azure-diagnostics-vs-resource-specific) for details on the differences between destination types.
        #[builder(into, default)]
        pub log_analytics_destination_type: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the ID of a Log Analytics Workspace where Diagnostics Data should be sent.
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        #[builder(into, default)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// One or more `metric` blocks as defined below.
        ///
        /// > **NOTE:** At least one `enabled_log` or `metric` block must be specified.
        #[builder(into, default)]
        pub metrics: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::DiagnosticSettingMetric>>,
        >,
        /// Specifies the name of the Diagnostic Setting. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If the name is set to 'service' it will not be possible to fully delete the diagnostic setting. This is due to legacy API support.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the market partner solution where Diagnostics Data should be sent. For potential partner integrations, [click to learn more about partner integration](https://learn.microsoft.com/en-us/azure/partner-solutions/overview).
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        #[builder(into, default)]
        pub partner_solution_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Storage Account where logs should be sent.
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        #[builder(into, default)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of an existing Resource on which to configure Diagnostic Settings. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DiagnosticSettingResult {
        /// One or more `enabled_log` blocks as defined below.
        ///
        /// > **NOTE:** At least one `enabled_log` or `metric` block must be specified. At least one type of Log or Metric must be enabled.
        pub enabled_logs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::DiagnosticSettingEnabledLog>>,
        >,
        /// Specifies the ID of an Event Hub Namespace Authorization Rule used to send Diagnostics Data.
        ///
        /// > **NOTE:** This can be sourced from the `azure.eventhub.EventHubNamespaceAuthorizationRule` resource and is different from a `azure.eventhub.AuthorizationRule` resource.
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        pub eventhub_authorization_rule_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Event Hub where Diagnostics Data should be sent.
        ///
        /// > **NOTE:** If this isn't specified then the default Event Hub will be used.
        pub eventhub_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Possible values are `AzureDiagnostics` and `Dedicated`. When set to `Dedicated`, logs sent to a Log Analytics workspace will go into resource specific tables, instead of the legacy `AzureDiagnostics` table.
        ///
        /// > **NOTE:** This setting will only have an effect if a `log_analytics_workspace_id` is provided. For some target resource type (e.g., Key Vault), this field is unconfigurable. Please see [resource types](https://learn.microsoft.com/en-us/azure/azure-monitor/reference/tables/azurediagnostics#resource-types) for services that use each method. Please [see the documentation](https://docs.microsoft.com/azure/azure-monitor/platform/diagnostic-logs-stream-log-store#azure-diagnostics-vs-resource-specific) for details on the differences between destination types.
        pub log_analytics_destination_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of a Log Analytics Workspace where Diagnostics Data should be sent.
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `metric` blocks as defined below.
        ///
        /// > **NOTE:** At least one `enabled_log` or `metric` block must be specified.
        pub metrics: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::DiagnosticSettingMetric>>,
        >,
        /// Specifies the name of the Diagnostic Setting. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If the name is set to 'service' it will not be possible to fully delete the diagnostic setting. This is due to legacy API support.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the market partner solution where Diagnostics Data should be sent. For potential partner integrations, [click to learn more about partner integration](https://learn.microsoft.com/en-us/azure/partner-solutions/overview).
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        pub partner_solution_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Storage Account where logs should be sent.
        ///
        /// > **NOTE:** At least one of `eventhub_authorization_rule_id`, `log_analytics_workspace_id`, `partner_solution_id` and `storage_account_id` must be specified.
        pub storage_account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of an existing Resource on which to configure Diagnostic Settings. Changing this forces a new resource to be created.
        pub target_resource_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DiagnosticSettingArgs,
    ) -> DiagnosticSettingResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let enabled_logs_binding = args.enabled_logs.get_output(context).get_inner();
        let eventhub_authorization_rule_id_binding = args
            .eventhub_authorization_rule_id
            .get_output(context)
            .get_inner();
        let eventhub_name_binding = args.eventhub_name.get_output(context).get_inner();
        let log_analytics_destination_type_binding = args
            .log_analytics_destination_type
            .get_output(context)
            .get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context)
            .get_inner();
        let metrics_binding = args.metrics.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let partner_solution_id_binding = args
            .partner_solution_id
            .get_output(context)
            .get_inner();
        let storage_account_id_binding = args
            .storage_account_id
            .get_output(context)
            .get_inner();
        let target_resource_id_binding = args
            .target_resource_id
            .get_output(context)
            .get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        DiagnosticSettingResult {
            enabled_logs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabledLogs"),
            ),
            eventhub_authorization_rule_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventhubAuthorizationRuleId"),
            ),
            eventhub_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventhubName"),
            ),
            log_analytics_destination_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logAnalyticsDestinationType"),
            ),
            log_analytics_workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logAnalyticsWorkspaceId"),
            ),
            metrics: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metrics"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            partner_solution_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("partnerSolutionId"),
            ),
            storage_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
            target_resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetResourceId"),
            ),
        }
    }
}
