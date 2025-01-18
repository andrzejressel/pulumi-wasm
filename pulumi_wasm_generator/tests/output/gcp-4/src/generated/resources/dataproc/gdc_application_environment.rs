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
pub mod gdc_application_environment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GdcApplicationEnvironmentArgs {
        /// The annotations to associate with this application environment. Annotations may be used to store client information, but are not used by the server.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The id of the application environment
        #[builder(into, default)]
        pub application_environment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// User-provided human-readable name to be used in user interfaces.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The labels to associate with this application environment. Labels may be used for filtering and billing tracking.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the application environment
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the namespace in which to create this ApplicationEnvironment. This namespace must already exist in the cluster
        #[builder(into, default)]
        pub namespace: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The id of the service instance to which this application environment belongs.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub serviceinstance: pulumi_wasm_rust::Output<String>,
        /// Represents the SparkApplicationEnvironmentConfig.
        /// Structure is documented below.
        #[builder(into, default)]
        pub spark_application_environment_config: pulumi_wasm_rust::Output<
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
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The id of the application environment
        pub application_environment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The timestamp when the resource was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// User-provided human-readable name to be used in user interfaces.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The labels to associate with this application environment. Labels may be used for filtering and billing tracking.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the application environment
        pub location: pulumi_wasm_rust::Output<String>,
        /// Identifier. The name of the application environment. Format: projects/{project}/locations/{location}/serviceInstances/{service_instance}/applicationEnvironments/{application_environment_id}
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the namespace in which to create this ApplicationEnvironment. This namespace must already exist in the cluster
        pub namespace: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The id of the service instance to which this application environment belongs.
        ///
        ///
        /// - - -
        pub serviceinstance: pulumi_wasm_rust::Output<String>,
        /// Represents the SparkApplicationEnvironmentConfig.
        /// Structure is documented below.
        pub spark_application_environment_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::dataproc::GdcApplicationEnvironmentSparkApplicationEnvironmentConfig,
            >,
        >,
        /// System generated unique identifier for this application environment, formatted as UUID4.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the resource was most recently updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: GdcApplicationEnvironmentArgs,
    ) -> GdcApplicationEnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_inner();
        let application_environment_id_binding = args
            .application_environment_id
            .get_inner();
        let display_name_binding = args.display_name.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let namespace_binding = args.namespace.get_inner();
        let project_binding = args.project.get_inner();
        let serviceinstance_binding = args.serviceinstance.get_inner();
        let spark_application_environment_config_binding = args
            .spark_application_environment_config
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "applicationEnvironmentId".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namespace".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "serviceinstance".into(),
                },
                register_interface::ResultField {
                    name: "sparkApplicationEnvironmentConfig".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GdcApplicationEnvironmentResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            application_environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationEnvironmentId").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespace").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            serviceinstance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceinstance").unwrap(),
            ),
            spark_application_environment_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sparkApplicationEnvironmentConfig").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
