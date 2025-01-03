/// The Cloud Deploy `DeliveryPipeline` resource
///
/// ## Example Usage
///
/// ### Canary_delivery_pipeline
/// Creates a basic Cloud Deploy delivery pipeline
/// ```yaml
/// resources:
///   primary:
///     type: gcp:clouddeploy:DeliveryPipeline
///     properties:
///       location: us-west1
///       name: pipeline
///       description: basic description
///       project: my-project-name
///       serialPipeline:
///         stages:
///           - deployParameters:
///               - values:
///                   deployParameterKey: deployParameterValue
///                 matchTargetLabels: {}
///             profiles:
///               - example-profile-one
///               - example-profile-two
///             targetId: example-target-one
///           - profiles: []
///             targetId: example-target-two
///       annotations:
///         my_first_annotation: example-annotation-1
///         my_second_annotation: example-annotation-2
///       labels:
///         my_first_label: example-label-1
///         my_second_label: example-label-2
/// ```
/// ### Canary_service_networking_delivery_pipeline
/// Creates a basic Cloud Deploy delivery pipeline
/// ```yaml
/// resources:
///   primary:
///     type: gcp:clouddeploy:DeliveryPipeline
///     properties:
///       location: us-west1
///       name: pipeline
///       description: basic description
///       project: my-project-name
///       serialPipeline:
///         stages:
///           - deployParameters:
///               - values:
///                   deployParameterKey: deployParameterValue
///                 matchTargetLabels: {}
///             profiles:
///               - example-profile-one
///               - example-profile-two
///             targetId: example-target-one
///           - profiles: []
///             targetId: example-target-two
///       annotations:
///         my_first_annotation: example-annotation-1
///         my_second_annotation: example-annotation-2
///       labels:
///         my_first_label: example-label-1
///         my_second_label: example-label-2
/// ```
/// ### Canaryrun_delivery_pipeline
/// Creates a basic Cloud Deploy delivery pipeline
/// ```yaml
/// resources:
///   primary:
///     type: gcp:clouddeploy:DeliveryPipeline
///     properties:
///       location: us-west1
///       name: pipeline
///       description: basic description
///       project: my-project-name
///       serialPipeline:
///         stages:
///           - deployParameters:
///               - values:
///                   deployParameterKey: deployParameterValue
///                 matchTargetLabels: {}
///             profiles:
///               - example-profile-one
///               - example-profile-two
///             targetId: example-target-one
///           - profiles: []
///             targetId: example-target-two
///       annotations:
///         my_first_annotation: example-annotation-1
///         my_second_annotation: example-annotation-2
///       labels:
///         my_first_label: example-label-1
///         my_second_label: example-label-2
/// ```
/// ### Delivery_pipeline
/// Creates a basic Cloud Deploy delivery pipeline
/// ```yaml
/// resources:
///   primary:
///     type: gcp:clouddeploy:DeliveryPipeline
///     properties:
///       location: us-west1
///       name: pipeline
///       description: basic description
///       project: my-project-name
///       serialPipeline:
///         stages:
///           - deployParameters:
///               - values:
///                   deployParameterKey: deployParameterValue
///                 matchTargetLabels: {}
///             profiles:
///               - example-profile-one
///               - example-profile-two
///             targetId: example-target-one
///           - profiles: []
///             targetId: example-target-two
///       annotations:
///         my_first_annotation: example-annotation-1
///         my_second_annotation: example-annotation-2
///       labels:
///         my_first_label: example-label-1
///         my_second_label: example-label-2
/// ```
/// ### Verify_delivery_pipeline
/// tests creating and updating a delivery pipeline with deployment verification strategy
/// ```yaml
/// resources:
///   primary:
///     type: gcp:clouddeploy:DeliveryPipeline
///     properties:
///       location: us-west1
///       name: pipeline
///       description: basic description
///       project: my-project-name
///       serialPipeline:
///         stages:
///           - deployParameters:
///               - values:
///                   deployParameterKey: deployParameterValue
///                 matchTargetLabels: {}
///             profiles:
///               - example-profile-one
///               - example-profile-two
///             targetId: example-target-one
///           - profiles: []
///             targetId: example-target-two
///       annotations:
///         my_first_annotation: example-annotation-1
///         my_second_annotation: example-annotation-2
///       labels:
///         my_first_label: example-label-1
///         my_second_label: example-label-2
/// ```
///
/// ## Import
///
/// DeliveryPipeline can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/deliveryPipelines/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, DeliveryPipeline can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:clouddeploy/deliveryPipeline:DeliveryPipeline default projects/{{project}}/locations/{{location}}/deliveryPipelines/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:clouddeploy/deliveryPipeline:DeliveryPipeline default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:clouddeploy/deliveryPipeline:DeliveryPipeline default {{location}}/{{name}}
/// ```
///
pub mod delivery_pipeline {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeliveryPipelineArgs {
        /// User annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See
        /// https://google.aip.dev/128#annotations for more details such as format and size limitations. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Description of the `DeliveryPipeline`. Max length is 255 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Labels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the
        /// following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and
        /// dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a
        /// lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values
        /// are additionally constrained to be <= 128 bytes. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field `effective_labels` for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of the `DeliveryPipeline`. Format is `a-z?`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// SerialPipeline defines a sequential set of stages for a `DeliveryPipeline`.
        #[builder(into, default)]
        pub serial_pipeline: pulumi_wasm_rust::Output<
            Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipeline>,
        >,
        /// When suspended, no new releases or rollouts can be created, but in-progress ones will complete.
        #[builder(into, default)]
        pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct DeliveryPipelineResult {
        /// User annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See
        /// https://google.aip.dev/128#annotations for more details such as format and size limitations. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Output only. Information around the state of the Delivery Pipeline.
        pub conditions: pulumi_wasm_rust::Output<
            Vec<super::super::types::clouddeploy::DeliveryPipelineCondition>,
        >,
        /// Output only. Time at which the pipeline was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Description of the `DeliveryPipeline`. Max length is 255 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Labels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the
        /// following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and
        /// dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a
        /// lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values
        /// are additionally constrained to be <= 128 bytes. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field `effective_labels` for all of the labels present on the
        /// resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of the `DeliveryPipeline`. Format is `a-z?`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// SerialPipeline defines a sequential set of stages for a `DeliveryPipeline`.
        pub serial_pipeline: pulumi_wasm_rust::Output<
            Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipeline>,
        >,
        /// When suspended, no new releases or rollouts can be created, but in-progress ones will complete.
        pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
        /// Output only. Unique identifier of the `DeliveryPipeline`.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. Most recent time at which the pipeline was updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DeliveryPipelineArgs) -> DeliveryPipelineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_inner();
        let description_binding = args.description.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let serial_pipeline_binding = args.serial_pipeline.get_inner();
        let suspended_binding = args.suspended.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:clouddeploy/deliveryPipeline:DeliveryPipeline".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
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
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "serialPipeline".into(),
                    value: &serial_pipeline_binding,
                },
                register_interface::ObjectField {
                    name: "suspended".into(),
                    value: &suspended_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "conditions".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
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
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "serialPipeline".into(),
                },
                register_interface::ResultField {
                    name: "suspended".into(),
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
        DeliveryPipelineResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            conditions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("conditions").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
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
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            serial_pipeline: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serialPipeline").unwrap(),
            ),
            suspended: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("suspended").unwrap(),
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
