/// Manages a Pipeline inside a Azure Data Factory.
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
///     let examplePipeline = pipeline::create(
///         "examplePipeline",
///         PipelineArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### With Activities
///
/// ```yaml
/// resources:
///   test:
///     type: azure:datafactory:Pipeline
///     properties:
///       name: example
///       dataFactoryId: ${testAzurermDataFactory.id}
///       variables:
///         bob: item1
///       activitiesJson: |
///         [
///             {
///                 "name": "Append variable1",
///                 "type": "AppendVariable",
///                 "dependsOn": [],
///                 "userProperties": [],
///                 "typeProperties": {
///                   "variableName": "bob",
///                   "value": "something"
///                 }
///             }
///         ]
/// ```
///
/// ## Import
///
/// Data Factory Pipeline's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/pipeline:Pipeline example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/pipelines/example
/// ```
///
pub mod pipeline {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PipelineArgs {
        /// A JSON object that contains the activities that will be associated with the Data Factory Pipeline.
        #[builder(into, default)]
        pub activities_json: pulumi_wasm_rust::Output<Option<String>>,
        /// List of tags that can be used for describing the Data Factory Pipeline.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The max number of concurrent runs for the Data Factory Pipeline. Must be between `1` and `50`.
        #[builder(into, default)]
        pub concurrency: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Pipeline.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The folder that this Pipeline is in. If not specified, the Pipeline will appear at the root level.
        #[builder(into, default)]
        pub folder: pulumi_wasm_rust::Output<Option<String>>,
        /// The TimeSpan value after which an Azure Monitoring Metric is fired.
        #[builder(into, default)]
        pub moniter_metrics_after_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Data Factory Pipeline. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of parameters to associate with the Data Factory Pipeline.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of variables to associate with the Data Factory Pipeline.
        #[builder(into, default)]
        pub variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PipelineResult {
        /// A JSON object that contains the activities that will be associated with the Data Factory Pipeline.
        pub activities_json: pulumi_wasm_rust::Output<Option<String>>,
        /// List of tags that can be used for describing the Data Factory Pipeline.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The max number of concurrent runs for the Data Factory Pipeline. Must be between `1` and `50`.
        pub concurrency: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Pipeline.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The folder that this Pipeline is in. If not specified, the Pipeline will appear at the root level.
        pub folder: pulumi_wasm_rust::Output<Option<String>>,
        /// The TimeSpan value after which an Azure Monitoring Metric is fired.
        pub moniter_metrics_after_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Data Factory Pipeline. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Pipeline.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of variables to associate with the Data Factory Pipeline.
        pub variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PipelineArgs) -> PipelineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let activities_json_binding = args.activities_json.get_inner();
        let annotations_binding = args.annotations.get_inner();
        let concurrency_binding = args.concurrency.get_inner();
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let description_binding = args.description.get_inner();
        let folder_binding = args.folder.get_inner();
        let moniter_metrics_after_duration_binding = args
            .moniter_metrics_after_duration
            .get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let variables_binding = args.variables.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/pipeline:Pipeline".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "activitiesJson".into(),
                    value: &activities_json_binding,
                },
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "concurrency".into(),
                    value: &concurrency_binding,
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
                    name: "moniterMetricsAfterDuration".into(),
                    value: &moniter_metrics_after_duration_binding,
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
                    name: "variables".into(),
                    value: &variables_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "activitiesJson".into(),
                },
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "concurrency".into(),
                },
                register_interface::ResultField {
                    name: "dataFactoryId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "folder".into(),
                },
                register_interface::ResultField {
                    name: "moniterMetricsAfterDuration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "variables".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PipelineResult {
            activities_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activitiesJson").unwrap(),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            concurrency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("concurrency").unwrap(),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFactoryId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            folder: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("folder").unwrap(),
            ),
            moniter_metrics_after_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("moniterMetricsAfterDuration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            variables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("variables").unwrap(),
            ),
        }
    }
}
