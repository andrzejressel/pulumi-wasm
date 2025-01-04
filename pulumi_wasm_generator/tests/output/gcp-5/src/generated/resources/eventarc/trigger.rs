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
pub mod trigger {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerArgs {
        /// Optional. The name of the channel associated with the trigger in
        /// `projects/{project}/locations/{location}/channels/{channel}` format. You must provide a channel to receive events from
        /// Eventarc SaaS partners.
        #[builder(into, default)]
        pub channel: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. Destination specifies where the events should be sent to.
        #[builder(into)]
        pub destination: pulumi_wasm_rust::Output<
            super::super::types::eventarc::TriggerDestination,
        >,
        /// Optional. EventDataContentType specifies the type of payload in MIME format that is expected from the CloudEvent data
        /// field. This is set to `application/json` if the value is not defined.
        #[builder(into, default)]
        pub event_data_content_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. User labels attached to the triggers that can be used to group resources. **Note**: This field is
        /// non-authoritative, and will only manage the labels present in your configuration. Please refer to the field
        /// `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Required. null The list of filters that applies to event attributes. Only events that match all the provided filters will be sent to the destination.
        #[builder(into)]
        pub matching_criterias: pulumi_wasm_rust::Output<
            Vec<super::super::types::eventarc::TriggerMatchingCriteria>,
        >,
        /// Required. The resource name of the trigger. Must be unique within the location on the project.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. The IAM service account email associated with the trigger. The service account represents the identity of the
        /// trigger. The principal who calls this API must have `iam.serviceAccounts.actAs` permission in the service account. See
        /// https://cloud.google.com/iam/docs/understanding-service-accounts#sa_common for more information. For Cloud Run
        /// destinations, this service account is used to generate identity tokens when invoking the service. See
        /// https://cloud.google.com/run/docs/triggering/pubsub-push#create-service-account for information on how to invoke
        /// authenticated Cloud Run services. In order to create Audit Log triggers, the service account should also have
        /// `roles/eventarc.eventReceiver` IAM role.
        #[builder(into, default)]
        pub service_account: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. In order to deliver messages, Eventarc may use other GCP products as transport intermediary. This field
        /// contains a reference to that transport intermediary. This information can be used for debugging purposes.
        #[builder(into, default)]
        pub transport: pulumi_wasm_rust::Output<
            Option<super::super::types::eventarc::TriggerTransport>,
        >,
    }
    #[allow(dead_code)]
    pub struct TriggerResult {
        /// Optional. The name of the channel associated with the trigger in
        /// `projects/{project}/locations/{location}/channels/{channel}` format. You must provide a channel to receive events from
        /// Eventarc SaaS partners.
        pub channel: pulumi_wasm_rust::Output<Option<String>>,
        /// Output only. The reason(s) why a trigger is in FAILED state.
        pub conditions: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. The creation time.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Required. Destination specifies where the events should be sent to.
        pub destination: pulumi_wasm_rust::Output<
            super::super::types::eventarc::TriggerDestination,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. This checksum is computed by the server based on the value of other fields, and may be sent only on create requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Optional. EventDataContentType specifies the type of payload in MIME format that is expected from the CloudEvent data
        /// field. This is set to `application/json` if the value is not defined.
        pub event_data_content_type: pulumi_wasm_rust::Output<String>,
        /// Optional. User labels attached to the triggers that can be used to group resources. **Note**: This field is
        /// non-authoritative, and will only manage the labels present in your configuration. Please refer to the field
        /// `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// Required. null The list of filters that applies to event attributes. Only events that match all the provided filters will be sent to the destination.
        pub matching_criterias: pulumi_wasm_rust::Output<
            Vec<super::super::types::eventarc::TriggerMatchingCriteria>,
        >,
        /// Required. The resource name of the trigger. Must be unique within the location on the project.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. The IAM service account email associated with the trigger. The service account represents the identity of the
        /// trigger. The principal who calls this API must have `iam.serviceAccounts.actAs` permission in the service account. See
        /// https://cloud.google.com/iam/docs/understanding-service-accounts#sa_common for more information. For Cloud Run
        /// destinations, this service account is used to generate identity tokens when invoking the service. See
        /// https://cloud.google.com/run/docs/triggering/pubsub-push#create-service-account for information on how to invoke
        /// authenticated Cloud Run services. In order to create Audit Log triggers, the service account should also have
        /// `roles/eventarc.eventReceiver` IAM role.
        pub service_account: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. In order to deliver messages, Eventarc may use other GCP products as transport intermediary. This field
        /// contains a reference to that transport intermediary. This information can be used for debugging purposes.
        pub transport: pulumi_wasm_rust::Output<
            super::super::types::eventarc::TriggerTransport,
        >,
        /// Output only. Server assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. The last-modified time.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TriggerArgs) -> TriggerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let channel_binding = args.channel.get_inner();
        let destination_binding = args.destination.get_inner();
        let event_data_content_type_binding = args.event_data_content_type.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let matching_criterias_binding = args.matching_criterias.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let service_account_binding = args.service_account.get_inner();
        let transport_binding = args.transport.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:eventarc/trigger:Trigger".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "channel".into(),
                    value: &channel_binding,
                },
                register_interface::ObjectField {
                    name: "destination".into(),
                    value: &destination_binding,
                },
                register_interface::ObjectField {
                    name: "eventDataContentType".into(),
                    value: &event_data_content_type_binding,
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
                    name: "matchingCriterias".into(),
                    value: &matching_criterias_binding,
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
                    name: "serviceAccount".into(),
                    value: &service_account_binding,
                },
                register_interface::ObjectField {
                    name: "transport".into(),
                    value: &transport_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "channel".into(),
                },
                register_interface::ResultField {
                    name: "conditions".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "destination".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "eventDataContentType".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "matchingCriterias".into(),
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
                    name: "serviceAccount".into(),
                },
                register_interface::ResultField {
                    name: "transport".into(),
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
        TriggerResult {
            channel: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("channel").unwrap(),
            ),
            conditions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("conditions").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            destination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destination").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            event_data_content_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventDataContentType").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            matching_criterias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("matchingCriterias").unwrap(),
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
            service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccount").unwrap(),
            ),
            transport: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transport").unwrap(),
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
