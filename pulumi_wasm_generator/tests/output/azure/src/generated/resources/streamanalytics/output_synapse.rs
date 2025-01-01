/// Manages a Stream Analytics Output to an Azure Synapse Analytics Workspace.
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestorageacc
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       accountKind: StorageV2
///       isHnsEnabled: 'true'
///   exampleDataLakeGen2Filesystem:
///     type: azure:storage:DataLakeGen2Filesystem
///     name: example
///     properties:
///       name: example
///       storageAccountId: ${exampleAccount.id}
///   exampleWorkspace:
///     type: azure:synapse:Workspace
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       storageDataLakeGen2FilesystemId: ${exampleDataLakeGen2Filesystem.id}
///       sqlAdministratorLogin: sqladminuser
///       sqlAdministratorLoginPassword: H@Sh1CoR3!
///       identity:
///         type: SystemAssigned
///   exampleOutputSynapse:
///     type: azure:streamanalytics:OutputSynapse
///     name: example
///     properties:
///       name: example-output-synapse
///       streamAnalyticsJobName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       server: ${exampleWorkspace.connectivityEndpoints.sqlOnDemand}
///       user: ${exampleWorkspace.sqlAdministratorLogin}
///       password: ${exampleWorkspace.sqlAdministratorLoginPassword}
///       database: master
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
/// A Stream Analytics Output to an Azure Synapse Analytics Workspace can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/outputSynapse:OutputSynapse example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/outputs/output1
/// ```
///
pub mod output_synapse {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutputSynapseArgs {
        /// The name of the Azure SQL database. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database: pulumi_wasm_rust::Output<String>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The password that will be used to connect to the Azure SQL database.
        #[builder(into)]
        pub password: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the SQL server containing the Azure SQL database. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server: pulumi_wasm_rust::Output<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_wasm_rust::Output<String>,
        /// The name of the table in the Azure SQL database. Changing this forces a new resource to be created.
        #[builder(into)]
        pub table: pulumi_wasm_rust::Output<String>,
        /// The user name that will be used to connect to the Azure SQL database. Changing this forces a new resource to be created.
        #[builder(into)]
        pub user: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct OutputSynapseResult {
        /// The name of the Azure SQL database. Changing this forces a new resource to be created.
        pub database: pulumi_wasm_rust::Output<String>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The password that will be used to connect to the Azure SQL database.
        pub password: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the SQL server containing the Azure SQL database. Changing this forces a new resource to be created.
        pub server: pulumi_wasm_rust::Output<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_wasm_rust::Output<String>,
        /// The name of the table in the Azure SQL database. Changing this forces a new resource to be created.
        pub table: pulumi_wasm_rust::Output<String>,
        /// The user name that will be used to connect to the Azure SQL database. Changing this forces a new resource to be created.
        pub user: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: OutputSynapseArgs) -> OutputSynapseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let database_binding = args.database.get_inner();
        let name_binding = args.name.get_inner();
        let password_binding = args.password.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let server_binding = args.server.get_inner();
        let stream_analytics_job_name_binding = args
            .stream_analytics_job_name
            .get_inner();
        let table_binding = args.table.get_inner();
        let user_binding = args.user.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/outputSynapse:OutputSynapse".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "database".into(),
                    value: &database_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "database".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
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
                    name: "user".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OutputSynapseResult {
            database: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("database").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
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
            user: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("user").unwrap(),
            ),
        }
    }
}
