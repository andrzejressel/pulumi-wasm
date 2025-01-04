/// Manages a Stream Analytics Reference Input from MS SQL. Reference data (also known as a lookup table) is a finite data set that is static or slowly changing in nature, used to perform a lookup or to correlate with your data stream. Learn more [here](https://docs.microsoft.com/azure/stream-analytics/stream-analytics-use-reference-data#azure-sql-database).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleServer:
///     type: azure:mssql:Server
///     name: example
///     properties:
///       name: example-sqlserver
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       version: '12.0'
///       administratorLogin: admin
///       administratorLoginPassword: password
///   exampleDatabase:
///     type: azure:mssql:Database
///     name: example
///     properties:
///       name: example-db
///       serverId: ${exampleServer.id}
///   exampleReferenceInputMssql:
///     type: azure:streamanalytics:ReferenceInputMssql
///     name: example
///     properties:
///       name: example-reference-input
///       resourceGroupName: ${example.resourceGroupName}
///       streamAnalyticsJobName: ${example.name}
///       server: ${exampleServer.fullyQualifiedDomainName}
///       database: ${exampleDatabase.name}
///       username: exampleuser
///       password: examplepassword
///       refreshType: RefreshPeriodicallyWithFull
///       refreshIntervalDuration: 00:20:00
///       fullSnapshotQuery: |2
///             SELECT *
///             INTO [YourOutputAlias]
///             FROM [YourInputAlias]
/// variables:
///   example:
///     fn::invoke:
///       function: azure:streamanalytics:getJob
///       arguments:
///         name: example-job
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// Stream Analytics can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/referenceInputMssql:ReferenceInputMssql example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/inputs/input1
/// ```
///
pub mod reference_input_mssql {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReferenceInputMssqlArgs {
        /// The MS SQL database name where the reference data exists.
        #[builder(into)]
        pub database: pulumi_wasm_rust::Output<String>,
        /// The query used to retrieve incremental changes in the reference data from the MS SQL database. Cannot be set when `refresh_type` is `Static`.
        #[builder(into, default)]
        pub delta_snapshot_query: pulumi_wasm_rust::Output<Option<String>>,
        /// The query used to retrieve the reference data from the MS SQL database.
        #[builder(into)]
        pub full_snapshot_query: pulumi_wasm_rust::Output<String>,
        /// The name of the Reference Input MS SQL data. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The password to connect to the MS SQL database.
        #[builder(into)]
        pub password: pulumi_wasm_rust::Output<String>,
        /// The frequency in `hh:mm:ss` with which the reference data should be retrieved from the MS SQL database e.g. `00:20:00` for every 20 minutes. Must be set when `refresh_type` is `RefreshPeriodicallyWithFull` or `RefreshPeriodicallyWithDelta`.
        #[builder(into, default)]
        pub refresh_interval_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines whether and how the reference data should be refreshed. Accepted values are `Static`, `RefreshPeriodicallyWithFull` and `RefreshPeriodicallyWithDelta`.
        #[builder(into)]
        pub refresh_type: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Job should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The fully qualified domain name of the MS SQL server.
        #[builder(into)]
        pub server: pulumi_wasm_rust::Output<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_wasm_rust::Output<String>,
        /// The name of the table in the Azure SQL database.
        #[builder(into, default)]
        pub table: pulumi_wasm_rust::Output<Option<String>>,
        /// The username to connect to the MS SQL database.
        #[builder(into)]
        pub username: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ReferenceInputMssqlResult {
        /// The MS SQL database name where the reference data exists.
        pub database: pulumi_wasm_rust::Output<String>,
        /// The query used to retrieve incremental changes in the reference data from the MS SQL database. Cannot be set when `refresh_type` is `Static`.
        pub delta_snapshot_query: pulumi_wasm_rust::Output<Option<String>>,
        /// The query used to retrieve the reference data from the MS SQL database.
        pub full_snapshot_query: pulumi_wasm_rust::Output<String>,
        /// The name of the Reference Input MS SQL data. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The password to connect to the MS SQL database.
        pub password: pulumi_wasm_rust::Output<String>,
        /// The frequency in `hh:mm:ss` with which the reference data should be retrieved from the MS SQL database e.g. `00:20:00` for every 20 minutes. Must be set when `refresh_type` is `RefreshPeriodicallyWithFull` or `RefreshPeriodicallyWithDelta`.
        pub refresh_interval_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines whether and how the reference data should be refreshed. Accepted values are `Static`, `RefreshPeriodicallyWithFull` and `RefreshPeriodicallyWithDelta`.
        pub refresh_type: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Job should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The fully qualified domain name of the MS SQL server.
        pub server: pulumi_wasm_rust::Output<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_wasm_rust::Output<String>,
        /// The name of the table in the Azure SQL database.
        pub table: pulumi_wasm_rust::Output<Option<String>>,
        /// The username to connect to the MS SQL database.
        pub username: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ReferenceInputMssqlArgs,
    ) -> ReferenceInputMssqlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let database_binding = args.database.get_inner();
        let delta_snapshot_query_binding = args.delta_snapshot_query.get_inner();
        let full_snapshot_query_binding = args.full_snapshot_query.get_inner();
        let name_binding = args.name.get_inner();
        let password_binding = args.password.get_inner();
        let refresh_interval_duration_binding = args
            .refresh_interval_duration
            .get_inner();
        let refresh_type_binding = args.refresh_type.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let server_binding = args.server.get_inner();
        let stream_analytics_job_name_binding = args
            .stream_analytics_job_name
            .get_inner();
        let table_binding = args.table.get_inner();
        let username_binding = args.username.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/referenceInputMssql:ReferenceInputMssql"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "database".into(),
                    value: &database_binding,
                },
                register_interface::ObjectField {
                    name: "deltaSnapshotQuery".into(),
                    value: &delta_snapshot_query_binding,
                },
                register_interface::ObjectField {
                    name: "fullSnapshotQuery".into(),
                    value: &full_snapshot_query_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "refreshIntervalDuration".into(),
                    value: &refresh_interval_duration_binding,
                },
                register_interface::ObjectField {
                    name: "refreshType".into(),
                    value: &refresh_type_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "server".into(),
                    value: &server_binding,
                },
                register_interface::ObjectField {
                    name: "streamAnalyticsJobName".into(),
                    value: &stream_analytics_job_name_binding,
                },
                register_interface::ObjectField {
                    name: "table".into(),
                    value: &table_binding,
                },
                register_interface::ObjectField {
                    name: "username".into(),
                    value: &username_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "database".into(),
                },
                register_interface::ResultField {
                    name: "deltaSnapshotQuery".into(),
                },
                register_interface::ResultField {
                    name: "fullSnapshotQuery".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
                register_interface::ResultField {
                    name: "refreshIntervalDuration".into(),
                },
                register_interface::ResultField {
                    name: "refreshType".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "server".into(),
                },
                register_interface::ResultField {
                    name: "streamAnalyticsJobName".into(),
                },
                register_interface::ResultField {
                    name: "table".into(),
                },
                register_interface::ResultField {
                    name: "username".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReferenceInputMssqlResult {
            database: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("database").unwrap(),
            ),
            delta_snapshot_query: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deltaSnapshotQuery").unwrap(),
            ),
            full_snapshot_query: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fullSnapshotQuery").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
            refresh_interval_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("refreshIntervalDuration").unwrap(),
            ),
            refresh_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("refreshType").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            server: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("server").unwrap(),
            ),
            stream_analytics_job_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamAnalyticsJobName").unwrap(),
            ),
            table: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("table").unwrap(),
            ),
            username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("username").unwrap(),
            ),
        }
    }
}
