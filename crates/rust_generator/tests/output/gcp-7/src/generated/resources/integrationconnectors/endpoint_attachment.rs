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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointAttachmentArgs {
        /// Description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enable global access for endpoint attachment.
        #[builder(into, default)]
        pub endpoint_global_access: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Resource labels to represent user provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location in which Endpoint Attachment needs to be created.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of Endpoint Attachment needs to be created.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The path of the service attachment.
        #[builder(into)]
        pub service_attachment: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointAttachmentResult {
        /// Time the Namespace was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Description of the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enable global access for endpoint attachment.
        pub endpoint_global_access: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Private Service Connect connection endpoint ip.
        pub endpoint_ip: pulumi_gestalt_rust::Output<String>,
        /// Resource labels to represent user provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location in which Endpoint Attachment needs to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Name of Endpoint Attachment needs to be created.
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
        /// The path of the service attachment.
        pub service_attachment: pulumi_gestalt_rust::Output<String>,
        /// Time the Namespace was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointAttachmentArgs,
    ) -> EndpointAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let endpoint_global_access_binding = args
            .endpoint_global_access
            .get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let service_attachment_binding = args.service_attachment.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:integrationconnectors/endpointAttachment:EndpointAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointGlobalAccess".into(),
                    value: &endpoint_global_access_binding.drop_type(),
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
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAttachment".into(),
                    value: &service_attachment_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EndpointAttachmentResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            endpoint_global_access: o.get_field("endpointGlobalAccess"),
            endpoint_ip: o.get_field("endpointIp"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            service_attachment: o.get_field("serviceAttachment"),
            update_time: o.get_field("updateTime"),
        }
    }
}
