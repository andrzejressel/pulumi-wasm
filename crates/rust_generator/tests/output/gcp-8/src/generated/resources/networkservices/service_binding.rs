/// ## Example Usage
///
/// ### Network Services Service Binding Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:servicedirectory:Namespace
///     properties:
///       namespaceId: my-namespace
///       location: us-central1
///   defaultService:
///     type: gcp:servicedirectory:Service
///     name: default
///     properties:
///       serviceId: my-service
///       namespace: ${default.id}
///       metadata:
///         stage: prod
///         region: us-central1
///   defaultServiceBinding:
///     type: gcp:networkservices:ServiceBinding
///     name: default
///     properties:
///       name: my-service-binding
///       labels:
///         foo: bar
///       description: my description
///       service: ${defaultService.id}
/// ```
///
/// ## Import
///
/// ServiceBinding can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/serviceBindings/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, ServiceBinding can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/serviceBinding:ServiceBinding default projects/{{project}}/locations/global/serviceBindings/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/serviceBinding:ServiceBinding default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/serviceBinding:ServiceBinding default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceBindingArgs {
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of label tags associated with the ServiceBinding resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the ServiceBinding resource.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The full Service Directory Service name of the format
        /// projects/*/locations/*/namespaces/*/services/*
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceBindingResult {
        /// Time the ServiceBinding was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of label tags associated with the ServiceBinding resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the ServiceBinding resource.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The full Service Directory Service name of the format
        /// projects/*/locations/*/namespaces/*/services/*
        pub service: pulumi_gestalt_rust::Output<String>,
        /// Time the ServiceBinding was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServiceBindingArgs,
    ) -> ServiceBindingResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let labels_binding_1 = args.labels.get_output(context);
        let labels_binding = labels_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let service_binding_1 = args.service.get_output(context);
        let service_binding = service_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkservices/serviceBinding:ServiceBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
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
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceBindingResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
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
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("service"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
