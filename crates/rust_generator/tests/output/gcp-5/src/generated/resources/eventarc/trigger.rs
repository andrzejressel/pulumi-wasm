/// The Eventarc Trigger resource
///
/// ## Example Usage
///
/// ### Basic
/// ```yaml
/// resources:
///   primary:
///     type: gcp:eventarc:Trigger
///     properties:
///       name: name
///       location: europe-west1
///       matchingCriterias:
///         - attribute: type
///           value: google.cloud.pubsub.topic.v1.messagePublished
///       destination:
///         cloudRunService:
///           service: ${default.name}
///           region: europe-west1
///       labels:
///         foo: bar
///   foo:
///     type: gcp:pubsub:Topic
///     properties:
///       name: topic
///   default:
///     type: gcp:cloudrun:Service
///     properties:
///       name: eventarc-service
///       location: europe-west1
///       metadata:
///         namespace: my-project-name
///       template:
///         spec:
///           containers:
///             - image: gcr.io/cloudrun/hello
///               ports:
///                 - containerPort: 8080
///           containerConcurrency: 50
///           timeoutSeconds: 100
///       traffics:
///         - percent: 100
///           latestRevision: true
/// ```
///
/// ## Import
///
/// Trigger can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/triggers/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Trigger can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:eventarc/trigger:Trigger default projects/{{project}}/locations/{{location}}/triggers/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:eventarc/trigger:Trigger default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:eventarc/trigger:Trigger default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod trigger {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerArgs {
        /// Optional. The name of the channel associated with the trigger in
        /// `projects/{project}/locations/{location}/channels/{channel}` format. You must provide a channel to receive events from
        /// Eventarc SaaS partners.
        #[builder(into, default)]
        pub channel: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. Destination specifies where the events should be sent to.
        #[builder(into)]
        pub destination: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::eventarc::TriggerDestination,
        >,
        /// Optional. EventDataContentType specifies the type of payload in MIME format that is expected from the CloudEvent data
        /// field. This is set to `application/json` if the value is not defined.
        #[builder(into, default)]
        pub event_data_content_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. User labels attached to the triggers that can be used to group resources. **Note**: This field is
        /// non-authoritative, and will only manage the labels present in your configuration. Please refer to the field
        /// `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. null The list of filters that applies to event attributes. Only events that match all the provided filters will be sent to the destination.
        #[builder(into)]
        pub matching_criterias: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::eventarc::TriggerMatchingCriteria>,
        >,
        /// Required. The resource name of the trigger. Must be unique within the location on the project.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. The IAM service account email associated with the trigger. The service account represents the identity of the
        /// trigger. The principal who calls this API must have `iam.serviceAccounts.actAs` permission in the service account. See
        /// https://cloud.google.com/iam/docs/understanding-service-accounts#sa_common for more information. For Cloud Run
        /// destinations, this service account is used to generate identity tokens when invoking the service. See
        /// https://cloud.google.com/run/docs/triggering/pubsub-push#create-service-account for information on how to invoke
        /// authenticated Cloud Run services. In order to create Audit Log triggers, the service account should also have
        /// `roles/eventarc.eventReceiver` IAM role.
        #[builder(into, default)]
        pub service_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. In order to deliver messages, Eventarc may use other GCP products as transport intermediary. This field
        /// contains a reference to that transport intermediary. This information can be used for debugging purposes.
        #[builder(into, default)]
        pub transport: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventarc::TriggerTransport>,
        >,
    }
    #[allow(dead_code)]
    pub struct TriggerResult {
        /// Optional. The name of the channel associated with the trigger in
        /// `projects/{project}/locations/{location}/channels/{channel}` format. You must provide a channel to receive events from
        /// Eventarc SaaS partners.
        pub channel: pulumi_gestalt_rust::Output<Option<String>>,
        /// Output only. The reason(s) why a trigger is in FAILED state.
        pub conditions: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. The creation time.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Required. Destination specifies where the events should be sent to.
        pub destination: pulumi_gestalt_rust::Output<
            super::super::types::eventarc::TriggerDestination,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. This checksum is computed by the server based on the value of other fields, and may be sent only on create requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Optional. EventDataContentType specifies the type of payload in MIME format that is expected from the CloudEvent data
        /// field. This is set to `application/json` if the value is not defined.
        pub event_data_content_type: pulumi_gestalt_rust::Output<String>,
        /// Optional. User labels attached to the triggers that can be used to group resources. **Note**: This field is
        /// non-authoritative, and will only manage the labels present in your configuration. Please refer to the field
        /// `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Required. null The list of filters that applies to event attributes. Only events that match all the provided filters will be sent to the destination.
        pub matching_criterias: pulumi_gestalt_rust::Output<
            Vec<super::super::types::eventarc::TriggerMatchingCriteria>,
        >,
        /// Required. The resource name of the trigger. Must be unique within the location on the project.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. The IAM service account email associated with the trigger. The service account represents the identity of the
        /// trigger. The principal who calls this API must have `iam.serviceAccounts.actAs` permission in the service account. See
        /// https://cloud.google.com/iam/docs/understanding-service-accounts#sa_common for more information. For Cloud Run
        /// destinations, this service account is used to generate identity tokens when invoking the service. See
        /// https://cloud.google.com/run/docs/triggering/pubsub-push#create-service-account for information on how to invoke
        /// authenticated Cloud Run services. In order to create Audit Log triggers, the service account should also have
        /// `roles/eventarc.eventReceiver` IAM role.
        pub service_account: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. In order to deliver messages, Eventarc may use other GCP products as transport intermediary. This field
        /// contains a reference to that transport intermediary. This information can be used for debugging purposes.
        pub transport: pulumi_gestalt_rust::Output<
            super::super::types::eventarc::TriggerTransport,
        >,
        /// Output only. Server assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. The last-modified time.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TriggerArgs,
    ) -> TriggerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let channel_binding = args.channel.get_output(context);
        let destination_binding = args.destination.get_output(context);
        let event_data_content_type_binding = args
            .event_data_content_type
            .get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let matching_criterias_binding = args.matching_criterias.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let service_account_binding = args.service_account.get_output(context);
        let transport_binding = args.transport.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:eventarc/trigger:Trigger".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "channel".into(),
                    value: channel_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destination".into(),
                    value: destination_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventDataContentType".into(),
                    value: event_data_content_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "matchingCriterias".into(),
                    value: matching_criterias_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAccount".into(),
                    value: service_account_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transport".into(),
                    value: transport_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TriggerResult {
            channel: o.get_field("channel"),
            conditions: o.get_field("conditions"),
            create_time: o.get_field("createTime"),
            destination: o.get_field("destination"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            event_data_content_type: o.get_field("eventDataContentType"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            matching_criterias: o.get_field("matchingCriterias"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            service_account: o.get_field("serviceAccount"),
            transport: o.get_field("transport"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
