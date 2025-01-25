/// A Cloud Deploy `CustomTargetType` defines a type of custom target that can be referenced in a
/// Cloud Deploy `Target` in order to facilitate deploying to other systems besides the supported runtimes.
///
///
/// To get more information about CustomTargetType, see:
///
/// * [API documentation](https://cloud.google.com/deploy/docs/api/reference/rest/v1/projects.locations.customTargetTypes)
/// * How-to Guides
///     * [Define and use a custom target type](https://cloud.google.com/deploy/docs/deploy-app-custom-target)
///
/// ## Example Usage
///
/// ### Clouddeploy Custom Target Type Basic
///
///
/// ```yaml
/// resources:
///   custom-target-type:
///     type: gcp:clouddeploy:CustomTargetType
///     properties:
///       location: us-central1
///       name: my-custom-target-type
///       description: My custom target type
///       annotations:
///         my_first_annotation: example-annotation-1
///         my_second_annotation: example-annotation-2
///       labels:
///         my_first_label: example-label-1
///         my_second_label: example-label-2
///       customActions:
///         renderAction: renderAction
///         deployAction: deployAction
/// ```
/// ### Clouddeploy Custom Target Type Git Skaffold Modules
///
///
/// ```yaml
/// resources:
///   custom-target-type:
///     type: gcp:clouddeploy:CustomTargetType
///     properties:
///       location: us-central1
///       name: my-custom-target-type
///       description: My custom target type
///       customActions:
///         renderAction: renderAction
///         deployAction: deployAction
///         includeSkaffoldModules:
///           - configs:
///               - my-config
///             git:
///               repo: http://github.com/example/example-repo.git
///               path: configs/skaffold.yaml
///               ref: main
/// ```
/// ### Clouddeploy Custom Target Type Gcs Skaffold Modules
///
///
/// ```yaml
/// resources:
///   custom-target-type:
///     type: gcp:clouddeploy:CustomTargetType
///     properties:
///       location: us-central1
///       name: my-custom-target-type
///       description: My custom target type
///       customActions:
///         renderAction: renderAction
///         deployAction: deployAction
///         includeSkaffoldModules:
///           - configs:
///               - my-config
///             googleCloudStorage:
///               source: gs://example-bucket/dir/configs/*
///               path: skaffold.yaml
/// ```
/// ### Clouddeploy Custom Target Type Gcb Repo Skaffold Modules
///
///
/// ```yaml
/// resources:
///   custom-target-type:
///     type: gcp:clouddeploy:CustomTargetType
///     properties:
///       location: us-central1
///       name: my-custom-target-type
///       description: My custom target type
///       customActions:
///         renderAction: renderAction
///         deployAction: deployAction
///         includeSkaffoldModules:
///           - configs:
///               - my-config
///             googleCloudBuildRepo:
///               repository: projects/example/locations/us-central1/connections/git/repositories/example-repo
///               path: configs/skaffold.yaml
///               ref: main
/// ```
///
/// ## Import
///
/// CustomTargetType can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/customTargetTypes/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, CustomTargetType can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:clouddeploy/customTargetType:CustomTargetType default projects/{{project}}/locations/{{location}}/customTargetTypes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:clouddeploy/customTargetType:CustomTargetType default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:clouddeploy/customTargetType:CustomTargetType default {{location}}/{{name}}
/// ```
///
pub mod custom_target_type {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomTargetTypeArgs {
        /// User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configures render and deploy for the `CustomTargetType` using Skaffold custom actions.
        /// Structure is documented below.
        #[builder(into, default)]
        pub custom_actions: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::clouddeploy::CustomTargetTypeCustomActions>,
        >,
        /// Description of the `CustomTargetType`. Max length is 255 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the source.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the `CustomTargetType`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CustomTargetTypeResult {
        /// User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Time at which the `CustomTargetType` was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Configures render and deploy for the `CustomTargetType` using Skaffold custom actions.
        /// Structure is documented below.
        pub custom_actions: pulumi_wasm_rust::Output<
            Option<super::super::types::clouddeploy::CustomTargetTypeCustomActions>,
        >,
        /// Resource id of the `CustomTargetType`.
        pub custom_target_type_id: pulumi_wasm_rust::Output<String>,
        /// Description of the `CustomTargetType`. Max length is 255 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The weak etag of the `CustomTargetType` resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the source.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of the `CustomTargetType`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Unique identifier of the `CustomTargetType`.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Time at which the `CustomTargetType` was updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CustomTargetTypeArgs,
    ) -> CustomTargetTypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let custom_actions_binding = args.custom_actions.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:clouddeploy/customTargetType:CustomTargetType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "customActions".into(),
                    value: &custom_actions_binding,
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
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "customActions".into(),
                },
                register_interface::ResultField {
                    name: "customTargetTypeId".into(),
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
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CustomTargetTypeResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            custom_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customActions").unwrap(),
            ),
            custom_target_type_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customTargetTypeId").unwrap(),
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
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
