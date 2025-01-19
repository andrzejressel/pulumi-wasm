/// An Integration connectors Endpoint Attachment.
///
///
/// To get more information about EndpointAttachment, see:
///
/// * [API documentation](https://cloud.google.com/integration-connectors/docs/reference/rest/v1/projects.locations.endpointAttachments)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/integration-connectors/docs/create-endpoint-attachment)
///
/// ## Example Usage
///
/// ### Integration Connectors Endpoint Attachment
///
///
/// ```yaml
/// resources:
///   sampleendpointattachment:
///     type: gcp:integrationconnectors:EndpointAttachment
///     properties:
///       name: test-endpoint-attachment
///       location: us-central1
///       description: tf created description
///       serviceAttachment: projects/connectors-example/regions/us-central1/serviceAttachments/test
///       labels:
///         foo: bar
/// ```
///
/// ## Import
///
/// EndpointAttachment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/endpointAttachments/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, EndpointAttachment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:integrationconnectors/endpointAttachment:EndpointAttachment default projects/{{project}}/locations/{{location}}/endpointAttachments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:integrationconnectors/endpointAttachment:EndpointAttachment default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:integrationconnectors/endpointAttachment:EndpointAttachment default {{location}}/{{name}}
/// ```
///
pub mod endpoint_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointAttachmentArgs {
        /// Description of the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Enable global access for endpoint attachment.
        #[builder(into, default)]
        pub endpoint_global_access: pulumi_wasm_rust::Output<Option<bool>>,
        /// Resource labels to represent user provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location in which Endpoint Attachment needs to be created.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of Endpoint Attachment needs to be created.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The path of the service attachment.
        #[builder(into)]
        pub service_attachment: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointAttachmentResult {
        /// Time the Namespace was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Description of the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enable global access for endpoint attachment.
        pub endpoint_global_access: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Private Service Connect connection endpoint ip.
        pub endpoint_ip: pulumi_wasm_rust::Output<String>,
        /// Resource labels to represent user provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location in which Endpoint Attachment needs to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of Endpoint Attachment needs to be created.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The path of the service attachment.
        pub service_attachment: pulumi_wasm_rust::Output<String>,
        /// Time the Namespace was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EndpointAttachmentArgs) -> EndpointAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let endpoint_global_access_binding = args.endpoint_global_access.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let service_attachment_binding = args.service_attachment.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:integrationconnectors/endpointAttachment:EndpointAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "endpointGlobalAccess".into(),
                    value: &endpoint_global_access_binding,
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
                    name: "serviceAttachment".into(),
                    value: &service_attachment_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "endpointGlobalAccess".into(),
                },
                register_interface::ResultField {
                    name: "endpointIp".into(),
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
                    name: "serviceAttachment".into(),
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
        EndpointAttachmentResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            endpoint_global_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointGlobalAccess").unwrap(),
            ),
            endpoint_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointIp").unwrap(),
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
            service_attachment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAttachment").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
