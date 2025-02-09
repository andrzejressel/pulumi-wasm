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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod custom_target_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomTargetTypeArgs {
        /// User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configures render and deploy for the `CustomTargetType` using Skaffold custom actions.
        /// Structure is documented below.
        #[builder(into, default)]
        pub custom_actions: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::clouddeploy::CustomTargetTypeCustomActions>,
        >,
        /// Description of the `CustomTargetType`. Max length is 255 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the source.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the `CustomTargetType`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CustomTargetTypeResult {
        /// User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Time at which the `CustomTargetType` was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Configures render and deploy for the `CustomTargetType` using Skaffold custom actions.
        /// Structure is documented below.
        pub custom_actions: pulumi_gestalt_rust::Output<
            Option<super::super::types::clouddeploy::CustomTargetTypeCustomActions>,
        >,
        /// Resource id of the `CustomTargetType`.
        pub custom_target_type_id: pulumi_gestalt_rust::Output<String>,
        /// Description of the `CustomTargetType`. Max length is 255 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The weak etag of the `CustomTargetType` resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the source.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Name of the `CustomTargetType`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Unique identifier of the `CustomTargetType`.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Time at which the `CustomTargetType` was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CustomTargetTypeArgs,
    ) -> CustomTargetTypeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let annotations_binding_1 = args.annotations.get_output(context);
        let annotations_binding = annotations_binding_1.get_inner();
        let custom_actions_binding_1 = args.custom_actions.get_output(context);
        let custom_actions_binding = custom_actions_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let labels_binding_1 = args.labels.get_output(context);
        let labels_binding = labels_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        CustomTargetTypeResult {
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            custom_actions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customActions"),
            ),
            custom_target_type_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customTargetTypeId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
