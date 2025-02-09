/// Manages a Data Flow inside an Azure Data Factory.
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleFactory:
///     type: azure:datafactory:Factory
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleLinkedCustomService:
///     type: azure:datafactory:LinkedCustomService
///     name: example
///     properties:
///       name: linked_service
///       dataFactoryId: ${exampleFactory.id}
///       type: AzureBlobStorage
///       typePropertiesJson: |
///         {
///           "connectionString": "${exampleAccount.primaryConnectionString}"
///         }
///   example1:
///     type: azure:datafactory:DatasetJson
///     properties:
///       name: dataset1
///       dataFactoryId: ${exampleFactory.id}
///       linkedServiceName: ${exampleLinkedCustomService.name}
///       azureBlobStorageLocation:
///         container: container
///         path: foo/bar/
///         filename: foo.txt
///       encoding: UTF-8
///   example2:
///     type: azure:datafactory:DatasetJson
///     properties:
///       name: dataset2
///       dataFactoryId: ${exampleFactory.id}
///       linkedServiceName: ${exampleLinkedCustomService.name}
///       azureBlobStorageLocation:
///         container: container
///         path: foo/bar/
///         filename: bar.txt
///       encoding: UTF-8
///   exampleDataFlow:
///     type: azure:datafactory:DataFlow
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       sources:
///         - name: source1
///           flowlet:
///             name: ${example1FlowletDataFlow.name}
///             parameters:
///               Key1: value1
///           dataset:
///             name: ${example1.name}
///       sinks:
///         - name: sink1
///           flowlet:
///             name: ${example2FlowletDataFlow.name}
///             parameters:
///               Key1: value1
///           dataset:
///             name: ${example2.name}
///       script: "source(\n  allowSchemaDrift: true, \n  validateSchema: false, \n  limit: 100, \n  ignoreNoFilesFound: false, \n  documentForm: 'documentPerLine') ~> source1 \nsource1 sink(\n  allowSchemaDrift: true, \n  validateSchema: false, \n  skipDuplicateMapInputs: true, \n  skipDuplicateMapOutputs: true) ~> sink1\n"
///   example1FlowletDataFlow:
///     type: azure:datafactory:FlowletDataFlow
///     name: example1
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       sources:
///         - name: source1
///           linkedService:
///             name: ${exampleLinkedCustomService.name}
///       sinks:
///         - name: sink1
///           linkedService:
///             name: ${exampleLinkedCustomService.name}
///       script: "source(\n  allowSchemaDrift: true, \n  validateSchema: false, \n  limit: 100, \n  ignoreNoFilesFound: false, \n  documentForm: 'documentPerLine') ~> source1 \nsource1 sink(\n  allowSchemaDrift: true, \n  validateSchema: false, \n  skipDuplicateMapInputs: true, \n  skipDuplicateMapOutputs: true) ~> sink1\n"
///   example2FlowletDataFlow:
///     type: azure:datafactory:FlowletDataFlow
///     name: example2
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       sources:
///         - name: source1
///           linkedService:
///             name: ${exampleLinkedCustomService.name}
///       sinks:
///         - name: sink1
///           linkedService:
///             name: ${exampleLinkedCustomService.name}
///       script: "source(\n  allowSchemaDrift: true, \n  validateSchema: false, \n  limit: 100, \n  ignoreNoFilesFound: false, \n  documentForm: 'documentPerLine') ~> source1 \nsource1 sink(\n  allowSchemaDrift: true, \n  validateSchema: false, \n  skipDuplicateMapInputs: true, \n  skipDuplicateMapOutputs: true) ~> sink1\n"
/// ```
///
/// ## Import
///
/// Data Factory Data Flow can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/dataFlow:DataFlow example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/dataflows/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_flow {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataFlowArgs {
        /// List of tags that can be used for describing the Data Factory Data Flow.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Data Flow with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Data Flow.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The folder that this Data Flow is in. If not specified, the Data Flow will appear at the root level.
        #[builder(into, default)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Data Factory Data Flow. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The script for the Data Factory Data Flow.
        #[builder(into, default)]
        pub script: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The script lines for the Data Factory Data Flow.
        #[builder(into, default)]
        pub script_lines: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// One or more `sink` blocks as defined below.
        #[builder(into)]
        pub sinks: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::datafactory::DataFlowSink>,
        >,
        /// One or more `source` blocks as defined below.
        #[builder(into)]
        pub sources: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::datafactory::DataFlowSource>,
        >,
        /// One or more `transformation` blocks as defined below.
        #[builder(into, default)]
        pub transformations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::datafactory::DataFlowTransformation>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataFlowResult {
        /// List of tags that can be used for describing the Data Factory Data Flow.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Data Flow with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Data Flow.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The folder that this Data Flow is in. If not specified, the Data Flow will appear at the root level.
        pub folder: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Data Factory Data Flow. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The script for the Data Factory Data Flow.
        pub script: pulumi_gestalt_rust::Output<Option<String>>,
        /// The script lines for the Data Factory Data Flow.
        pub script_lines: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// One or more `sink` blocks as defined below.
        pub sinks: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datafactory::DataFlowSink>,
        >,
        /// One or more `source` blocks as defined below.
        pub sources: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datafactory::DataFlowSource>,
        >,
        /// One or more `transformation` blocks as defined below.
        pub transformations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::datafactory::DataFlowTransformation>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DataFlowArgs,
    ) -> DataFlowResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let annotations_binding_1 = args.annotations.get_output(context);
        let annotations_binding = annotations_binding_1.get_inner();
        let data_factory_id_binding_1 = args.data_factory_id.get_output(context);
        let data_factory_id_binding = data_factory_id_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let folder_binding_1 = args.folder.get_output(context);
        let folder_binding = folder_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let script_binding_1 = args.script.get_output(context);
        let script_binding = script_binding_1.get_inner();
        let script_lines_binding_1 = args.script_lines.get_output(context);
        let script_lines_binding = script_lines_binding_1.get_inner();
        let sinks_binding_1 = args.sinks.get_output(context);
        let sinks_binding = sinks_binding_1.get_inner();
        let sources_binding_1 = args.sources.get_output(context);
        let sources_binding = sources_binding_1.get_inner();
        let transformations_binding_1 = args.transformations.get_output(context);
        let transformations_binding = transformations_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/dataFlow:DataFlow".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "script".into(),
                    value: &script_binding,
                },
                register_interface::ObjectField {
                    name: "scriptLines".into(),
                    value: &script_lines_binding,
                },
                register_interface::ObjectField {
                    name: "sinks".into(),
                    value: &sinks_binding,
                },
                register_interface::ObjectField {
                    name: "sources".into(),
                    value: &sources_binding,
                },
                register_interface::ObjectField {
                    name: "transformations".into(),
                    value: &transformations_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DataFlowResult {
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
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            script: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("script"),
            ),
            script_lines: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scriptLines"),
            ),
            sinks: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sinks")),
            sources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sources"),
            ),
            transformations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transformations"),
            ),
        }
    }
}
