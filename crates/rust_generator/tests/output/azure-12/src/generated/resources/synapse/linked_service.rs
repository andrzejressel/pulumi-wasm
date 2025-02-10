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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod linked_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceArgs {
        /// A map of additional properties to associate with the Synapse Linked Service.
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Synapse Linked Service.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The description for the Synapse Linked Service.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `integration_runtime` block as defined below.
        #[builder(into, default)]
        pub integration_runtime: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::synapse::LinkedServiceIntegrationRuntime>,
        >,
        /// The name which should be used for this Synapse Linked Service. Changing this forces a new Synapse Linked Service to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of parameters to associate with the Synapse Linked Service.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Synapse Workspace ID in which to associate the Linked Service with. Changing this forces a new Synapse Linked Service to be created.
        #[builder(into)]
        pub synapse_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
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
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A JSON object that contains the properties of the Synapse Linked Service.
        #[builder(into)]
        pub type_properties_json: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceResult {
        /// A map of additional properties to associate with the Synapse Linked Service.
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Synapse Linked Service.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The description for the Synapse Linked Service.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `integration_runtime` block as defined below.
        pub integration_runtime: pulumi_gestalt_rust::Output<
            Option<super::super::types::synapse::LinkedServiceIntegrationRuntime>,
        >,
        /// The name which should be used for this Synapse Linked Service. Changing this forces a new Synapse Linked Service to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of parameters to associate with the Synapse Linked Service.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Synapse Workspace ID in which to associate the Linked Service with. Changing this forces a new Synapse Linked Service to be created.
        pub synapse_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The type of data stores that will be connected to Synapse. Valid Values include `AmazonMWS`, `AmazonRdsForOracle`, `AmazonRdsForSqlServer`, `AmazonRedshift`, `AmazonS3`, `AzureBatch`. Changing this forces a new resource to be created.
        /// `AzureBlobFS`, `AzureBlobStorage`, `AzureDataExplorer`, `AzureDataLakeAnalytics`, `AzureDataLakeStore`, `AzureDatabricks`, `AzureDatabricksDeltaLake`, `AzureFileStorage`, `AzureFunction`,
        /// `AzureKeyVault`, `AzureML`, `AzureMLService`, `AzureMariaDB`, `AzureMySql`, `AzurePostgreSql`, `AzureSqlDW`, `AzureSqlDatabase`, `AzureSqlMI`, `AzureSearch`, `AzureStorage`,
        /// `AzureTableStorage`, `Cassandra`, `CommonDataServiceForApps`, `Concur`, `CosmosDb`, `CosmosDbMongoDbApi`, `Couchbase`, `CustomDataSource`, `Db2`, `Drill`,
        /// `Dynamics`, `DynamicsAX`, `DynamicsCrm`, `Eloqua`, `FileServer`, `FtpServer`, `GoogleAdWords`, `GoogleBigQuery`, `GoogleCloudStorage`, `Greenplum`, `HBase`, `HDInsight`,
        /// `HDInsightOnDemand`, `HttpServer`, `Hdfs`, `Hive`, `Hubspot`, `Impala`, `Informix`, `Jira`, `LinkedService`, `Magento`, `MariaDB`, `Marketo`, `MicrosoftAccess`, `MongoDb`,
        /// `MongoDbAtlas`, `MongoDbV2`, `MySql`, `Netezza`, `OData`, `Odbc`, `Office365`, `Oracle`, `OracleServiceCloud`, `Paypal`, `Phoenix`, `PostgreSql`, `Presto`, `QuickBooks`,
        /// `Responsys`, `RestService`, `SqlServer`, `Salesforce`, `SalesforceMarketingCloud`, `SalesforceServiceCloud`, `SapBW`, `SapCloudForCustomer`, `SapEcc`, `SapHana`, `SapOpenHub`,
        /// `SapTable`, `ServiceNow`, `Sftp`, `SharePointOnlineList`, `Shopify`, `Snowflake`, `Spark`, `Square`, `Sybase`, `Teradata`, `Vertica`, `Web`, `Xero`, `Zoho`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// A JSON object that contains the properties of the Synapse Linked Service.
        pub type_properties_json: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LinkedServiceArgs,
    ) -> LinkedServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_properties_binding = args
            .additional_properties
            .get_output(context);
        let annotations_binding = args.annotations.get_output(context);
        let description_binding = args.description.get_output(context);
        let integration_runtime_binding = args.integration_runtime.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let synapse_workspace_id_binding = args.synapse_workspace_id.get_output(context);
        let type__binding = args.type_.get_output(context);
        let type_properties_json_binding = args.type_properties_json.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:synapse/linkedService:LinkedService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalProperties".into(),
                    value: additional_properties_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: annotations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationRuntime".into(),
                    value: integration_runtime_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "synapseWorkspaceId".into(),
                    value: synapse_workspace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "typePropertiesJson".into(),
                    value: type_properties_json_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LinkedServiceResult {
            additional_properties: o.get_field("additionalProperties"),
            annotations: o.get_field("annotations"),
            description: o.get_field("description"),
            integration_runtime: o.get_field("integrationRuntime"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            synapse_workspace_id: o.get_field("synapseWorkspaceId"),
            type_: o.get_field("type"),
            type_properties_json: o.get_field("typePropertiesJson"),
        }
    }
}
