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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod linked_service_azure_databricks {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceAzureDatabricksArgs {
        /// Authenticate to ADB via an access token.
        #[builder(into, default)]
        pub access_token: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The domain URL of the databricks instance.
        #[builder(into)]
        pub adb_domain: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of additional properties to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Linked Service.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The cluster_id of an existing cluster within the linked ADB instance.
        #[builder(into, default)]
        pub existing_cluster_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Leverages an instance pool within the linked ADB instance as one `instance_pool` block defined below.
        #[builder(into, default)]
        pub instance_pool: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::LinkedServiceAzureDatabricksInstancePool,
            >,
        >,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub integration_runtime_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Authenticate to ADB via Azure Key Vault Linked Service as defined in the `key_vault_password` block below.
        #[builder(into, default)]
        pub key_vault_password: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::LinkedServiceAzureDatabricksKeyVaultPassword,
            >,
        >,
        /// Authenticate to ADB via managed service identity.
        #[builder(into, default)]
        pub msi_work_space_resource_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates new clusters within the linked ADB instance as defined in the `new_cluster_config` block below.
        #[builder(into, default)]
        pub new_cluster_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::LinkedServiceAzureDatabricksNewClusterConfig,
            >,
        >,
        /// A map of parameters to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceAzureDatabricksResult {
        /// Authenticate to ADB via an access token.
        pub access_token: pulumi_gestalt_rust::Output<Option<String>>,
        /// The domain URL of the databricks instance.
        pub adb_domain: pulumi_gestalt_rust::Output<String>,
        /// A map of additional properties to associate with the Data Factory Linked Service.
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Linked Service.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The cluster_id of an existing cluster within the linked ADB instance.
        pub existing_cluster_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Leverages an instance pool within the linked ADB instance as one `instance_pool` block defined below.
        pub instance_pool: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureDatabricksInstancePool,
            >,
        >,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        pub integration_runtime_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Authenticate to ADB via Azure Key Vault Linked Service as defined in the `key_vault_password` block below.
        pub key_vault_password: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureDatabricksKeyVaultPassword,
            >,
        >,
        /// Authenticate to ADB via managed service identity.
        pub msi_work_space_resource_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates new clusters within the linked ADB instance as defined in the `new_cluster_config` block below.
        pub new_cluster_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureDatabricksNewClusterConfig,
            >,
        >,
        /// A map of parameters to associate with the Data Factory Linked Service.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LinkedServiceAzureDatabricksArgs,
    ) -> LinkedServiceAzureDatabricksResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let access_token_binding_1 = args.access_token.get_output(context);
        let access_token_binding = access_token_binding_1.get_inner();
        let adb_domain_binding_1 = args.adb_domain.get_output(context);
        let adb_domain_binding = adb_domain_binding_1.get_inner();
        let additional_properties_binding_1 = args
            .additional_properties
            .get_output(context);
        let additional_properties_binding = additional_properties_binding_1.get_inner();
        let annotations_binding_1 = args.annotations.get_output(context);
        let annotations_binding = annotations_binding_1.get_inner();
        let data_factory_id_binding_1 = args.data_factory_id.get_output(context);
        let data_factory_id_binding = data_factory_id_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let existing_cluster_id_binding_1 = args.existing_cluster_id.get_output(context);
        let existing_cluster_id_binding = existing_cluster_id_binding_1.get_inner();
        let instance_pool_binding_1 = args.instance_pool.get_output(context);
        let instance_pool_binding = instance_pool_binding_1.get_inner();
        let integration_runtime_name_binding_1 = args
            .integration_runtime_name
            .get_output(context);
        let integration_runtime_name_binding = integration_runtime_name_binding_1
            .get_inner();
        let key_vault_password_binding_1 = args.key_vault_password.get_output(context);
        let key_vault_password_binding = key_vault_password_binding_1.get_inner();
        let msi_work_space_resource_id_binding_1 = args
            .msi_work_space_resource_id
            .get_output(context);
        let msi_work_space_resource_id_binding = msi_work_space_resource_id_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let new_cluster_config_binding_1 = args.new_cluster_config.get_output(context);
        let new_cluster_config_binding = new_cluster_config_binding_1.get_inner();
        let parameters_binding_1 = args.parameters.get_output(context);
        let parameters_binding = parameters_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        LinkedServiceAzureDatabricksResult {
            access_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessToken"),
            ),
            adb_domain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("adbDomain"),
            ),
            additional_properties: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("additionalProperties"),
            ),
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            data_factory_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataFactoryId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            existing_cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("existingClusterId"),
            ),
            instance_pool: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instancePool"),
            ),
            integration_runtime_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("integrationRuntimeName"),
            ),
            key_vault_password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultPassword"),
            ),
            msi_work_space_resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("msiWorkSpaceResourceId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            new_cluster_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("newClusterConfig"),
            ),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
        }
    }
}
