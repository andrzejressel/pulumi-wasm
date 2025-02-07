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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FlowletDataFlowArgs,
    ) -> FlowletDataFlowResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let data_factory_id_binding = args
            .data_factory_id
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let folder_binding = args.folder.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let script_binding = args.script.get_output(context).get_inner();
        let script_lines_binding = args.script_lines.get_output(context).get_inner();
        let sinks_binding = args.sinks.get_output(context).get_inner();
        let sources_binding = args.sources.get_output(context).get_inner();
        let transformations_binding = args
            .transformations
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/flowletDataFlow:FlowletDataFlow".into(),
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
        FlowletDataFlowResult {
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
