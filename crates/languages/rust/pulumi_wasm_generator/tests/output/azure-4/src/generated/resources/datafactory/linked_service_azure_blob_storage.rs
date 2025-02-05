/// Manages a Linked Service (connection) between an Azure Blob Storage Account and Azure Data Factory.
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
///   exampleLinkedServiceAzureBlobStorage:
///     type: azure:datafactory:LinkedServiceAzureBlobStorage
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
///
/// ## Import
///
/// Data Factory Linked Service's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/linkedServiceAzureBlobStorage:LinkedServiceAzureBlobStorage example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/linkedservices/example
/// ```
///
pub mod linked_service_azure_blob_storage {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceAzureBlobStorageArgs {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to Azure Blob Storage Linked Service:
        #[builder(into, default)]
        pub additional_properties: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The connection string. Conflicts with `connection_string_insecure`, `sas_uri` and `service_endpoint`.
        #[builder(into, default)]
        pub connection_string: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The connection string sent insecurely. Conflicts with `connection_string`, `sas_uri` and `service_endpoint`.
        ///
        /// > **Note:** `connection_string` uses the Azure [SecureString](https://learn.microsoft.com/en-us/dotnet/api/microsoft.azure.management.datafactory.models.securestring) to encrypt the contents within the REST payload sent to Azure whilst the `connection_string_insecure` is sent as a regular string. Both properties are still sent using SSL/HTTPS. At this time the portal will not decrypt Secure Strings so the `connection_string` property in the portal will show as `******` whilst `connection_string_insecure` will be viewable in the portal.
        #[builder(into, default)]
        pub connection_string_insecure: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The description for the Data Factory Linked Service.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub integration_runtime_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `key_vault_sas_token` block as defined below. Use this argument to store SAS Token in an existing Key Vault. It needs an existing Key Vault Data Factory Linked Service. A `sas_uri` is required.
        #[builder(into, default)]
        pub key_vault_sas_token: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::LinkedServiceAzureBlobStorageKeyVaultSasToken,
            >,
        >,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of parameters to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The SAS URI. Conflicts with `connection_string_insecure`, `connection_string` and `service_endpoint`.
        #[builder(into, default)]
        pub sas_uri: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub service_endpoint: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub service_principal_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub service_principal_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub service_principal_linked_key_vault_key: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::LinkedServiceAzureBlobStorageServicePrincipalLinkedKeyVaultKey,
            >,
        >,
        #[builder(into, default)]
        pub storage_kind: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub tenant_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub use_managed_identity: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceAzureBlobStorageResult {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to Azure Blob Storage Linked Service:
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The connection string. Conflicts with `connection_string_insecure`, `sas_uri` and `service_endpoint`.
        pub connection_string: pulumi_wasm_rust::Output<Option<String>>,
        /// The connection string sent insecurely. Conflicts with `connection_string`, `sas_uri` and `service_endpoint`.
        ///
        /// > **Note:** `connection_string` uses the Azure [SecureString](https://learn.microsoft.com/en-us/dotnet/api/microsoft.azure.management.datafactory.models.securestring) to encrypt the contents within the REST payload sent to Azure whilst the `connection_string_insecure` is sent as a regular string. Both properties are still sent using SSL/HTTPS. At this time the portal will not decrypt Secure Strings so the `connection_string` property in the portal will show as `******` whilst `connection_string_insecure` will be viewable in the portal.
        pub connection_string_insecure: pulumi_wasm_rust::Output<Option<String>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Linked Service.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        pub integration_runtime_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `key_vault_sas_token` block as defined below. Use this argument to store SAS Token in an existing Key Vault. It needs an existing Key Vault Data Factory Linked Service. A `sas_uri` is required.
        pub key_vault_sas_token: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureBlobStorageKeyVaultSasToken,
            >,
        >,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Linked Service.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The SAS URI. Conflicts with `connection_string_insecure`, `connection_string` and `service_endpoint`.
        pub sas_uri: pulumi_wasm_rust::Output<Option<String>>,
        pub service_endpoint: pulumi_wasm_rust::Output<Option<String>>,
        pub service_principal_id: pulumi_wasm_rust::Output<Option<String>>,
        pub service_principal_key: pulumi_wasm_rust::Output<Option<String>>,
        pub service_principal_linked_key_vault_key: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureBlobStorageServicePrincipalLinkedKeyVaultKey,
            >,
        >,
        pub storage_kind: pulumi_wasm_rust::Output<Option<String>>,
        pub tenant_id: pulumi_wasm_rust::Output<Option<String>>,
        pub use_managed_identity: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LinkedServiceAzureBlobStorageArgs,
    ) -> LinkedServiceAzureBlobStorageResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_properties_binding = args
            .additional_properties
            .get_output(context)
            .get_inner();
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let connection_string_binding = args
            .connection_string
            .get_output(context)
            .get_inner();
        let connection_string_insecure_binding = args
            .connection_string_insecure
            .get_output(context)
            .get_inner();
        let data_factory_id_binding = args
            .data_factory_id
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let integration_runtime_name_binding = args
            .integration_runtime_name
            .get_output(context)
            .get_inner();
        let key_vault_sas_token_binding = args
            .key_vault_sas_token
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let sas_uri_binding = args.sas_uri.get_output(context).get_inner();
        let service_endpoint_binding = args
            .service_endpoint
            .get_output(context)
            .get_inner();
        let service_principal_id_binding = args
            .service_principal_id
            .get_output(context)
            .get_inner();
        let service_principal_key_binding = args
            .service_principal_key
            .get_output(context)
            .get_inner();
        let service_principal_linked_key_vault_key_binding = args
            .service_principal_linked_key_vault_key
            .get_output(context)
            .get_inner();
        let storage_kind_binding = args.storage_kind.get_output(context).get_inner();
        let tenant_id_binding = args.tenant_id.get_output(context).get_inner();
        let use_managed_identity_binding = args
            .use_managed_identity
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/linkedServiceAzureBlobStorage:LinkedServiceAzureBlobStorage"
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
                    name: "connectionStringInsecure".into(),
                    value: &connection_string_insecure_binding,
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
                    name: "keyVaultSasToken".into(),
                    value: &key_vault_sas_token_binding,
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
                    name: "sasUri".into(),
                    value: &sas_uri_binding,
                },
                register_interface::ObjectField {
                    name: "serviceEndpoint".into(),
                    value: &service_endpoint_binding,
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
                    name: "servicePrincipalLinkedKeyVaultKey".into(),
                    value: &service_principal_linked_key_vault_key_binding,
                },
                register_interface::ObjectField {
                    name: "storageKind".into(),
                    value: &storage_kind_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        LinkedServiceAzureBlobStorageResult {
            additional_properties: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("additionalProperties"),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            connection_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionString"),
            ),
            connection_string_insecure: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionStringInsecure"),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataFactoryId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            integration_runtime_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("integrationRuntimeName"),
            ),
            key_vault_sas_token: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyVaultSasToken"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            sas_uri: pulumi_wasm_rust::__private::into_domain(o.extract_field("sasUri")),
            service_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceEndpoint"),
            ),
            service_principal_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("servicePrincipalId"),
            ),
            service_principal_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("servicePrincipalKey"),
            ),
            service_principal_linked_key_vault_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("servicePrincipalLinkedKeyVaultKey"),
            ),
            storage_kind: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageKind"),
            ),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tenantId"),
            ),
            use_managed_identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("useManagedIdentity"),
            ),
        }
    }
}
