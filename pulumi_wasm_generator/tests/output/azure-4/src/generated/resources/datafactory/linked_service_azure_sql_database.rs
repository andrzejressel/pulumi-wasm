/// Manages a Linked Service (connection) between Azure SQL Database and Azure Data Factory.
///
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
///     let exampleLinkedServiceAzureSqlDatabase = linked_service_azure_sql_database::create(
///         "exampleLinkedServiceAzureSqlDatabase",
///         LinkedServiceAzureSqlDatabaseArgs::builder()
///             .connection_string(
///                 "data source=serverhostname;initial catalog=master;user id=testUser;Password=test;integrated security=False;encrypt=True;connection timeout=30",
///             )
///             .data_factory_id("${exampleFactory.id}")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory Azure SQL Database Linked Service's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/linkedServiceAzureSqlDatabase:LinkedServiceAzureSqlDatabase example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/linkedservices/example
/// ```
///
pub mod linked_service_azure_sql_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceAzureSqlDatabaseArgs {
        /// A map of additional properties to associate with the Data Factory Linked Service Azure SQL Database.
        #[builder(into, default)]
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service Azure SQL Database.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The connection string in which to authenticate with Azure SQL Database. Exactly one of either `connection_string` or `key_vault_connection_string` is required.
        #[builder(into, default)]
        pub connection_string: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of a User-assigned Managed Identity. Use this argument to authenticate against the linked resource using a User-assigned Managed Identity.
        #[builder(into, default)]
        pub credential_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Linked Service Azure SQL Database.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service Azure SQL Database.
        #[builder(into, default)]
        pub integration_runtime_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `key_vault_connection_string` block as defined below. Use this argument to store Azure SQL Database connection string in an existing Key Vault. It needs an existing Key Vault Data Factory Linked Service. Exactly one of either `connection_string` or `key_vault_connection_string` is required.
        #[builder(into, default)]
        pub key_vault_connection_string: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureSqlDatabaseKeyVaultConnectionString,
            >,
        >,
        /// A `key_vault_password` block as defined below. Use this argument to store SQL Server password in an existing Key Vault. It needs an existing Key Vault Data Factory Linked Service.
        #[builder(into, default)]
        pub key_vault_password: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureSqlDatabaseKeyVaultPassword,
            >,
        >,
        /// Specifies the name of the Data Factory Linked Service Azure SQL Database. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of parameters to associate with the Data Factory Linked Service Azure SQL Database.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The service principal id in which to authenticate against the Azure SQL Database. Required if `service_principal_key` is set.
        #[builder(into, default)]
        pub service_principal_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The service principal key in which to authenticate against the Azure SQL Database. Required if `service_principal_id` is set.
        #[builder(into, default)]
        pub service_principal_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The tenant id or name in which to authenticate against the Azure SQL Database.
        #[builder(into, default)]
        pub tenant_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to use the Data Factory's managed identity to authenticate against the Azure SQL Database. Incompatible with `service_principal_id` and `service_principal_key`
        #[builder(into, default)]
        pub use_managed_identity: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceAzureSqlDatabaseResult {
        /// A map of additional properties to associate with the Data Factory Linked Service Azure SQL Database.
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service Azure SQL Database.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The connection string in which to authenticate with Azure SQL Database. Exactly one of either `connection_string` or `key_vault_connection_string` is required.
        pub connection_string: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of a User-assigned Managed Identity. Use this argument to authenticate against the linked resource using a User-assigned Managed Identity.
        pub credential_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Linked Service Azure SQL Database.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service Azure SQL Database.
        pub integration_runtime_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `key_vault_connection_string` block as defined below. Use this argument to store Azure SQL Database connection string in an existing Key Vault. It needs an existing Key Vault Data Factory Linked Service. Exactly one of either `connection_string` or `key_vault_connection_string` is required.
        pub key_vault_connection_string: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureSqlDatabaseKeyVaultConnectionString,
            >,
        >,
        /// A `key_vault_password` block as defined below. Use this argument to store SQL Server password in an existing Key Vault. It needs an existing Key Vault Data Factory Linked Service.
        pub key_vault_password: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureSqlDatabaseKeyVaultPassword,
            >,
        >,
        /// Specifies the name of the Data Factory Linked Service Azure SQL Database. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Linked Service Azure SQL Database.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The service principal id in which to authenticate against the Azure SQL Database. Required if `service_principal_key` is set.
        pub service_principal_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The service principal key in which to authenticate against the Azure SQL Database. Required if `service_principal_id` is set.
        pub service_principal_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The tenant id or name in which to authenticate against the Azure SQL Database.
        pub tenant_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to use the Data Factory's managed identity to authenticate against the Azure SQL Database. Incompatible with `service_principal_id` and `service_principal_key`
        pub use_managed_identity: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: LinkedServiceAzureSqlDatabaseArgs,
    ) -> LinkedServiceAzureSqlDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_properties_binding = args.additional_properties.get_inner();
        let annotations_binding = args.annotations.get_inner();
        let connection_string_binding = args.connection_string.get_inner();
        let credential_name_binding = args.credential_name.get_inner();
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let description_binding = args.description.get_inner();
        let integration_runtime_name_binding = args.integration_runtime_name.get_inner();
        let key_vault_connection_string_binding = args
            .key_vault_connection_string
            .get_inner();
        let key_vault_password_binding = args.key_vault_password.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let service_principal_id_binding = args.service_principal_id.get_inner();
        let service_principal_key_binding = args.service_principal_key.get_inner();
        let tenant_id_binding = args.tenant_id.get_inner();
        let use_managed_identity_binding = args.use_managed_identity.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/linkedServiceAzureSqlDatabase:LinkedServiceAzureSqlDatabase"
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
                    name: "credentialName".into(),
                    value: &credential_name_binding,
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
                    name: "servicePrincipalId".into(),
                    value: &service_principal_id_binding,
                },
                register_interface::ObjectField {
                    name: "servicePrincipalKey".into(),
                    value: &service_principal_key_binding,
                },
                register_interface::ObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding,
                },
                register_interface::ObjectField {
                    name: "useManagedIdentity".into(),
                    value: &use_managed_identity_binding,
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
                    name: "connectionString".into(),
                },
                register_interface::ResultField {
                    name: "credentialName".into(),
                },
                register_interface::ResultField {
                    name: "dataFactoryId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "integrationRuntimeName".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultPassword".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "servicePrincipalId".into(),
                },
                register_interface::ResultField {
                    name: "servicePrincipalKey".into(),
                },
                register_interface::ResultField {
                    name: "tenantId".into(),
                },
                register_interface::ResultField {
                    name: "useManagedIdentity".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LinkedServiceAzureSqlDatabaseResult {
            additional_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalProperties").unwrap(),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionString").unwrap(),
            ),
            credential_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("credentialName").unwrap(),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFactoryId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            integration_runtime_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationRuntimeName").unwrap(),
            ),
            key_vault_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultConnectionString").unwrap(),
            ),
            key_vault_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultPassword").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            service_principal_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicePrincipalId").unwrap(),
            ),
            service_principal_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicePrincipalKey").unwrap(),
            ),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantId").unwrap(),
            ),
            use_managed_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("useManagedIdentity").unwrap(),
            ),
        }
    }
}
