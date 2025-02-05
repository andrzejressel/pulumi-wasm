/// Manages a MS SQL Server Extended Auditing Policy.
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
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("examplesa")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder()
///             .administrator_login("missadministrator")
///             .administrator_login_password("AdminPassword123!")
///             .location("${example.location}")
///             .name("example-sqlserver")
///             .resource_group_name("${example.name}")
///             .version("12.0")
///             .build_struct(),
///     );
///     let exampleServerExtendedAuditingPolicy = server_extended_auditing_policy::create(
///         "exampleServerExtendedAuditingPolicy",
///         ServerExtendedAuditingPolicyArgs::builder()
///             .retention_in_days(6)
///             .server_id("${exampleServer.id}")
///             .storage_account_access_key("${exampleAccount.primaryAccessKey}")
///             .storage_account_access_key_is_secondary(false)
///             .storage_endpoint("${exampleAccount.primaryBlobEndpoint}")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### With Storage Account Behind VNet And Firewall
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: virtnetname-1
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: subnetname-1
///       resourceGroupName: ${exampleResourceGroup.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///       serviceEndpoints:
///         - Microsoft.Sql
///         - Microsoft.Storage
///       enforcePrivateLinkEndpointNetworkPolicies: true
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${primary.id}
///       roleDefinitionName: Storage Blob Data Contributor
///       principalId: ${exampleServer.identity.principalId}
///   exampleServer:
///     type: azure:mssql:Server
///     name: example
///     properties:
///       name: example-sqlserver
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       version: '12.0'
///       administratorLogin: missadministrator
///       administratorLoginPassword: AdminPassword123!
///       minimumTlsVersion: '1.2'
///       identity:
///         type: SystemAssigned
///   sqlvnetrule:
///     type: azurerm:sqlVirtualNetworkRule
///     properties:
///       name: sql-vnet-rule
///       resourceGroupName: ${exampleResourceGroup.name}
///       serverName: ${exampleServer.name}
///       subnetId: ${exampleSubnet.id}
///   exampleSqlFirewallRule:
///     type: azurerm:sqlFirewallRule
///     name: example
///     properties:
///       name: FirewallRule1
///       resourceGroupName: ${exampleResourceGroup.name}
///       serverName: ${exampleServer.name}
///       startIpAddress: 0.0.0.0
///       endIpAddress: 0.0.0.0
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplesa
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       accountKind: StorageV2
///       allowNestedItemsToBePublic: false
///       networkRules:
///         defaultAction: Deny
///         ipRules:
///           - 127.0.0.1
///         virtualNetworkSubnetIds:
///           - ${exampleSubnet.id}
///         bypasses:
///           - AzureServices
///       identity:
///         type: SystemAssigned
///   exampleServerExtendedAuditingPolicy:
///     type: azure:mssql:ServerExtendedAuditingPolicy
///     name: example
///     properties:
///       storageEndpoint: ${exampleAccount.primaryBlobEndpoint}
///       serverId: ${exampleServer.id}
///       retentionInDays: 6
///       logMonitoringEnabled: false
///       storageAccountSubscriptionId: ${primaryAzurermSubscription.subscriptionId}
///     options:
///       dependsOn:
///         - ${exampleAssignment}
///         - ${exampleAccount}
/// variables:
///   primary:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
///   example:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
///
/// ## Import
///
/// MS SQL Server Extended Auditing Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/serverExtendedAuditingPolicy:ServerExtendedAuditingPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Sql/servers/sqlServer1/extendedAuditingSettings/default
/// ```
///
pub mod server_extended_auditing_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerExtendedAuditingPolicyArgs {
        /// A list of Actions-Groups and Actions to audit.
        #[builder(into, default)]
        pub audit_actions_and_groups: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Whether to enable the extended auditing policy. Possible values are `true` and `false`. Defaults to `true`.
        ///
        /// ->**NOTE:**  If `enabled` is `true`, `storage_endpoint` or `log_monitoring_enabled` are required.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Enable audit events to Azure Monitor? To enable server audit events to Azure Monitor, please enable its main database audit events to Azure Monitor. Defaults to `true`.
        #[builder(into, default)]
        pub log_monitoring_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies condition of where clause when creating an audit.
        #[builder(into, default)]
        pub predicate_expression: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The number of days to retain logs for in the storage account. Defaults to `0`.
        #[builder(into, default)]
        pub retention_in_days: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The ID of the SQL Server to set the extended auditing policy. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The access key to use for the auditing storage account.
        #[builder(into, default)]
        pub storage_account_access_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Is `storage_account_access_key` value the storage's secondary key?
        #[builder(into, default)]
        pub storage_account_access_key_is_secondary: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The ID of the Subscription containing the Storage Account.
        #[builder(into, default)]
        pub storage_account_subscription_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all extended auditing logs.
        #[builder(into, default)]
        pub storage_endpoint: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ServerExtendedAuditingPolicyResult {
        /// A list of Actions-Groups and Actions to audit.
        pub audit_actions_and_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// Whether to enable the extended auditing policy. Possible values are `true` and `false`. Defaults to `true`.
        ///
        /// ->**NOTE:**  If `enabled` is `true`, `storage_endpoint` or `log_monitoring_enabled` are required.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enable audit events to Azure Monitor? To enable server audit events to Azure Monitor, please enable its main database audit events to Azure Monitor. Defaults to `true`.
        pub log_monitoring_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies condition of where clause when creating an audit.
        pub predicate_expression: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of days to retain logs for in the storage account. Defaults to `0`.
        pub retention_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the SQL Server to set the extended auditing policy. Changing this forces a new resource to be created.
        pub server_id: pulumi_wasm_rust::Output<String>,
        /// The access key to use for the auditing storage account.
        pub storage_account_access_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Is `storage_account_access_key` value the storage's secondary key?
        pub storage_account_access_key_is_secondary: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// The ID of the Subscription containing the Storage Account.
        pub storage_account_subscription_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all extended auditing logs.
        pub storage_endpoint: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServerExtendedAuditingPolicyArgs,
    ) -> ServerExtendedAuditingPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let audit_actions_and_groups_binding = args
            .audit_actions_and_groups
            .get_output(context)
            .get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let log_monitoring_enabled_binding = args
            .log_monitoring_enabled
            .get_output(context)
            .get_inner();
        let predicate_expression_binding = args
            .predicate_expression
            .get_output(context)
            .get_inner();
        let retention_in_days_binding = args
            .retention_in_days
            .get_output(context)
            .get_inner();
        let server_id_binding = args.server_id.get_output(context).get_inner();
        let storage_account_access_key_binding = args
            .storage_account_access_key
            .get_output(context)
            .get_inner();
        let storage_account_access_key_is_secondary_binding = args
            .storage_account_access_key_is_secondary
            .get_output(context)
            .get_inner();
        let storage_account_subscription_id_binding = args
            .storage_account_subscription_id
            .get_output(context)
            .get_inner();
        let storage_endpoint_binding = args
            .storage_endpoint
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/serverExtendedAuditingPolicy:ServerExtendedAuditingPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "auditActionsAndGroups".into(),
                    value: &audit_actions_and_groups_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "logMonitoringEnabled".into(),
                    value: &log_monitoring_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "predicateExpression".into(),
                    value: &predicate_expression_binding,
                },
                register_interface::ObjectField {
                    name: "retentionInDays".into(),
                    value: &retention_in_days_binding,
                },
                register_interface::ObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountAccessKey".into(),
                    value: &storage_account_access_key_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountAccessKeyIsSecondary".into(),
                    value: &storage_account_access_key_is_secondary_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountSubscriptionId".into(),
                    value: &storage_account_subscription_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageEndpoint".into(),
                    value: &storage_endpoint_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServerExtendedAuditingPolicyResult {
            audit_actions_and_groups: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("auditActionsAndGroups"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            log_monitoring_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logMonitoringEnabled"),
            ),
            predicate_expression: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("predicateExpression"),
            ),
            retention_in_days: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("retentionInDays"),
            ),
            server_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serverId"),
            ),
            storage_account_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountAccessKey"),
            ),
            storage_account_access_key_is_secondary: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountAccessKeyIsSecondary"),
            ),
            storage_account_subscription_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountSubscriptionId"),
            ),
            storage_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageEndpoint"),
            ),
        }
    }
}
