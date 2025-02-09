/// Manages a MS SQL Database Extended Auditing Policy.
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
///     let exampleDatabase = database::create(
///         "exampleDatabase",
///         DatabaseArgs::builder()
///             .name("example-db")
///             .server_id("${exampleServer.id}")
///             .build_struct(),
///     );
///     let exampleDatabaseExtendedAuditingPolicy = database_extended_auditing_policy::create(
///         "exampleDatabaseExtendedAuditingPolicy",
///         DatabaseExtendedAuditingPolicyArgs::builder()
///             .database_id("${exampleDatabase.id}")
///             .retention_in_days(6)
///             .storage_account_access_key("${exampleAccount.primaryAccessKey}")
///             .storage_account_access_key_is_secondary(false)
///             .storage_endpoint("${exampleAccount.primaryBlobEndpoint}")
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
/// }
/// ```
///
/// ## Import
///
/// MS SQL Database Extended Auditing Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/databaseExtendedAuditingPolicy:DatabaseExtendedAuditingPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Sql/servers/sqlServer1/databases/db1/extendedAuditingSettings/default
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod database_extended_auditing_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseExtendedAuditingPolicyArgs {
        /// The ID of the SQL database to set the extended auditing policy. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to enable the extended auditing policy. Possible values are `true` and `false`. Defaults to `true`.
        ///
        /// ->**NOTE:**  If `enabled` is `true`, `storage_endpoint` or `log_monitoring_enabled` are required.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enable audit events to Azure Monitor? Defaults to `true`.
        ///
        /// > **NOTE:** To enable sending audit events to Log Analytics, please refer to the example which can be found in the `./examples/sql-azure/sql_auditing_log_analytics` directory within the GitHub Repository.  To enable sending server audit events to Log Analytics, please enable the master database to send audit events to Log Analytics.
        /// To enable audit events to Eventhub, please refer to the example which can be found in the `./examples/sql-azure/sql_auditing_eventhub` directory within the GitHub Repository.
        #[builder(into, default)]
        pub log_monitoring_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The number of days to retain logs for in the storage account. Defaults to `0`.
        #[builder(into, default)]
        pub retention_in_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The access key to use for the auditing storage account.
        #[builder(into, default)]
        pub storage_account_access_key: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Is `storage_account_access_key` value the storage's secondary key?
        #[builder(into, default)]
        pub storage_account_access_key_is_secondary: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all extended auditing logs.
        #[builder(into, default)]
        pub storage_endpoint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DatabaseExtendedAuditingPolicyResult {
        /// The ID of the SQL database to set the extended auditing policy. Changing this forces a new resource to be created.
        pub database_id: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable the extended auditing policy. Possible values are `true` and `false`. Defaults to `true`.
        ///
        /// ->**NOTE:**  If `enabled` is `true`, `storage_endpoint` or `log_monitoring_enabled` are required.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Enable audit events to Azure Monitor? Defaults to `true`.
        ///
        /// > **NOTE:** To enable sending audit events to Log Analytics, please refer to the example which can be found in the `./examples/sql-azure/sql_auditing_log_analytics` directory within the GitHub Repository.  To enable sending server audit events to Log Analytics, please enable the master database to send audit events to Log Analytics.
        /// To enable audit events to Eventhub, please refer to the example which can be found in the `./examples/sql-azure/sql_auditing_eventhub` directory within the GitHub Repository.
        pub log_monitoring_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The number of days to retain logs for in the storage account. Defaults to `0`.
        pub retention_in_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The access key to use for the auditing storage account.
        pub storage_account_access_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Is `storage_account_access_key` value the storage's secondary key?
        pub storage_account_access_key_is_secondary: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all extended auditing logs.
        pub storage_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DatabaseExtendedAuditingPolicyArgs,
    ) -> DatabaseExtendedAuditingPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let database_id_binding_1 = args.database_id.get_output(context);
        let database_id_binding = database_id_binding_1.get_inner();
        let enabled_binding_1 = args.enabled.get_output(context);
        let enabled_binding = enabled_binding_1.get_inner();
        let log_monitoring_enabled_binding_1 = args
            .log_monitoring_enabled
            .get_output(context);
        let log_monitoring_enabled_binding = log_monitoring_enabled_binding_1
            .get_inner();
        let retention_in_days_binding_1 = args.retention_in_days.get_output(context);
        let retention_in_days_binding = retention_in_days_binding_1.get_inner();
        let storage_account_access_key_binding_1 = args
            .storage_account_access_key
            .get_output(context);
        let storage_account_access_key_binding = storage_account_access_key_binding_1
            .get_inner();
        let storage_account_access_key_is_secondary_binding_1 = args
            .storage_account_access_key_is_secondary
            .get_output(context);
        let storage_account_access_key_is_secondary_binding = storage_account_access_key_is_secondary_binding_1
            .get_inner();
        let storage_endpoint_binding_1 = args.storage_endpoint.get_output(context);
        let storage_endpoint_binding = storage_endpoint_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/databaseExtendedAuditingPolicy:DatabaseExtendedAuditingPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "databaseId".into(),
                    value: &database_id_binding,
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
                    name: "retentionInDays".into(),
                    value: &retention_in_days_binding,
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
                    name: "storageEndpoint".into(),
                    value: &storage_endpoint_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DatabaseExtendedAuditingPolicyResult {
            database_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseId"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            log_monitoring_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logMonitoringEnabled"),
            ),
            retention_in_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retentionInDays"),
            ),
            storage_account_access_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountAccessKey"),
            ),
            storage_account_access_key_is_secondary: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountAccessKeyIsSecondary"),
            ),
            storage_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageEndpoint"),
            ),
        }
    }
}
