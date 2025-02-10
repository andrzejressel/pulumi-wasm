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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod reference_input_mssql {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReferenceInputMssqlArgs {
        /// The MS SQL database name where the reference data exists.
        #[builder(into)]
        pub database: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The query used to retrieve incremental changes in the reference data from the MS SQL database. Cannot be set when `refresh_type` is `Static`.
        #[builder(into, default)]
        pub delta_snapshot_query: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The query used to retrieve the reference data from the MS SQL database.
        #[builder(into)]
        pub full_snapshot_query: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Reference Input MS SQL data. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The password to connect to the MS SQL database.
        #[builder(into)]
        pub password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The frequency in `hh:mm:ss` with which the reference data should be retrieved from the MS SQL database e.g. `00:20:00` for every 20 minutes. Must be set when `refresh_type` is `RefreshPeriodicallyWithFull` or `RefreshPeriodicallyWithDelta`.
        #[builder(into, default)]
        pub refresh_interval_duration: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Defines whether and how the reference data should be refreshed. Accepted values are `Static`, `RefreshPeriodicallyWithFull` and `RefreshPeriodicallyWithDelta`.
        #[builder(into)]
        pub refresh_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Stream Analytics Job should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The fully qualified domain name of the MS SQL server.
        #[builder(into)]
        pub server: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the table in the Azure SQL database.
        #[builder(into, default)]
        pub table: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The username to connect to the MS SQL database.
        #[builder(into)]
        pub username: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ReferenceInputMssqlResult {
        /// The MS SQL database name where the reference data exists.
        pub database: pulumi_gestalt_rust::Output<String>,
        /// The query used to retrieve incremental changes in the reference data from the MS SQL database. Cannot be set when `refresh_type` is `Static`.
        pub delta_snapshot_query: pulumi_gestalt_rust::Output<Option<String>>,
        /// The query used to retrieve the reference data from the MS SQL database.
        pub full_snapshot_query: pulumi_gestalt_rust::Output<String>,
        /// The name of the Reference Input MS SQL data. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The password to connect to the MS SQL database.
        pub password: pulumi_gestalt_rust::Output<String>,
        /// The frequency in `hh:mm:ss` with which the reference data should be retrieved from the MS SQL database e.g. `00:20:00` for every 20 minutes. Must be set when `refresh_type` is `RefreshPeriodicallyWithFull` or `RefreshPeriodicallyWithDelta`.
        pub refresh_interval_duration: pulumi_gestalt_rust::Output<Option<String>>,
        /// Defines whether and how the reference data should be refreshed. Accepted values are `Static`, `RefreshPeriodicallyWithFull` and `RefreshPeriodicallyWithDelta`.
        pub refresh_type: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Job should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified domain name of the MS SQL server.
        pub server: pulumi_gestalt_rust::Output<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the table in the Azure SQL database.
        pub table: pulumi_gestalt_rust::Output<Option<String>>,
        /// The username to connect to the MS SQL database.
        pub username: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReferenceInputMssqlArgs,
    ) -> ReferenceInputMssqlResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let database_binding = args.database.get_output(context);
        let delta_snapshot_query_binding = args.delta_snapshot_query.get_output(context);
        let full_snapshot_query_binding = args.full_snapshot_query.get_output(context);
        let name_binding = args.name.get_output(context);
        let password_binding = args.password.get_output(context);
        let refresh_interval_duration_binding = args
            .refresh_interval_duration
            .get_output(context);
        let refresh_type_binding = args.refresh_type.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let server_binding = args.server.get_output(context);
        let stream_analytics_job_name_binding = args
            .stream_analytics_job_name
            .get_output(context);
        let table_binding = args.table.get_output(context);
        let username_binding = args.username.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:streamanalytics/referenceInputMssql:ReferenceInputMssql"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "database".into(),
                    value: database_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deltaSnapshotQuery".into(),
                    value: delta_snapshot_query_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fullSnapshotQuery".into(),
                    value: full_snapshot_query_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "refreshIntervalDuration".into(),
                    value: refresh_interval_duration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "refreshType".into(),
                    value: refresh_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "server".into(),
                    value: server_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamAnalyticsJobName".into(),
                    value: stream_analytics_job_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "table".into(),
                    value: table_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "username".into(),
                    value: username_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReferenceInputMssqlResult {
            database: o.get_field("database"),
            delta_snapshot_query: o.get_field("deltaSnapshotQuery"),
            full_snapshot_query: o.get_field("fullSnapshotQuery"),
            name: o.get_field("name"),
            password: o.get_field("password"),
            refresh_interval_duration: o.get_field("refreshIntervalDuration"),
            refresh_type: o.get_field("refreshType"),
            resource_group_name: o.get_field("resourceGroupName"),
            server: o.get_field("server"),
            stream_analytics_job_name: o.get_field("streamAnalyticsJobName"),
            table: o.get_field("table"),
            username: o.get_field("username"),
        }
    }
}
