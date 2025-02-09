/// Manages a Linked Service (connection) between a SQL Server and Azure Data Factory.
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
///     let exampleFactory = factory::create(
///         "exampleFactory",
///         FactoryArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLinkedServiceSqlServer = linked_service_sql_server::create(
///         "exampleLinkedServiceSqlServer",
///         LinkedServiceSqlServerArgs::builder()
///             .connection_string(
///                 "Integrated Security=False;Data Source=test;Initial Catalog=test;User ID=test;Password=test",
///             )
///             .data_factory_id("${exampleFactory.id}")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### With Password In Key Vault
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: standard
///   exampleFactory:
///     type: azure:datafactory:Factory
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleLinkedServiceKeyVault:
///     type: azure:datafactory:LinkedServiceKeyVault
///     name: example
///     properties:
///       name: kvlink
///       dataFactoryId: ${exampleFactory.id}
///       keyVaultId: ${exampleKeyVault.id}
///   exampleLinkedServiceSqlServer:
///     type: azure:datafactory:LinkedServiceSqlServer
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       connectionString: Integrated Security=False;Data Source=test;Initial Catalog=test;User ID=test;
///       keyVaultPassword:
///         linkedServiceName: ${exampleLinkedServiceKeyVault.name}
///         secretName: secret
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Data Factory SQL Server Linked Service's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/linkedServiceSqlServer:LinkedServiceSqlServer example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/linkedservices/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod linked_service_sql_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceSqlServerArgs {
        /// A map of additional properties to associate with the Data Factory Linked Service SQL Server.
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service SQL Server.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The connection string in which to authenticate with the SQL Server. Exactly one of either `connection_string` or `key_vault_connection_string` is required.
        #[builder(into, default)]
        pub connection_string: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Linked Service SQL Server.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service SQL Server.
        #[builder(into, default)]
        pub integration_runtime_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `key_vault_connection_string` block as defined below. Use this argument to store SQL Server connection string in an existing Key Vault. It needs an existing Key Vault Data Factory Linked Service. Exactly one of either `connection_string` or `key_vault_connection_string` is required.
        #[builder(into, default)]
        pub key_vault_connection_string: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::LinkedServiceSqlServerKeyVaultConnectionString,
            >,
        >,
        /// A `key_vault_password` block as defined below. Use this argument to store SQL Server password in an existing Key Vault. It needs an existing Key Vault Data Factory Linked Service.
        #[builder(into, default)]
        pub key_vault_password: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::LinkedServiceSqlServerKeyVaultPassword,
            >,
        >,
        /// Specifies the name of the Data Factory Linked Service SQL Server. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of parameters to associate with the Data Factory Linked Service SQL Server.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The on-premises Windows authentication user name.
        #[builder(into, default)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceSqlServerResult {
        /// A map of additional properties to associate with the Data Factory Linked Service SQL Server.
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service SQL Server.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The connection string in which to authenticate with the SQL Server. Exactly one of either `connection_string` or `key_vault_connection_string` is required.
        pub connection_string: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Linked Service SQL Server.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service SQL Server.
        pub integration_runtime_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `key_vault_connection_string` block as defined below. Use this argument to store SQL Server connection string in an existing Key Vault. It needs an existing Key Vault Data Factory Linked Service. Exactly one of either `connection_string` or `key_vault_connection_string` is required.
        pub key_vault_connection_string: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceSqlServerKeyVaultConnectionString,
            >,
        >,
        /// A `key_vault_password` block as defined below. Use this argument to store SQL Server password in an existing Key Vault. It needs an existing Key Vault Data Factory Linked Service.
        pub key_vault_password: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceSqlServerKeyVaultPassword,
            >,
        >,
        /// Specifies the name of the Data Factory Linked Service SQL Server. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Linked Service SQL Server.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The on-premises Windows authentication user name.
        pub user_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LinkedServiceSqlServerArgs,
    ) -> LinkedServiceSqlServerResult {
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
        let integration_runtime_name_binding_1 = args
            .integration_runtime_name
            .get_output(context);
        let integration_runtime_name_binding = integration_runtime_name_binding_1
            .get_inner();
        let key_vault_connection_string_binding_1 = args
            .key_vault_connection_string
            .get_output(context);
        let key_vault_connection_string_binding = key_vault_connection_string_binding_1
            .get_inner();
        let key_vault_password_binding_1 = args.key_vault_password.get_output(context);
        let key_vault_password_binding = key_vault_password_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let parameters_binding_1 = args.parameters.get_output(context);
        let parameters_binding = parameters_binding_1.get_inner();
        let user_name_binding_1 = args.user_name.get_output(context);
        let user_name_binding = user_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/linkedServiceSqlServer:LinkedServiceSqlServer"
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
                    name: "integrationRuntimeName".into(),
                    value: &integration_runtime_name_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultConnectionString".into(),
                    value: &key_vault_connection_string_binding,
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
                    name: "userName".into(),
                    value: &user_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LinkedServiceSqlServerResult {
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
            integration_runtime_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("integrationRuntimeName"),
            ),
            key_vault_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultConnectionString"),
            ),
            key_vault_password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultPassword"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            user_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userName"),
            ),
        }
    }
}
