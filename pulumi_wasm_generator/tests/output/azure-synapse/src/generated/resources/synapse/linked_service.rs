/// Manages a Synapse Linked Service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountKind: BlobStorage
///       accountTier: Standard
///       accountReplicationType: LRS
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
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       storageDataLakeGen2FilesystemId: ${exampleDataLakeGen2Filesystem.id}
///       sqlAdministratorLogin: sqladminuser
///       sqlAdministratorLoginPassword: H@Sh1CoR3!
///       managedVirtualNetworkEnabled: true
///       identity:
///         type: SystemAssigned
///   exampleFirewallRule:
///     type: azure:synapse:FirewallRule
///     name: example
///     properties:
///       name: allowAll
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       startIpAddress: 0.0.0.0
///       endIpAddress: 255.255.255.255
///   exampleIntegrationRuntimeAzure:
///     type: azure:synapse:IntegrationRuntimeAzure
///     name: example
///     properties:
///       name: example
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       location: ${example.location}
///   exampleLinkedService:
///     type: azure:synapse:LinkedService
///     name: example
///     properties:
///       name: example
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       type: AzureBlobStorage
///       typePropertiesJson: |
///         {
///           "connectionString": "${exampleAccount.primaryConnectionString}"
///         }
///       integrationRuntime:
///         name: ${exampleIntegrationRuntimeAzure.name}
///     options:
///       dependsOn:
///         - ${exampleFirewallRule}
/// ```
///
/// ## Import
///
/// Synapse Linked Services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/linkedService:LinkedService example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.Synapse/workspaces/workspace1/linkedServices/linkedservice1
/// ```
///
pub mod linked_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceArgs {
        /// A map of additional properties to associate with the Synapse Linked Service.
        #[builder(into, default)]
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Synapse Linked Service.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The description for the Synapse Linked Service.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A `integration_runtime` block as defined below.
        #[builder(into, default)]
        pub integration_runtime: pulumi_wasm_rust::Output<
            Option<super::super::types::synapse::LinkedServiceIntegrationRuntime>,
        >,
        /// The name which should be used for this Synapse Linked Service. Changing this forces a new Synapse Linked Service to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of parameters to associate with the Synapse Linked Service.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Synapse Workspace ID in which to associate the Linked Service with. Changing this forces a new Synapse Linked Service to be created.
        #[builder(into)]
        pub synapse_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The type of data stores that will be connected to Synapse. Valid Values include `AmazonMWS`, `AmazonRdsForOracle`, `AmazonRdsForSqlServer`, `AmazonRedshift`, `AmazonS3`, `AzureBatch`. Changing this forces a new resource to be created.
        /// `AzureBlobFS`, `AzureBlobStorage`, `AzureDataExplorer`, `AzureDataLakeAnalytics`, `AzureDataLakeStore`, `AzureDatabricks`, `AzureDatabricksDeltaLake`, `AzureFileStorage`, `AzureFunction`,
        /// `AzureKeyVault`, `AzureML`, `AzureMLService`, `AzureMariaDB`, `AzureMySql`, `AzurePostgreSql`, `AzureSqlDW`, `AzureSqlDatabase`, `AzureSqlMI`, `AzureSearch`, `AzureStorage`,
        /// `AzureTableStorage`, `Cassandra`, `CommonDataServiceForApps`, `Concur`, `CosmosDb`, `CosmosDbMongoDbApi`, `Couchbase`, `CustomDataSource`, `Db2`, `Drill`,
        /// `Dynamics`, `DynamicsAX`, `DynamicsCrm`, `Eloqua`, `FileServer`, `FtpServer`, `GoogleAdWords`, `GoogleBigQuery`, `GoogleCloudStorage`, `Greenplum`, `HBase`, `HDInsight`,
        /// `HDInsightOnDemand`, `HttpServer`, `Hdfs`, `Hive`, `Hubspot`, `Impala`, `Informix`, `Jira`, `LinkedService`, `Magento`, `MariaDB`, `Marketo`, `MicrosoftAccess`, `MongoDb`,
        /// `MongoDbAtlas`, `MongoDbV2`, `MySql`, `Netezza`, `OData`, `Odbc`, `Office365`, `Oracle`, `OracleServiceCloud`, `Paypal`, `Phoenix`, `PostgreSql`, `Presto`, `QuickBooks`,
        /// `Responsys`, `RestService`, `SqlServer`, `Salesforce`, `SalesforceMarketingCloud`, `SalesforceServiceCloud`, `SapBW`, `SapCloudForCustomer`, `SapEcc`, `SapHana`, `SapOpenHub`,
        /// `SapTable`, `ServiceNow`, `Sftp`, `SharePointOnlineList`, `Shopify`, `Snowflake`, `Spark`, `Square`, `Sybase`, `Teradata`, `Vertica`, `Web`, `Xero`, `Zoho`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
        /// A JSON object that contains the properties of the Synapse Linked Service.
        #[builder(into)]
        pub type_properties_json: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceResult {
        /// A map of additional properties to associate with the Synapse Linked Service.
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Synapse Linked Service.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The description for the Synapse Linked Service.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A `integration_runtime` block as defined below.
        pub integration_runtime: pulumi_wasm_rust::Output<
            Option<super::super::types::synapse::LinkedServiceIntegrationRuntime>,
        >,
        /// The name which should be used for this Synapse Linked Service. Changing this forces a new Synapse Linked Service to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of parameters to associate with the Synapse Linked Service.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Synapse Workspace ID in which to associate the Linked Service with. Changing this forces a new Synapse Linked Service to be created.
        pub synapse_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The type of data stores that will be connected to Synapse. Valid Values include `AmazonMWS`, `AmazonRdsForOracle`, `AmazonRdsForSqlServer`, `AmazonRedshift`, `AmazonS3`, `AzureBatch`. Changing this forces a new resource to be created.
        /// `AzureBlobFS`, `AzureBlobStorage`, `AzureDataExplorer`, `AzureDataLakeAnalytics`, `AzureDataLakeStore`, `AzureDatabricks`, `AzureDatabricksDeltaLake`, `AzureFileStorage`, `AzureFunction`,
        /// `AzureKeyVault`, `AzureML`, `AzureMLService`, `AzureMariaDB`, `AzureMySql`, `AzurePostgreSql`, `AzureSqlDW`, `AzureSqlDatabase`, `AzureSqlMI`, `AzureSearch`, `AzureStorage`,
        /// `AzureTableStorage`, `Cassandra`, `CommonDataServiceForApps`, `Concur`, `CosmosDb`, `CosmosDbMongoDbApi`, `Couchbase`, `CustomDataSource`, `Db2`, `Drill`,
        /// `Dynamics`, `DynamicsAX`, `DynamicsCrm`, `Eloqua`, `FileServer`, `FtpServer`, `GoogleAdWords`, `GoogleBigQuery`, `GoogleCloudStorage`, `Greenplum`, `HBase`, `HDInsight`,
        /// `HDInsightOnDemand`, `HttpServer`, `Hdfs`, `Hive`, `Hubspot`, `Impala`, `Informix`, `Jira`, `LinkedService`, `Magento`, `MariaDB`, `Marketo`, `MicrosoftAccess`, `MongoDb`,
        /// `MongoDbAtlas`, `MongoDbV2`, `MySql`, `Netezza`, `OData`, `Odbc`, `Office365`, `Oracle`, `OracleServiceCloud`, `Paypal`, `Phoenix`, `PostgreSql`, `Presto`, `QuickBooks`,
        /// `Responsys`, `RestService`, `SqlServer`, `Salesforce`, `SalesforceMarketingCloud`, `SalesforceServiceCloud`, `SapBW`, `SapCloudForCustomer`, `SapEcc`, `SapHana`, `SapOpenHub`,
        /// `SapTable`, `ServiceNow`, `Sftp`, `SharePointOnlineList`, `Shopify`, `Snowflake`, `Spark`, `Square`, `Sybase`, `Teradata`, `Vertica`, `Web`, `Xero`, `Zoho`.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// A JSON object that contains the properties of the Synapse Linked Service.
        pub type_properties_json: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LinkedServiceArgs) -> LinkedServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_properties_binding = args.additional_properties.get_inner();
        let annotations_binding = args.annotations.get_inner();
        let description_binding = args.description.get_inner();
        let integration_runtime_binding = args.integration_runtime.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let synapse_workspace_id_binding = args.synapse_workspace_id.get_inner();
        let type__binding = args.type_.get_inner();
        let type_properties_json_binding = args.type_properties_json.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:synapse/linkedService:LinkedService".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalProperties".into(),
                    value: &additional_properties_binding,
                },
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "integrationRuntime".into(),
                    value: &integration_runtime_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "synapseWorkspaceId".into(),
                    value: &synapse_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "typePropertiesJson".into(),
                    value: &type_properties_json_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalProperties".into(),
                },
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "integrationRuntime".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "synapseWorkspaceId".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "typePropertiesJson".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LinkedServiceResult {
            additional_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalProperties").unwrap(),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            integration_runtime: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationRuntime").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            synapse_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("synapseWorkspaceId").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            type_properties_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("typePropertiesJson").unwrap(),
            ),
        }
    }
}