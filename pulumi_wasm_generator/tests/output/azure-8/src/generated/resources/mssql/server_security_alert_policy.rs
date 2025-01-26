/// Manages a Security Alert Policy for a MSSQL Server.
///
/// > **NOTE** Security Alert Policy is currently only available for MS SQL databases.
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
///   exampleSqlServer:
///     type: azurerm:sqlServer
///     name: example
///     properties:
///       name: mysqlserver
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       version: '12.0'
///       administratorLogin: 4dm1n157r470r
///       administratorLoginPassword: 4-v3ry-53cr37-p455w0rd
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: accteststorageaccount
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: GRS
///   exampleServerSecurityAlertPolicy:
///     type: azure:mssql:ServerSecurityAlertPolicy
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       serverName: ${exampleSqlServer.name}
///       state: Enabled
///       storageEndpoint: ${exampleAccount.primaryBlobEndpoint}
///       storageAccountAccessKey: ${exampleAccount.primaryAccessKey}
///       disabledAlerts:
///         - Sql_Injection
///         - Data_Exfiltration
///       retentionDays: 20
/// ```
///
/// ## Import
///
/// MS SQL Server Security Alert Policy can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/serverSecurityAlertPolicy:ServerSecurityAlertPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/acceptanceTestResourceGroup1/providers/Microsoft.Sql/servers/mssqlserver/securityAlertPolicies/Default
/// ```
///
pub mod server_security_alert_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerSecurityAlertPolicyArgs {
        /// Specifies an array of alerts that are disabled. Allowed values are: `Sql_Injection`, `Sql_Injection_Vulnerability`, `Access_Anomaly`, `Data_Exfiltration`, `Unsafe_Action`.
        #[builder(into, default)]
        pub disabled_alerts: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Boolean flag which specifies if the alert is sent to the account administrators or not. Defaults to `false`.
        #[builder(into, default)]
        pub email_account_admins: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies an array of email addresses to which the alert is sent.
        #[builder(into, default)]
        pub email_addresses: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the resource group that contains the MS SQL Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the number of days to keep in the Threat Detection audit logs. Defaults to `0`.
        #[builder(into, default)]
        pub retention_days: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Specifies the name of the MS SQL Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the state of the policy, whether it is enabled or disabled or a policy has not been applied yet on the specific database server. Possible values are `Disabled`, `Enabled` and `New`.
        #[builder(into)]
        pub state: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the identifier key of the Threat Detection audit storage account. This is mandatory when you use `storage_endpoint` to specify a storage account blob endpoint.
        ///
        /// > **NOTE:**  Please note that storage accounts configured with `shared_access_key_enabled = false` cannot be used to configure `azure.mssql.ServerSecurityAlertPolicy` with `storage_endpoint` for now.
        #[builder(into, default)]
        pub storage_account_access_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all Threat Detection audit logs.
        #[builder(into, default)]
        pub storage_endpoint: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ServerSecurityAlertPolicyResult {
        /// Specifies an array of alerts that are disabled. Allowed values are: `Sql_Injection`, `Sql_Injection_Vulnerability`, `Access_Anomaly`, `Data_Exfiltration`, `Unsafe_Action`.
        pub disabled_alerts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Boolean flag which specifies if the alert is sent to the account administrators or not. Defaults to `false`.
        pub email_account_admins: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies an array of email addresses to which the alert is sent.
        pub email_addresses: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the resource group that contains the MS SQL Server. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the number of days to keep in the Threat Detection audit logs. Defaults to `0`.
        pub retention_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the MS SQL Server. Changing this forces a new resource to be created.
        pub server_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the state of the policy, whether it is enabled or disabled or a policy has not been applied yet on the specific database server. Possible values are `Disabled`, `Enabled` and `New`.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Specifies the identifier key of the Threat Detection audit storage account. This is mandatory when you use `storage_endpoint` to specify a storage account blob endpoint.
        ///
        /// > **NOTE:**  Please note that storage accounts configured with `shared_access_key_enabled = false` cannot be used to configure `azure.mssql.ServerSecurityAlertPolicy` with `storage_endpoint` for now.
        pub storage_account_access_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all Threat Detection audit logs.
        pub storage_endpoint: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServerSecurityAlertPolicyArgs,
    ) -> ServerSecurityAlertPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let disabled_alerts_binding = args
            .disabled_alerts
            .get_output(context)
            .get_inner();
        let email_account_admins_binding = args
            .email_account_admins
            .get_output(context)
            .get_inner();
        let email_addresses_binding = args
            .email_addresses
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let retention_days_binding = args.retention_days.get_output(context).get_inner();
        let server_name_binding = args.server_name.get_output(context).get_inner();
        let state_binding = args.state.get_output(context).get_inner();
        let storage_account_access_key_binding = args
            .storage_account_access_key
            .get_output(context)
            .get_inner();
        let storage_endpoint_binding = args
            .storage_endpoint
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/serverSecurityAlertPolicy:ServerSecurityAlertPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "disabledAlerts".into(),
                    value: &disabled_alerts_binding,
                },
                register_interface::ObjectField {
                    name: "emailAccountAdmins".into(),
                    value: &email_account_admins_binding,
                },
                register_interface::ObjectField {
                    name: "emailAddresses".into(),
                    value: &email_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "retentionDays".into(),
                    value: &retention_days_binding,
                },
                register_interface::ObjectField {
                    name: "serverName".into(),
                    value: &server_name_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountAccessKey".into(),
                    value: &storage_account_access_key_binding,
                },
                register_interface::ObjectField {
                    name: "storageEndpoint".into(),
                    value: &storage_endpoint_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "disabledAlerts".into(),
                },
                register_interface::ResultField {
                    name: "emailAccountAdmins".into(),
                },
                register_interface::ResultField {
                    name: "emailAddresses".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "retentionDays".into(),
                },
                register_interface::ResultField {
                    name: "serverName".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "storageEndpoint".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServerSecurityAlertPolicyResult {
            disabled_alerts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disabledAlerts").unwrap(),
            ),
            email_account_admins: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailAccountAdmins").unwrap(),
            ),
            email_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailAddresses").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionDays").unwrap(),
            ),
            server_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverName").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            storage_account_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountAccessKey").unwrap(),
            ),
            storage_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageEndpoint").unwrap(),
            ),
        }
    }
}
