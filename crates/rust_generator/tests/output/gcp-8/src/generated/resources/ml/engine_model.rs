/// Represents a machine learning solution.
///
/// A model can have multiple versions, each of which is a deployed, trained model
/// ready to receive prediction requests. The model itself is just a container.
///
///
/// To get more information about Model, see:
///
/// * [API documentation](https://cloud.google.com/ai-platform/prediction/docs/reference/rest/v1/projects.models)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/ai-platform/prediction/docs/deploying-models)
///
/// ## Example Usage
///
/// ### Ml Model Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = engine_model::create(
///         "default",
///         EngineModelArgs::builder()
///             .description("My model")
///             .name("default")
///             .regions("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Ml Model Full
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:ml:EngineModel
///     properties:
///       name: default
///       description: My model
///       regions: us-central1
///       labels:
///         my_model: foo
///       onlinePredictionLogging: true
///       onlinePredictionConsoleLogging: true
/// ```
///
/// ## Import
///
/// Model can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/models/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Model can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:ml/engineModel:EngineModel default projects/{{project}}/models/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:ml/engineModel:EngineModel default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:ml/engineModel:EngineModel default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod engine_model {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EngineModelArgs {
        /// The default version of the model. This version will be used to handle
        /// prediction requests that do not specify a version.
        /// Structure is documented below.
        #[builder(into, default)]
        pub default_version: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ml::EngineModelDefaultVersion>,
        >,
        /// The description specified for the model when it was created.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more labels that you can add, to organize your models.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name specified for the model.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If true, online prediction nodes send stderr and stdout streams to Stackdriver Logging
        #[builder(into, default)]
        pub online_prediction_console_logging: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// If true, online prediction access logs are sent to StackDriver Logging.
        #[builder(into, default)]
        pub online_prediction_logging: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The list of regions where the model is going to be deployed.
        /// Currently only one region per model is supported
        #[builder(into, default)]
        pub regions: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EngineModelResult {
        /// The default version of the model. This version will be used to handle
        /// prediction requests that do not specify a version.
        /// Structure is documented below.
        pub default_version: pulumi_gestalt_rust::Output<
            Option<super::super::types::ml::EngineModelDefaultVersion>,
        >,
        /// The description specified for the model when it was created.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// One or more labels that you can add, to organize your models.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name specified for the model.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// If true, online prediction nodes send stderr and stdout streams to Stackdriver Logging
        pub online_prediction_console_logging: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If true, online prediction access logs are sent to StackDriver Logging.
        pub online_prediction_logging: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The list of regions where the model is going to be deployed.
        /// Currently only one region per model is supported
        pub regions: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EngineModelArgs,
    ) -> EngineModelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let default_version_binding = args.default_version.get_output(context);
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let online_prediction_console_logging_binding = args
            .online_prediction_console_logging
            .get_output(context);
        let online_prediction_logging_binding = args
            .online_prediction_logging
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let regions_binding = args.regions.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:ml/engineModel:EngineModel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultVersion".into(),
                    value: default_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "onlinePredictionConsoleLogging".into(),
                    value: online_prediction_console_logging_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "onlinePredictionLogging".into(),
                    value: online_prediction_logging_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regions".into(),
                    value: regions_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EngineModelResult {
            default_version: o.get_field("defaultVersion"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            online_prediction_console_logging: o
                .get_field("onlinePredictionConsoleLogging"),
            online_prediction_logging: o.get_field("onlinePredictionLogging"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            regions: o.get_field("regions"),
        }
    }
}
