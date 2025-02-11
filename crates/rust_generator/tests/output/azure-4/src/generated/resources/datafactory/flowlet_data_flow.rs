/// Manages a Flowlet Data Flow inside an Azure Data Factory.
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
///     let example1 = dataset_json::create(
///         "example1",
///         DatasetJsonArgs::builder()
///             .azure_blob_storage_location(
///                 DatasetJsonAzureBlobStorageLocation::builder()
///                     .container("container")
///                     .filename("foo.txt")
///                     .path("foo/bar/")
///                     .build_struct(),
///             )
///             .data_factory_id("${exampleFactory.id}")
///             .encoding("UTF-8")
///             .linked_service_name("${exampleLinkedCustomService.name}")
///             .name("dataset1")
///             .build_struct(),
///     );
///     let example1FlowletDataFlow = flowlet_data_flow::create(
///         "example1FlowletDataFlow",
///         FlowletDataFlowArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .name("example")
///             .script(
///                 "source(\n  allowSchemaDrift: true, \n  validateSchema: false, \n  limit: 100, \n  ignoreNoFilesFound: false, \n  documentForm: 'documentPerLine') ~> source1 \nsource1 sink(\n  allowSchemaDrift: true, \n  validateSchema: false, \n  skipDuplicateMapInputs: true, \n  skipDuplicateMapOutputs: true) ~> sink1\n",
///             )
///             .sinks(
///                 vec![
///                     FlowletDataFlowSink::builder()
///                     .linkedService(FlowletDataFlowSinkLinkedService::builder()
///                     .name("${exampleLinkedCustomService.name}").build_struct())
///                     .name("sink1").build_struct(),
///                 ],
///             )
///             .sources(
///                 vec![
///                     FlowletDataFlowSource::builder()
///                     .linkedService(FlowletDataFlowSourceLinkedService::builder()
///                     .name("${exampleLinkedCustomService.name}").build_struct())
///                     .name("source1").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let example2 = dataset_json::create(
///         "example2",
///         DatasetJsonArgs::builder()
///             .azure_blob_storage_location(
///                 DatasetJsonAzureBlobStorageLocation::builder()
///                     .container("container")
///                     .filename("bar.txt")
///                     .path("foo/bar/")
///                     .build_struct(),
///             )
///             .data_factory_id("${exampleFactory.id}")
///             .encoding("UTF-8")
///             .linked_service_name("${exampleLinkedCustomService.name}")
///             .name("dataset2")
///             .build_struct(),
///     );
///     let example2FlowletDataFlow = flowlet_data_flow::create(
///         "example2FlowletDataFlow",
///         FlowletDataFlowArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .name("example")
///             .script(
///                 "source(\n  allowSchemaDrift: true, \n  validateSchema: false, \n  limit: 100, \n  ignoreNoFilesFound: false, \n  documentForm: 'documentPerLine') ~> source1 \nsource1 sink(\n  allowSchemaDrift: true, \n  validateSchema: false, \n  skipDuplicateMapInputs: true, \n  skipDuplicateMapOutputs: true) ~> sink1\n",
///             )
///             .sinks(
///                 vec![
///                     FlowletDataFlowSink::builder()
///                     .linkedService(FlowletDataFlowSinkLinkedService::builder()
///                     .name("${exampleLinkedCustomService.name}").build_struct())
///                     .name("sink1").build_struct(),
///                 ],
///             )
///             .sources(
///                 vec![
///                     FlowletDataFlowSource::builder()
///                     .linkedService(FlowletDataFlowSourceLinkedService::builder()
///                     .name("${exampleLinkedCustomService.name}").build_struct())
///                     .name("source1").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
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
///     let exampleFlowletDataFlow = flowlet_data_flow::create(
///         "exampleFlowletDataFlow",
///         FlowletDataFlowArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .name("example")
///             .script(
///                 "source(\n  allowSchemaDrift: true, \n  validateSchema: false, \n  limit: 100, \n  ignoreNoFilesFound: false, \n  documentForm: 'documentPerLine') ~> source1 \nsource1 sink(\n  allowSchemaDrift: true, \n  validateSchema: false, \n  skipDuplicateMapInputs: true, \n  skipDuplicateMapOutputs: true) ~> sink1\n",
///             )
///             .sinks(
///                 vec![
///                     FlowletDataFlowSink::builder()
///                     .flowlet(FlowletDataFlowSinkFlowlet::builder()
///                     .name("${example2FlowletDataFlow.name}").build_struct())
///                     .linkedService(FlowletDataFlowSinkLinkedService::builder()
///                     .name("${exampleLinkedCustomService.name}").build_struct())
///                     .name("sink1").build_struct(),
///                 ],
///             )
///             .sources(
///                 vec![
///                     FlowletDataFlowSource::builder()
///                     .flowlet(FlowletDataFlowSourceFlowlet::builder()
///                     .name("${example1FlowletDataFlow.name}").build_struct())
///                     .linkedService(FlowletDataFlowSourceLinkedService::builder()
///                     .name("${exampleLinkedCustomService.name}").build_struct())
///                     .name("source1").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleLinkedCustomService = linked_custom_service::create(
///         "exampleLinkedCustomService",
///         LinkedCustomServiceArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .name("linked_service")
///             .type_("AzureBlobStorage")
///             .type_properties_json(
///                 "{\n  \"connectionString\": \"${exampleAccount.primaryConnectionString}\"\n}\n",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory Flowlet Data Flow can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/flowletDataFlow:FlowletDataFlow example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/dataflows/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod flowlet_data_flow {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlowletDataFlowArgs {
        /// List of tags that can be used for describing the Data Factory Flowlet Data Flow.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Data Flow with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Flowlet Data Flow.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The folder that this Data Flow is in. If not specified, the Data Flow will appear at the root level.
        #[builder(into, default)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Data Factory Flowlet Data Flow. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The script for the Data Factory Flowlet Data Flow.
        #[builder(into, default)]
        pub script: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The script lines for the Data Factory Flowlet Data Flow.
        #[builder(into, default)]
        pub script_lines: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// One or more `sink` blocks as defined below.
        #[builder(into, default)]
        pub sinks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::datafactory::FlowletDataFlowSink>>,
        >,
        /// One or more `source` blocks as defined below.
        #[builder(into, default)]
        pub sources: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::datafactory::FlowletDataFlowSource>>,
        >,
        /// One or more `transformation` blocks as defined below.
        #[builder(into, default)]
        pub transformations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::datafactory::FlowletDataFlowTransformation>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FlowletDataFlowResult {
        /// List of tags that can be used for describing the Data Factory Flowlet Data Flow.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Data Flow with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Flowlet Data Flow.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The folder that this Data Flow is in. If not specified, the Data Flow will appear at the root level.
        pub folder: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Data Factory Flowlet Data Flow. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The script for the Data Factory Flowlet Data Flow.
        pub script: pulumi_gestalt_rust::Output<Option<String>>,
        /// The script lines for the Data Factory Flowlet Data Flow.
        pub script_lines: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// One or more `sink` blocks as defined below.
        pub sinks: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::datafactory::FlowletDataFlowSink>>,
        >,
        /// One or more `source` blocks as defined below.
        pub sources: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::datafactory::FlowletDataFlowSource>>,
        >,
        /// One or more `transformation` blocks as defined below.
        pub transformations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::datafactory::FlowletDataFlowTransformation>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FlowletDataFlowArgs,
    ) -> FlowletDataFlowResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let folder_binding = args.folder.get_output(context);
        let name_binding = args.name.get_output(context);
        let script_binding = args.script.get_output(context);
        let script_lines_binding = args.script_lines.get_output(context);
        let sinks_binding = args.sinks.get_output(context);
        let sources_binding = args.sources.get_output(context);
        let transformations_binding = args.transformations.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/flowletDataFlow:FlowletDataFlow".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folder".into(),
                    value: &folder_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "script".into(),
                    value: &script_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scriptLines".into(),
                    value: &script_lines_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sinks".into(),
                    value: &sinks_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sources".into(),
                    value: &sources_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transformations".into(),
                    value: &transformations_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FlowletDataFlowResult {
            annotations: o.get_field("annotations"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            folder: o.get_field("folder"),
            name: o.get_field("name"),
            script: o.get_field("script"),
            script_lines: o.get_field("scriptLines"),
            sinks: o.get_field("sinks"),
            sources: o.get_field("sources"),
            transformations: o.get_field("transformations"),
        }
    }
}
