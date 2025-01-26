/// Manages a Stream Analytics Output to Microsoft SQL Server Database.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: rg-example
///       location: West Europe
///   exampleSqlServer:
///     type: azurerm:sqlServer
///     name: example
///     properties:
///       name: example-server
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       version: '12.0'
///       administratorLogin: dbadmin
///       administratorLoginPassword: example-password
///   exampleSqlDatabase:
///     type: azurerm:sqlDatabase
///     name: example
///     properties:
///       name: exampledb
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       serverName: ${exampleSqlServer.name}
///       requestedServiceObjectiveName: S0
///       collation: SQL_LATIN1_GENERAL_CP1_CI_AS
///       maxSizeBytes: '268435456000'
///       createMode: Default
///   exampleOutputMssql:
///     type: azure:streamanalytics:OutputMssql
///     name: example
///     properties:
///       name: example-output-sql
///       streamAnalyticsJobName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       server: ${exampleSqlServer.fullyQualifiedDomainName}
///       user: ${exampleSqlServer.administratorLogin}
///       password: ${exampleSqlServer.administratorLoginPassword}
///       database: ${exampleSqlDatabase.name}
///       table: ExampleTable
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
/// Stream Analytics Outputs to Microsoft SQL Server Database can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/outputMssql:OutputMssql example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/outputs/output1
/// ```
///
pub mod output_mssql {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutputMssqlArgs {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        #[builder(into, default)]
        pub authentication_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The MS SQL database name where the reference table exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database: pulumi_wasm_rust::InputOrOutput<String>,
        /// The max batch count to write to the SQL Database. Defaults to `10000`. Possible values are between `1` and `1073741824`.
        #[builder(into, default)]
        pub max_batch_count: pulumi_wasm_rust::InputOrOutput<Option<f64>>,
        /// The max writer count for the SQL Database. Defaults to `1`. Possible values are `0` which bases the writer count on the query partition and `1` which corresponds to a single writer.
        #[builder(into, default)]
        pub max_writer_count: pulumi_wasm_rust::InputOrOutput<Option<f64>>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Password used together with username, to login to the Microsoft SQL Server. Required if `authentication_mode` is `ConnectionString`.
        #[builder(into, default)]
        pub password: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The SQL server url. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Table in the database that the output points to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub table: pulumi_wasm_rust::InputOrOutput<String>,
        /// Username used to login to the Microsoft SQL Server. Changing this forces a new resource to be created. Required if `authentication_mode` is `ConnectionString`.
        #[builder(into, default)]
        pub user: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OutputMssqlResult {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        pub authentication_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The MS SQL database name where the reference table exists. Changing this forces a new resource to be created.
        pub database: pulumi_wasm_rust::Output<String>,
        /// The max batch count to write to the SQL Database. Defaults to `10000`. Possible values are between `1` and `1073741824`.
        pub max_batch_count: pulumi_wasm_rust::Output<Option<f64>>,
        /// The max writer count for the SQL Database. Defaults to `1`. Possible values are `0` which bases the writer count on the query partition and `1` which corresponds to a single writer.
        pub max_writer_count: pulumi_wasm_rust::Output<Option<f64>>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Password used together with username, to login to the Microsoft SQL Server. Required if `authentication_mode` is `ConnectionString`.
        pub password: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SQL server url. Changing this forces a new resource to be created.
        pub server: pulumi_wasm_rust::Output<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_wasm_rust::Output<String>,
        /// Table in the database that the output points to. Changing this forces a new resource to be created.
        pub table: pulumi_wasm_rust::Output<String>,
        /// Username used to login to the Microsoft SQL Server. Changing this forces a new resource to be created. Required if `authentication_mode` is `ConnectionString`.
        pub user: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: OutputMssqlArgs,
    ) -> OutputMssqlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_mode_binding = args
            .authentication_mode
            .get_output(context)
            .get_inner();
        let database_binding = args.database.get_output(context).get_inner();
        let max_batch_count_binding = args
            .max_batch_count
            .get_output(context)
            .get_inner();
        let max_writer_count_binding = args
            .max_writer_count
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let server_binding = args.server.get_output(context).get_inner();
        let stream_analytics_job_name_binding = args
            .stream_analytics_job_name
            .get_output(context)
            .get_inner();
        let table_binding = args.table.get_output(context).get_inner();
        let user_binding = args.user.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/outputMssql:OutputMssql".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationMode".into(),
                    value: &authentication_mode_binding,
                },
                register_interface::ObjectField {
                    name: "database".into(),
                    value: &database_binding,
                },
                register_interface::ObjectField {
                    name: "maxBatchCount".into(),
                    value: &max_batch_count_binding,
                },
                register_interface::ObjectField {
                    name: "maxWriterCount".into(),
                    value: &max_writer_count_binding,
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
                    name: "user".into(),
                    value: &user_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        OutputMssqlResult {
            authentication_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authenticationMode"),
            ),
            database: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("database"),
            ),
            max_batch_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maxBatchCount"),
            ),
            max_writer_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maxWriterCount"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            server: pulumi_wasm_rust::__private::into_domain(o.extract_field("server")),
            stream_analytics_job_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("streamAnalyticsJobName"),
            ),
            table: pulumi_wasm_rust::__private::into_domain(o.extract_field("table")),
            user: pulumi_wasm_rust::__private::into_domain(o.extract_field("user")),
        }
    }
}
