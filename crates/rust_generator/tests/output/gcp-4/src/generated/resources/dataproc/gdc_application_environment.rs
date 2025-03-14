/// An ApplicationEnvironment contains shared configuration that may be referenced by multiple SparkApplications.
///
///
/// To get more information about ApplicationEnvironment, see:
///
/// * [API documentation](https://cloud.google.com/dataproc-gdc/docs/reference/rest/v1/projects.locations.applicationEnvironments)
/// * How-to Guides
///     * [Dataproc Intro](https://cloud.google.com/dataproc/)
///
/// ## Example Usage
///
/// ### Dataprocgdc Applicationenvironment Basic
///
///
/// ```yaml
/// resources:
///   application-environment:
///     type: gcp:dataproc:GdcApplicationEnvironment
///     properties:
///       applicationEnvironmentId: dp-tf-e2e-application-environment-basic
///       serviceinstance: do-not-delete-dataproc-gdc-instance
///       project: my-project
///       location: us-west2
///       namespace: default
/// ```
/// ### Dataprocgdc Applicationenvironment
///
///
/// ```yaml
/// resources:
///   application-environment:
///     type: gcp:dataproc:GdcApplicationEnvironment
///     properties:
///       applicationEnvironmentId: dp-tf-e2e-application-environment
///       serviceinstance: do-not-delete-dataproc-gdc-instance
///       project: my-project
///       location: us-west2
///       namespace: default
///       displayName: An application environment
///       labels:
///         test-label: label-value
///       annotations:
///         an_annotation: annotation_value
///       sparkApplicationEnvironmentConfig:
///         defaultProperties:
///           spark.executor.memory: 4g
///         defaultVersion: '1.2'
/// ```
///
/// ## Import
///
/// ApplicationEnvironment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/serviceInstances/{{serviceinstance}}/applicationEnvironments/{{application_environment_id}}`
///
/// * `{{project}}/{{location}}/{{serviceinstance}}/{{application_environment_id}}`
///
/// * `{{location}}/{{serviceinstance}}/{{application_environment_id}}`
///
/// When using the `pulumi import` command, ApplicationEnvironment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataproc/gdcApplicationEnvironment:GdcApplicationEnvironment default projects/{{project}}/locations/{{location}}/serviceInstances/{{serviceinstance}}/applicationEnvironments/{{application_environment_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/gdcApplicationEnvironment:GdcApplicationEnvironment default {{project}}/{{location}}/{{serviceinstance}}/{{application_environment_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/gdcApplicationEnvironment:GdcApplicationEnvironment default {{location}}/{{serviceinstance}}/{{application_environment_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod gdc_application_environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GdcApplicationEnvironmentArgs {
        /// The annotations to associate with this application environment. Annotations may be used to store client information, but are not used by the server.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The id of the application environment
        #[builder(into, default)]
        pub application_environment_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// User-provided human-readable name to be used in user interfaces.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The labels to associate with this application environment. Labels may be used for filtering and billing tracking.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the application environment
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the namespace in which to create this ApplicationEnvironment. This namespace must already exist in the cluster
        #[builder(into, default)]
        pub namespace: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The id of the service instance to which this application environment belongs.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub serviceinstance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Represents the SparkApplicationEnvironmentConfig.
        /// Structure is documented below.
        #[builder(into, default)]
        pub spark_application_environment_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::dataproc::GdcApplicationEnvironmentSparkApplicationEnvironmentConfig,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GdcApplicationEnvironmentResult {
        /// The annotations to associate with this application environment. Annotations may be used to store client information, but are not used by the server.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The id of the application environment
        pub application_environment_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The timestamp when the resource was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// User-provided human-readable name to be used in user interfaces.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The labels to associate with this application environment. Labels may be used for filtering and billing tracking.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the application environment
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Identifier. The name of the application environment. Format: projects/{project}/locations/{location}/serviceInstances/{service_instance}/applicationEnvironments/{application_environment_id}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the namespace in which to create this ApplicationEnvironment. This namespace must already exist in the cluster
        pub namespace: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The id of the service instance to which this application environment belongs.
        ///
        ///
        /// - - -
        pub serviceinstance: pulumi_gestalt_rust::Output<String>,
        /// Represents the SparkApplicationEnvironmentConfig.
        /// Structure is documented below.
        pub spark_application_environment_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::dataproc::GdcApplicationEnvironmentSparkApplicationEnvironmentConfig,
            >,
        >,
        /// System generated unique identifier for this application environment, formatted as UUID4.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the resource was most recently updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GdcApplicationEnvironmentArgs,
    ) -> GdcApplicationEnvironmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let application_environment_id_binding = args
            .application_environment_id
            .get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let namespace_binding = args.namespace.get_output(context);
        let project_binding = args.project.get_output(context);
        let serviceinstance_binding = args.serviceinstance.get_output(context);
        let spark_application_environment_config_binding = args
            .spark_application_environment_config
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataproc/gdcApplicationEnvironment:GdcApplicationEnvironment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationEnvironmentId".into(),
                    value: &application_environment_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceinstance".into(),
                    value: &serviceinstance_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sparkApplicationEnvironmentConfig".into(),
                    value: &spark_application_environment_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GdcApplicationEnvironmentResult {
            annotations: o.get_field("annotations"),
            application_environment_id: o.get_field("applicationEnvironmentId"),
            create_time: o.get_field("createTime"),
            display_name: o.get_field("displayName"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            namespace: o.get_field("namespace"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            serviceinstance: o.get_field("serviceinstance"),
            spark_application_environment_config: o
                .get_field("sparkApplicationEnvironmentConfig"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
