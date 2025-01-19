/// Manages a Linked Service (connection) between Azure Databricks and Azure Data Factory.
///
/// ## Example Usage
///
/// ### With Managed Identity & New Cluster
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example
///       location: East US
///   #Create a Linked Service using managed identity and new cluster config
///   exampleFactory:
///     type: azure:datafactory:Factory
///     name: example
///     properties:
///       name: TestDtaFactory92783401247
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       identity:
///         type: SystemAssigned
///   #Create a databricks instance
///   exampleWorkspace:
///     type: azure:databricks:Workspace
///     name: example
///     properties:
///       name: databricks-test
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: standard
///   msiLinked:
///     type: azure:datafactory:LinkedServiceAzureDatabricks
///     name: msi_linked
///     properties:
///       name: ADBLinkedServiceViaMSI
///       dataFactoryId: ${exampleFactory.id}
///       description: ADB Linked Service via MSI
///       adbDomain: https://${exampleWorkspace.workspaceUrl}
///       msiWorkSpaceResourceId: ${exampleWorkspace.id}
///       newClusterConfig:
///         nodeType: Standard_NC12
///         clusterVersion: 5.5.x-gpu-scala2.11
///         minNumberOfWorkers: 1
///         maxNumberOfWorkers: 5
///         driverNodeType: Standard_NC12
///         logDestination: dbfs:/logs
///         customTags:
///           custom_tag1: sct_value_1
///           custom_tag2: sct_value_2
///         sparkConfig:
///           config1: value1
///           config2: value2
///         sparkEnvironmentVariables:
///           envVar1: value1
///           envVar2: value2
///         initScripts:
///           - init.sh
///           - init2.sh
/// ```
///
///
/// ### With Access Token & Existing Cluster
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let atLinked = linked_service_azure_databricks::create(
///         "atLinked",
///         LinkedServiceAzureDatabricksArgs::builder()
///             .access_token("SomeDatabricksAccessToken")
///             .adb_domain("https://${exampleWorkspace.workspaceUrl}")
///             .data_factory_id("${exampleFactory.id}")
///             .description("ADB Linked Service via Access Token")
///             .existing_cluster_id("0308-201146-sly615")
///             .name("ADBLinkedServiceViaAccessToken")
///             .build_struct(),
///     );
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder().location("East US").name("example").build_struct(),
///     );
///     let exampleFactory = factory::create(
///         "exampleFactory",
///         FactoryArgs::builder()
///             .location("${example.location}")
///             .name("TestDtaFactory92783401247")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleWorkspace = workspace::create(
///         "exampleWorkspace",
///         WorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("databricks-test")
///             .resource_group_name("${example.name}")
///             .sku("standard")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory Linked Services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/linkedServiceAzureDatabricks:LinkedServiceAzureDatabricks example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/linkedservices/example
/// ```
///
pub mod linked_service_azure_databricks {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceAzureDatabricksArgs {
        /// Authenticate to ADB via an access token.
        #[builder(into, default)]
        pub access_token: pulumi_wasm_rust::Output<Option<String>>,
        /// The domain URL of the databricks instance.
        #[builder(into)]
        pub adb_domain: pulumi_wasm_rust::Output<String>,
        /// A map of additional properties to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Linked Service.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The cluster_id of an existing cluster within the linked ADB instance.
        #[builder(into, default)]
        pub existing_cluster_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Leverages an instance pool within the linked ADB instance as one `instance_pool` block defined below.
        #[builder(into, default)]
        pub instance_pool: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureDatabricksInstancePool,
            >,
        >,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub integration_runtime_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Authenticate to ADB via Azure Key Vault Linked Service as defined in the `key_vault_password` block below.
        #[builder(into, default)]
        pub key_vault_password: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureDatabricksKeyVaultPassword,
            >,
        >,
        /// Authenticate to ADB via managed service identity.
        #[builder(into, default)]
        pub msi_work_space_resource_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates new clusters within the linked ADB instance as defined in the `new_cluster_config` block below.
        #[builder(into, default)]
        pub new_cluster_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureDatabricksNewClusterConfig,
            >,
        >,
        /// A map of parameters to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceAzureDatabricksResult {
        /// Authenticate to ADB via an access token.
        pub access_token: pulumi_wasm_rust::Output<Option<String>>,
        /// The domain URL of the databricks instance.
        pub adb_domain: pulumi_wasm_rust::Output<String>,
        /// A map of additional properties to associate with the Data Factory Linked Service.
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Linked Service.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The cluster_id of an existing cluster within the linked ADB instance.
        pub existing_cluster_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Leverages an instance pool within the linked ADB instance as one `instance_pool` block defined below.
        pub instance_pool: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureDatabricksInstancePool,
            >,
        >,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        pub integration_runtime_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Authenticate to ADB via Azure Key Vault Linked Service as defined in the `key_vault_password` block below.
        pub key_vault_password: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureDatabricksKeyVaultPassword,
            >,
        >,
        /// Authenticate to ADB via managed service identity.
        pub msi_work_space_resource_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates new clusters within the linked ADB instance as defined in the `new_cluster_config` block below.
        pub new_cluster_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureDatabricksNewClusterConfig,
            >,
        >,
        /// A map of parameters to associate with the Data Factory Linked Service.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: LinkedServiceAzureDatabricksArgs,
    ) -> LinkedServiceAzureDatabricksResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_token_binding = args.access_token.get_inner();
        let adb_domain_binding = args.adb_domain.get_inner();
        let additional_properties_binding = args.additional_properties.get_inner();
        let annotations_binding = args.annotations.get_inner();
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let description_binding = args.description.get_inner();
        let existing_cluster_id_binding = args.existing_cluster_id.get_inner();
        let instance_pool_binding = args.instance_pool.get_inner();
        let integration_runtime_name_binding = args.integration_runtime_name.get_inner();
        let key_vault_password_binding = args.key_vault_password.get_inner();
        let msi_work_space_resource_id_binding = args
            .msi_work_space_resource_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let new_cluster_config_binding = args.new_cluster_config.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/linkedServiceAzureDatabricks:LinkedServiceAzureDatabricks"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessToken".into(),
                    value: &access_token_binding,
                },
                register_interface::ObjectField {
                    name: "adbDomain".into(),
                    value: &adb_domain_binding,
                },
                register_interface::ObjectField {
                    name: "additionalProperties".into(),
                    value: &additional_properties_binding,
                },
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "existingClusterId".into(),
                    value: &existing_cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "instancePool".into(),
                    value: &instance_pool_binding,
                },
                register_interface::ObjectField {
                    name: "integrationRuntimeName".into(),
                    value: &integration_runtime_name_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultPassword".into(),
                    value: &key_vault_password_binding,
                },
                register_interface::ObjectField {
                    name: "msiWorkSpaceResourceId".into(),
                    value: &msi_work_space_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "newClusterConfig".into(),
                    value: &new_cluster_config_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessToken".into(),
                },
                register_interface::ResultField {
                    name: "adbDomain".into(),
                },
                register_interface::ResultField {
                    name: "additionalProperties".into(),
                },
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "dataFactoryId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "existingClusterId".into(),
                },
                register_interface::ResultField {
                    name: "instancePool".into(),
                },
                register_interface::ResultField {
                    name: "integrationRuntimeName".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultPassword".into(),
                },
                register_interface::ResultField {
                    name: "msiWorkSpaceResourceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "newClusterConfig".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LinkedServiceAzureDatabricksResult {
            access_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessToken").unwrap(),
            ),
            adb_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adbDomain").unwrap(),
            ),
            additional_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalProperties").unwrap(),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFactoryId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            existing_cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("existingClusterId").unwrap(),
            ),
            instance_pool: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instancePool").unwrap(),
            ),
            integration_runtime_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationRuntimeName").unwrap(),
            ),
            key_vault_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultPassword").unwrap(),
            ),
            msi_work_space_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("msiWorkSpaceResourceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            new_cluster_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("newClusterConfig").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
        }
    }
}
