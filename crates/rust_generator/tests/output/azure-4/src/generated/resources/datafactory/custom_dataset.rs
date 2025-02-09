/// Manages a Dataset inside an Azure Data Factory. This is a generic resource that supports all different Dataset Types.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleFactory:
///     type: azure:datafactory:Factory
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       identity:
///         type: SystemAssigned
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
///   exampleLinkedCustomService:
///     type: azure:datafactory:LinkedCustomService
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       type: AzureBlobStorage
///       typePropertiesJson: |
///         {
///           "connectionString":"${exampleAccount.primaryConnectionString}"
///         }
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: content
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: private
///   exampleCustomDataset:
///     type: azure:datafactory:CustomDataset
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       type: Json
///       linkedService:
///         name: ${exampleLinkedCustomService.name}
///         parameters:
///           key1: value1
///       typePropertiesJson: |
///         {
///           "location": {
///             "container":"${exampleContainer.name}",
///             "fileName":"foo.txt",
///             "folderPath": "foo/bar/",
///             "type":"AzureBlobStorageLocation"
///           },
///           "encodingName":"UTF-8"
///         }
///       description: test description
///       annotations:
///         - test1
///         - test2
///         - test3
///       folder: testFolder
///       parameters:
///         foo: test1
///         Bar: Test2
///       additionalProperties:
///         foo: test1
///         bar: test2
///       schemaJson: |
///         {
///           "type": "object",
///           "properties": {
///             "name": {
///               "type": "object",
///               "properties": {
///                 "firstName": {
///                   "type": "string"
///                 },
///                 "lastName": {
///                   "type": "string"
///                 }
///               }
///             },
///             "age": {
///               "type": "integer"
///             }
///           }
///         }
/// ```
///
/// ## Import
///
/// Data Factory Datasets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/customDataset:CustomDataset example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/datasets/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod custom_dataset {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomDatasetArgs {
        /// A map of additional properties to associate with the Data Factory Dataset.
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Dataset with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Dataset.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        #[builder(into, default)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `linked_service` block as defined below.
        #[builder(into)]
        pub linked_service: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::datafactory::CustomDatasetLinkedService,
        >,
        /// Specifies the name of the Data Factory Dataset. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of parameters to associate with the Data Factory Dataset.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A JSON object that contains the schema of the Data Factory Dataset.
        #[builder(into, default)]
        pub schema_json: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of dataset that will be associated with Data Factory. Changing this forces a new resource to be created.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A JSON object that contains the properties of the Data Factory Dataset.
        #[builder(into)]
        pub type_properties_json: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CustomDatasetResult {
        /// A map of additional properties to associate with the Data Factory Dataset.
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Dataset with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Dataset.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        pub folder: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `linked_service` block as defined below.
        pub linked_service: pulumi_gestalt_rust::Output<
            super::super::types::datafactory::CustomDatasetLinkedService,
        >,
        /// Specifies the name of the Data Factory Dataset. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Dataset.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A JSON object that contains the schema of the Data Factory Dataset.
        pub schema_json: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of dataset that will be associated with Data Factory. Changing this forces a new resource to be created.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// A JSON object that contains the properties of the Data Factory Dataset.
        pub type_properties_json: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CustomDatasetArgs,
    ) -> CustomDatasetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
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
        let folder_binding_1 = args.folder.get_output(context);
        let folder_binding = folder_binding_1.get_inner();
        let linked_service_binding_1 = args.linked_service.get_output(context);
        let linked_service_binding = linked_service_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let parameters_binding_1 = args.parameters.get_output(context);
        let parameters_binding = parameters_binding_1.get_inner();
        let schema_json_binding_1 = args.schema_json.get_output(context);
        let schema_json_binding = schema_json_binding_1.get_inner();
        let type__binding_1 = args.type_.get_output(context);
        let type__binding = type__binding_1.get_inner();
        let type_properties_json_binding_1 = args
            .type_properties_json
            .get_output(context);
        let type_properties_json_binding = type_properties_json_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/customDataset:CustomDataset".into(),
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
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "folder".into(),
                    value: &folder_binding,
                },
                register_interface::ObjectField {
                    name: "linkedService".into(),
                    value: &linked_service_binding,
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
                    name: "schemaJson".into(),
                    value: &schema_json_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        CustomDatasetResult {
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
            folder: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("folder"),
            ),
            linked_service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("linkedService"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            schema_json: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schemaJson"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            type_properties_json: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("typePropertiesJson"),
            ),
        }
    }
}
