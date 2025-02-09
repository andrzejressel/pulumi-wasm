/// Manages a Pipeline inside a Azure Data Factory.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod pipeline {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PipelineArgs {
        /// A JSON object that contains the activities that will be associated with the Data Factory Pipeline.
        #[builder(into, default)]
        pub activities_json: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of tags that can be used for describing the Data Factory Pipeline.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The max number of concurrent runs for the Data Factory Pipeline. Must be between `1` and `50`.
        #[builder(into, default)]
        pub concurrency: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Pipeline.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The folder that this Pipeline is in. If not specified, the Pipeline will appear at the root level.
        #[builder(into, default)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The TimeSpan value after which an Azure Monitoring Metric is fired.
        #[builder(into, default)]
        pub moniter_metrics_after_duration: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the Data Factory Pipeline. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of parameters to associate with the Data Factory Pipeline.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of variables to associate with the Data Factory Pipeline.
        #[builder(into, default)]
        pub variables: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PipelineResult {
        /// A JSON object that contains the activities that will be associated with the Data Factory Pipeline.
        pub activities_json: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of tags that can be used for describing the Data Factory Pipeline.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The max number of concurrent runs for the Data Factory Pipeline. Must be between `1` and `50`.
        pub concurrency: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Pipeline.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The folder that this Pipeline is in. If not specified, the Pipeline will appear at the root level.
        pub folder: pulumi_gestalt_rust::Output<Option<String>>,
        /// The TimeSpan value after which an Azure Monitoring Metric is fired.
        pub moniter_metrics_after_duration: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Data Factory Pipeline. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Pipeline.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of variables to associate with the Data Factory Pipeline.
        pub variables: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PipelineArgs,
    ) -> PipelineResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let activities_json_binding_1 = args.activities_json.get_output(context);
        let activities_json_binding = activities_json_binding_1.get_inner();
        let annotations_binding_1 = args.annotations.get_output(context);
        let annotations_binding = annotations_binding_1.get_inner();
        let concurrency_binding_1 = args.concurrency.get_output(context);
        let concurrency_binding = concurrency_binding_1.get_inner();
        let data_factory_id_binding_1 = args.data_factory_id.get_output(context);
        let data_factory_id_binding = data_factory_id_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let folder_binding_1 = args.folder.get_output(context);
        let folder_binding = folder_binding_1.get_inner();
        let moniter_metrics_after_duration_binding_1 = args
            .moniter_metrics_after_duration
            .get_output(context);
        let moniter_metrics_after_duration_binding = moniter_metrics_after_duration_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let parameters_binding_1 = args.parameters.get_output(context);
        let parameters_binding = parameters_binding_1.get_inner();
        let variables_binding_1 = args.variables.get_output(context);
        let variables_binding = variables_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/pipeline:Pipeline".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        PipelineResult {
            activities_json: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("activitiesJson"),
            ),
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            concurrency: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("concurrency"),
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
            moniter_metrics_after_duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("moniterMetricsAfterDuration"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            variables: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("variables"),
            ),
        }
    }
}
