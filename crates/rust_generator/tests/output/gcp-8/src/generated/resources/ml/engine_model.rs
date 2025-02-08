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
#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EngineModelArgs,
    ) -> EngineModelResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let default_version_binding = args
            .default_version
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let online_prediction_console_logging_binding = args
            .online_prediction_console_logging
            .get_output(context)
            .get_inner();
        let online_prediction_logging_binding = args
            .online_prediction_logging
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let regions_binding = args.regions.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:ml/engineModel:EngineModel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultVersion".into(),
                    value: &default_version_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "onlinePredictionConsoleLogging".into(),
                    value: &online_prediction_console_logging_binding,
                },
                register_interface::ObjectField {
                    name: "onlinePredictionLogging".into(),
                    value: &online_prediction_logging_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "regions".into(),
                    value: &regions_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EngineModelResult {
            default_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultVersion"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            online_prediction_console_logging: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("onlinePredictionConsoleLogging"),
            ),
            online_prediction_logging: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("onlinePredictionLogging"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            regions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("regions"),
            ),
        }
    }
}
