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
#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GdcApplicationEnvironmentArgs,
    ) -> GdcApplicationEnvironmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let application_environment_id_binding = args
            .application_environment_id
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let namespace_binding = args.namespace.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let serviceinstance_binding = args
            .serviceinstance
            .get_output(context)
            .get_inner();
        let spark_application_environment_config_binding = args
            .spark_application_environment_config
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataproc/gdcApplicationEnvironment:GdcApplicationEnvironment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "applicationEnvironmentId".into(),
                    value: &application_environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "serviceinstance".into(),
                    value: &serviceinstance_binding,
                },
                register_interface::ObjectField {
                    name: "sparkApplicationEnvironmentConfig".into(),
                    value: &spark_application_environment_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GdcApplicationEnvironmentResult {
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            application_environment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationEnvironmentId"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            effective_annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            namespace: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namespace"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            serviceinstance: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceinstance"),
            ),
            spark_application_environment_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sparkApplicationEnvironmentConfig"),
            ),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
