/// Manages a Linked Service (connection) between a SFTP Server and Azure Data Factory.
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
///   exampleFactory:
///     type: azure:datafactory:Factory
///     name: example
///     properties:
///       name: example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///   exampleLinkedServiceAzureFileStorage:
///     type: azure:datafactory:LinkedServiceAzureFileStorage
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       connectionString: ${example.primaryConnectionString}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:storage:getAccount
///       arguments:
///         name: storageaccountname
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// Data Factory Linked Service's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/linkedServiceAzureFileStorage:LinkedServiceAzureFileStorage example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/linkedservices/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod linked_service_azure_file_storage {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceAzureFileStorageArgs {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to Azure File Storage Linked Service:
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The connection string.
        #[builder(into)]
        pub connection_string: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Linked Service.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the file share.
        #[builder(into, default)]
        pub file_share: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Host name of the server.
        #[builder(into, default)]
        pub host: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub integration_runtime_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `key_vault_password` block as defined below. Use this argument to store Azure File Storage password in an existing Key Vault. It needs an existing Key Vault Data Factory Linked Service.
        #[builder(into, default)]
        pub key_vault_password: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::LinkedServiceAzureFileStorageKeyVaultPassword,
            >,
        >,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of parameters to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The password to log in the server.
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user ID to log in the server.
        #[builder(into, default)]
        pub user_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceAzureFileStorageResult {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to Azure File Storage Linked Service:
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The connection string.
        pub connection_string: pulumi_gestalt_rust::Output<String>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Linked Service.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the file share.
        pub file_share: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Host name of the server.
        pub host: pulumi_gestalt_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        pub integration_runtime_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `key_vault_password` block as defined below. Use this argument to store Azure File Storage password in an existing Key Vault. It needs an existing Key Vault Data Factory Linked Service.
        pub key_vault_password: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureFileStorageKeyVaultPassword,
            >,
        >,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Linked Service.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The password to log in the server.
        pub password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The user ID to log in the server.
        pub user_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LinkedServiceAzureFileStorageArgs,
    ) -> LinkedServiceAzureFileStorageResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let additional_properties_binding_1 = args
            .additional_properties
            .get_output(context);
        let additional_properties_binding = additional_properties_binding_1.get_inner();
        let annotations_binding_1 = args.annotations.get_output(context);
        let annotations_binding = annotations_binding_1.get_inner();
        let connection_string_binding_1 = args.connection_string.get_output(context);
        let connection_string_binding = connection_string_binding_1.get_inner();
        let data_factory_id_binding_1 = args.data_factory_id.get_output(context);
        let data_factory_id_binding = data_factory_id_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let file_share_binding_1 = args.file_share.get_output(context);
        let file_share_binding = file_share_binding_1.get_inner();
        let host_binding_1 = args.host.get_output(context);
        let host_binding = host_binding_1.get_inner();
        let integration_runtime_name_binding_1 = args
            .integration_runtime_name
            .get_output(context);
        let integration_runtime_name_binding = integration_runtime_name_binding_1
            .get_inner();
        let key_vault_password_binding_1 = args.key_vault_password.get_output(context);
        let key_vault_password_binding = key_vault_password_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let parameters_binding_1 = args.parameters.get_output(context);
        let parameters_binding = parameters_binding_1.get_inner();
        let password_binding_1 = args.password.get_output(context);
        let password_binding = password_binding_1.get_inner();
        let user_id_binding_1 = args.user_id.get_output(context);
        let user_id_binding = user_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/linkedServiceAzureFileStorage:LinkedServiceAzureFileStorage"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "connectionString".into(),
                    value: &connection_string_binding,
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
                    name: "fileShare".into(),
                    value: &file_share_binding,
                },
                register_interface::ObjectField {
                    name: "host".into(),
                    value: &host_binding,
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "userId".into(),
                    value: &user_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LinkedServiceAzureFileStorageResult {
            additional_properties: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("additionalProperties"),
            ),
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionString"),
            ),
            data_factory_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataFactoryId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            file_share: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fileShare"),
            ),
            host: pulumi_gestalt_rust::__private::into_domain(o.extract_field("host")),
            integration_runtime_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("integrationRuntimeName"),
            ),
            key_vault_password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultPassword"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            user_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userId"),
            ),
        }
    }
}
