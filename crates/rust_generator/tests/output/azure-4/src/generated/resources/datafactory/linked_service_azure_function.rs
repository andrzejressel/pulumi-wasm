/// Manages a Linked Service (connection) between an Azure Function and Azure Data Factory.
///
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
///   exampleLinkedServiceAzureFunction:
///     type: azure:datafactory:LinkedServiceAzureFunction
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       url: https://${example.defaultHostname}
///       key: foo
/// variables:
///   example:
///     fn::invoke:
///       function: azure:appservice:getFunctionApp
///       arguments:
///         name: test-azure-functions
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// Data Factory Linked Service's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/linkedServiceAzureFunction:LinkedServiceAzureFunction example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/linkedservices/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod linked_service_azure_function {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceAzureFunctionArgs {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to Azure Function Linked Service:
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
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub integration_runtime_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The system key of the Azure Function. Exactly one of either `key` or `key_vault_key` is required
        #[builder(into, default)]
        pub key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `key_vault_key` block as defined below. Use this Argument to store the system key of the Azure Function in an existing Key Vault. It needs an existing Key Vault Data Factory Linked Service. Exactly one of either `key` or `key_vault_key` is required.
        #[builder(into, default)]
        pub key_vault_key: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::LinkedServiceAzureFunctionKeyVaultKey,
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
        /// The url of the Azure Function.
        #[builder(into)]
        pub url: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceAzureFunctionResult {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to Azure Function Linked Service:
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Linked Service.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        pub integration_runtime_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The system key of the Azure Function. Exactly one of either `key` or `key_vault_key` is required
        pub key: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `key_vault_key` block as defined below. Use this Argument to store the system key of the Azure Function in an existing Key Vault. It needs an existing Key Vault Data Factory Linked Service. Exactly one of either `key` or `key_vault_key` is required.
        pub key_vault_key: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceAzureFunctionKeyVaultKey,
            >,
        >,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Linked Service.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The url of the Azure Function.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LinkedServiceAzureFunctionArgs,
    ) -> LinkedServiceAzureFunctionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_properties_binding = args
            .additional_properties
            .get_output(context);
        let annotations_binding = args.annotations.get_output(context);
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let integration_runtime_name_binding = args
            .integration_runtime_name
            .get_output(context);
        let key_binding = args.key.get_output(context);
        let key_vault_key_binding = args.key_vault_key.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let url_binding = args.url.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/linkedServiceAzureFunction:LinkedServiceAzureFunction"
                .into(),
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
                    name: "dataFactoryId".into(),
                    value: data_factory_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationRuntimeName".into(),
                    value: integration_runtime_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "key".into(),
                    value: key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultKey".into(),
                    value: key_vault_key_binding.get_id(),
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
                    name: "url".into(),
                    value: url_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LinkedServiceAzureFunctionResult {
            additional_properties: o.get_field("additionalProperties"),
            annotations: o.get_field("annotations"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            integration_runtime_name: o.get_field("integrationRuntimeName"),
            key: o.get_field("key"),
            key_vault_key: o.get_field("keyVaultKey"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            url: o.get_field("url"),
        }
    }
}
