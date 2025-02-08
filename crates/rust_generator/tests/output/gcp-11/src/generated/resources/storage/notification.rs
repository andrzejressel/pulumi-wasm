/// Creates a new notification configuration on a specified bucket, establishing a flow of event notifications from GCS to a Cloud Pub/Sub topic.
///  For more information see
/// [the official documentation](https://cloud.google.com/storage/docs/pubsub-notifications)
/// and
/// [API](https://cloud.google.com/storage/docs/json_api/v1/notifications).
///
/// In order to enable notifications, a special Google Cloud Storage service account unique to the project
/// must exist and have the IAM permission "projects.topics.publish" for a Cloud Pub/Sub topic in the project.
/// This service account is not created automatically when a project is created.
/// To ensure the service account exists and obtain its email address for use in granting the correct IAM permission, use the
/// [`gcp.storage.getProjectServiceAccount`](https://www.terraform.io/docs/providers/google/d/storage_project_service_account.html)
/// datasource's `email_address` value, and see below for an example of enabling notifications by granting the correct IAM permission.
/// See [the notifications documentation](https://cloud.google.com/storage/docs/gsutil/commands/notification) for more details.
///
/// >**NOTE**: This resource can affect your storage IAM policy. If you are using this in the same config as your storage IAM policy resources, consider
/// making this resource dependent on those IAM resources via `depends_on`. This will safeguard against errors due to IAM race conditions.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   notification:
///     type: gcp:storage:Notification
///     properties:
///       bucket: ${bucket.name}
///       payloadFormat: JSON_API_V1
///       topic: ${topic.id}
///       eventTypes:
///         - OBJECT_FINALIZE
///         - OBJECT_METADATA_UPDATE
///       customAttributes:
///         new-attribute: new-attribute-value
///     options:
///       dependsOn:
///         - ${binding}
///   binding:
///     type: gcp:pubsub:TopicIAMBinding
///     properties:
///       topic: ${topic.id}
///       role: roles/pubsub.publisher
///       members:
///         - serviceAccount:${gcsAccount.emailAddress}
///   # End enabling notifications
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: default_bucket
///       location: US
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: default_topic
/// variables:
///   # Enable notifications by giving the correct IAM permission to the unique service account.
///   gcsAccount:
///     fn::invoke:
///       function: gcp:storage:getProjectServiceAccount
///       arguments: {}
/// ```
///
/// ## Import
///
/// Storage notifications can be imported using any of these accepted formats:
///
/// * `{{bucket_name}}/notificationConfigs/{{id}}`
///
/// When using the `pulumi import` command, Storage notifications can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:storage/notification:Notification default {{bucket_name}}/notificationConfigs/{{id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod notification {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotificationArgs {
        /// The name of the bucket.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A set of key/value attribute pairs to attach to each Cloud PubSub message published for this notification subscription
        #[builder(into, default)]
        pub custom_attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of event type filters for this notification config. If not specified, Cloud Storage will send notifications for all event types. The valid types are: `"OBJECT_FINALIZE"`, `"OBJECT_METADATA_UPDATE"`, `"OBJECT_DELETE"`, `"OBJECT_ARCHIVE"`
        #[builder(into, default)]
        pub event_types: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies a prefix path filter for this notification config. Cloud Storage will only send notifications for objects in this bucket whose names begin with the specified prefix.
        #[builder(into, default)]
        pub object_name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The desired content of the Payload. One of `"JSON_API_V1"` or `"NONE"`.
        #[builder(into)]
        pub payload_format: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Cloud PubSub topic to which this subscription publishes. Expects either the
        /// topic name, assumed to belong to the default GCP provider project, or the project-level name,
        /// i.e. `projects/my-gcp-project/topics/my-topic` or `my-topic`. If the project is not set in the provider,
        /// you will need to use the project-level name.
        ///
        /// - - -
        #[builder(into)]
        pub topic: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NotificationResult {
        /// The name of the bucket.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// A set of key/value attribute pairs to attach to each Cloud PubSub message published for this notification subscription
        pub custom_attributes: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of event type filters for this notification config. If not specified, Cloud Storage will send notifications for all event types. The valid types are: `"OBJECT_FINALIZE"`, `"OBJECT_METADATA_UPDATE"`, `"OBJECT_DELETE"`, `"OBJECT_ARCHIVE"`
        pub event_types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of the created notification.
        pub notification_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies a prefix path filter for this notification config. Cloud Storage will only send notifications for objects in this bucket whose names begin with the specified prefix.
        pub object_name_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// The desired content of the Payload. One of `"JSON_API_V1"` or `"NONE"`.
        pub payload_format: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The Cloud PubSub topic to which this subscription publishes. Expects either the
        /// topic name, assumed to belong to the default GCP provider project, or the project-level name,
        /// i.e. `projects/my-gcp-project/topics/my-topic` or `my-topic`. If the project is not set in the provider,
        /// you will need to use the project-level name.
        ///
        /// - - -
        pub topic: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NotificationArgs,
    ) -> NotificationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let custom_attributes_binding = args
            .custom_attributes
            .get_output(context)
            .get_inner();
        let event_types_binding = args.event_types.get_output(context).get_inner();
        let object_name_prefix_binding = args
            .object_name_prefix
            .get_output(context)
            .get_inner();
        let payload_format_binding = args.payload_format.get_output(context).get_inner();
        let topic_binding = args.topic.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:storage/notification:Notification".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "customAttributes".into(),
                    value: &custom_attributes_binding,
                },
                register_interface::ObjectField {
                    name: "eventTypes".into(),
                    value: &event_types_binding,
                },
                register_interface::ObjectField {
                    name: "objectNamePrefix".into(),
                    value: &object_name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "payloadFormat".into(),
                    value: &payload_format_binding,
                },
                register_interface::ObjectField {
                    name: "topic".into(),
                    value: &topic_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NotificationResult {
            bucket: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucket"),
            ),
            custom_attributes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customAttributes"),
            ),
            event_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventTypes"),
            ),
            notification_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notificationId"),
            ),
            object_name_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("objectNamePrefix"),
            ),
            payload_format: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("payloadFormat"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            topic: pulumi_gestalt_rust::__private::into_domain(o.extract_field("topic")),
        }
    }
}
