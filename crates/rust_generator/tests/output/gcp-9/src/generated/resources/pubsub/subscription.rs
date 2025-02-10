/// A named resource representing the stream of messages from a single,
/// specific topic, to be delivered to the subscribing application.
///
///
/// To get more information about Subscription, see:
///
/// * [API documentation](https://cloud.google.com/pubsub/docs/reference/rest/v1/projects.subscriptions)
/// * How-to Guides
///     * [Managing Subscriptions](https://cloud.google.com/pubsub/docs/admin#managing_subscriptions)
///
/// > **Note:** You can retrieve the email of the Google Managed Pub/Sub Service Account used for forwarding
/// by using the `gcp.projects.ServiceIdentity` resource.
///
/// ## Example Usage
///
/// ### Pubsub Subscription Push
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:pubsub:Topic
///     properties:
///       name: example-topic
///   exampleSubscription:
///     type: gcp:pubsub:Subscription
///     name: example
///     properties:
///       name: example-subscription
///       topic: ${example.id}
///       ackDeadlineSeconds: 20
///       labels:
///         foo: bar
///       pushConfig:
///         pushEndpoint: https://example.com/push
///         attributes:
///           x-goog-version: v1
/// ```
/// ### Pubsub Subscription Pull
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:pubsub:Topic
///     properties:
///       name: example-topic
///   exampleSubscription:
///     type: gcp:pubsub:Subscription
///     name: example
///     properties:
///       name: example-subscription
///       topic: ${example.id}
///       labels:
///         foo: bar
///       messageRetentionDuration: 1200s
///       retainAckedMessages: true
///       ackDeadlineSeconds: 20
///       expirationPolicy:
///         ttl: 300000.5s
///       retryPolicy:
///         minimumBackoff: 10s
///       enableMessageOrdering: false
/// ```
/// ### Pubsub Subscription Pull Filter
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:pubsub:Topic
///     properties:
///       name: example-topic
///   exampleSubscription:
///     type: gcp:pubsub:Subscription
///     name: example
///     properties:
///       name: example-subscription
///       topic: ${example.id}
///       labels:
///         foo: bar
///       filter: |2
///             attributes.foo = "foo"
///             AND attributes.bar = "bar"
///       ackDeadlineSeconds: 20
/// ```
/// ### Pubsub Subscription Dead Letter
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = topic::create(
///         "example",
///         TopicArgs::builder().name("example-topic").build_struct(),
///     );
///     let exampleDeadLetter = topic::create(
///         "exampleDeadLetter",
///         TopicArgs::builder().name("example-topic-dead-letter").build_struct(),
///     );
///     let exampleSubscription = subscription::create(
///         "exampleSubscription",
///         SubscriptionArgs::builder()
///             .dead_letter_policy(
///                 SubscriptionDeadLetterPolicy::builder()
///                     .deadLetterTopic("${exampleDeadLetter.id}")
///                     .maxDeliveryAttempts(10)
///                     .build_struct(),
///             )
///             .name("example-subscription")
///             .topic("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Pubsub Subscription Push Bq
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:pubsub:Topic
///     properties:
///       name: example-topic
///   exampleSubscription:
///     type: gcp:pubsub:Subscription
///     name: example
///     properties:
///       name: example-subscription
///       topic: ${example.id}
///       bigqueryConfig:
///         table: ${testTable.project}.${testTable.datasetId}.${testTable.tableId}
///   test:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: example_dataset
///   testTable:
///     type: gcp:bigquery:Table
///     name: test
///     properties:
///       tableId: example_table
///       datasetId: ${test.datasetId}
///       schema: |
///         [
///           {
///             "name": "data",
///             "type": "STRING",
///             "mode": "NULLABLE",
///             "description": "The data"
///           }
///         ]
///       deletionProtection: false
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Pubsub Subscription Push Bq Table Schema
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:pubsub:Topic
///     properties:
///       name: example-topic
///   exampleSubscription:
///     type: gcp:pubsub:Subscription
///     name: example
///     properties:
///       name: example-subscription
///       topic: ${example.id}
///       bigqueryConfig:
///         table: ${testTable.project}.${testTable.datasetId}.${testTable.tableId}
///         useTableSchema: true
///   test:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: example_dataset
///   testTable:
///     type: gcp:bigquery:Table
///     name: test
///     properties:
///       tableId: example_table
///       datasetId: ${test.datasetId}
///       schema: |
///         [
///           {
///             "name": "data",
///             "type": "STRING",
///             "mode": "NULLABLE",
///             "description": "The data"
///           }
///         ]
///       deletionProtection: false
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Pubsub Subscription Push Bq Service Account
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:pubsub:Topic
///     properties:
///       name: example-topic
///   exampleSubscription:
///     type: gcp:pubsub:Subscription
///     name: example
///     properties:
///       name: example-subscription
///       topic: ${example.id}
///       bigqueryConfig:
///         table: ${testTable.project}.${testTable.datasetId}.${testTable.tableId}
///         serviceAccountEmail: ${bqWriteServiceAccount.email}
///     options:
///       dependsOn:
///         - ${bqWriteServiceAccount}
///         - ${bigqueryMetadataViewer}
///         - ${bigqueryDataEditor}
///   bqWriteServiceAccount:
///     type: gcp:serviceaccount:Account
///     name: bq_write_service_account
///     properties:
///       accountId: example-bqw
///       displayName: BQ Write Service Account
///   bigqueryMetadataViewer:
///     type: gcp:projects:IAMMember
///     name: bigquery_metadata_viewer
///     properties:
///       project: ${project.projectId}
///       role: roles/bigquery.metadataViewer
///       member: serviceAccount:${bqWriteServiceAccount.email}
///   bigqueryDataEditor:
///     type: gcp:projects:IAMMember
///     name: bigquery_data_editor
///     properties:
///       project: ${project.projectId}
///       role: roles/bigquery.dataEditor
///       member: serviceAccount:${bqWriteServiceAccount.email}
///   test:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: example_dataset
///   testTable:
///     type: gcp:bigquery:Table
///     name: test
///     properties:
///       deletionProtection: false
///       tableId: example_table
///       datasetId: ${test.datasetId}
///       schema: |
///         [
///           {
///             "name": "data",
///             "type": "STRING",
///             "mode": "NULLABLE",
///             "description": "The data"
///           }
///         ]
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Pubsub Subscription Push Cloudstorage
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:storage:Bucket
///     properties:
///       name: example-bucket
///       location: US
///       uniformBucketLevelAccess: true
///   exampleTopic:
///     type: gcp:pubsub:Topic
///     name: example
///     properties:
///       name: example-topic
///   exampleSubscription:
///     type: gcp:pubsub:Subscription
///     name: example
///     properties:
///       name: example-subscription
///       topic: ${exampleTopic.id}
///       cloudStorageConfig:
///         bucket: ${example.name}
///         filenamePrefix: pre-
///         filenameSuffix: -_33395
///         filenameDatetimeFormat: YYYY-MM-DD/hh_mm_ssZ
///         maxBytes: 1000
///         maxDuration: 300s
///         maxMessages: 1000
///     options:
///       dependsOn:
///         - ${example}
///         - ${admin}
///   admin:
///     type: gcp:storage:BucketIAMMember
///     properties:
///       bucket: ${example.name}
///       role: roles/storage.admin
///       member: serviceAccount:service-${project.number}@gcp-sa-pubsub.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Pubsub Subscription Push Cloudstorage Avro
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:storage:Bucket
///     properties:
///       name: example-bucket
///       location: US
///       uniformBucketLevelAccess: true
///   exampleTopic:
///     type: gcp:pubsub:Topic
///     name: example
///     properties:
///       name: example-topic
///   exampleSubscription:
///     type: gcp:pubsub:Subscription
///     name: example
///     properties:
///       name: example-subscription
///       topic: ${exampleTopic.id}
///       cloudStorageConfig:
///         bucket: ${example.name}
///         filenamePrefix: pre-
///         filenameSuffix: -_76044
///         filenameDatetimeFormat: YYYY-MM-DD/hh_mm_ssZ
///         maxBytes: 1000
///         maxDuration: 300s
///         maxMessages: 1000
///         avroConfig:
///           writeMetadata: true
///           useTopicSchema: true
///     options:
///       dependsOn:
///         - ${example}
///         - ${admin}
///   admin:
///     type: gcp:storage:BucketIAMMember
///     properties:
///       bucket: ${example.name}
///       role: roles/storage.admin
///       member: serviceAccount:service-${project.number}@gcp-sa-pubsub.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Pubsub Subscription Push Cloudstorage Service Account
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:storage:Bucket
///     properties:
///       name: example-bucket
///       location: US
///       uniformBucketLevelAccess: true
///   exampleTopic:
///     type: gcp:pubsub:Topic
///     name: example
///     properties:
///       name: example-topic
///   exampleSubscription:
///     type: gcp:pubsub:Subscription
///     name: example
///     properties:
///       name: example-subscription
///       topic: ${exampleTopic.id}
///       cloudStorageConfig:
///         bucket: ${example.name}
///         filenamePrefix: pre-
///         filenameSuffix: -_69391
///         filenameDatetimeFormat: YYYY-MM-DD/hh_mm_ssZ
///         maxBytes: 1000
///         maxDuration: 300s
///         serviceAccountEmail: ${storageWriteServiceAccount.email}
///     options:
///       dependsOn:
///         - ${storageWriteServiceAccount}
///         - ${example}
///         - ${admin}
///   storageWriteServiceAccount:
///     type: gcp:serviceaccount:Account
///     name: storage_write_service_account
///     properties:
///       accountId: example-stw
///       displayName: Storage Write Service Account
///   admin:
///     type: gcp:storage:BucketIAMMember
///     properties:
///       bucket: ${example.name}
///       role: roles/storage.admin
///       member: serviceAccount:${storageWriteServiceAccount.email}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Subscription can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/subscriptions/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Subscription can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:pubsub/subscription:Subscription default projects/{{project}}/subscriptions/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/subscription:Subscription default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/subscription:Subscription default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionArgs {
        /// This value is the maximum time after a subscriber receives a message
        /// before the subscriber should acknowledge the message. After message
        /// delivery but before the ack deadline expires and before the message is
        /// acknowledged, it is an outstanding message and will not be delivered
        /// again during that time (on a best-effort basis).
        /// For pull subscriptions, this value is used as the initial value for
        /// the ack deadline. To override this value for a given message, call
        /// subscriptions.modifyAckDeadline with the corresponding ackId if using
        /// pull. The minimum custom deadline you can specify is 10 seconds. The
        /// maximum custom deadline you can specify is 600 seconds (10 minutes).
        /// If this parameter is 0, a default value of 10 seconds is used.
        /// For push delivery, this value is also used to set the request timeout
        /// for the call to the push endpoint.
        /// If the subscriber never acknowledges the message, the Pub/Sub system
        /// will eventually redeliver the message.
        #[builder(into, default)]
        pub ack_deadline_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// If delivery to BigQuery is used with this subscription, this field is used to configure it.
        /// Either pushConfig, bigQueryConfig or cloudStorageConfig can be set, but not combined.
        /// If all three are empty, then the subscriber will pull and ack messages using API methods.
        /// Structure is documented below.
        #[builder(into, default)]
        pub bigquery_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pubsub::SubscriptionBigqueryConfig>,
        >,
        /// If delivery to Cloud Storage is used with this subscription, this field is used to configure it.
        /// Either pushConfig, bigQueryConfig or cloudStorageConfig can be set, but not combined.
        /// If all three are empty, then the subscriber will pull and ack messages using API methods.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cloud_storage_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pubsub::SubscriptionCloudStorageConfig>,
        >,
        /// A policy that specifies the conditions for dead lettering messages in
        /// this subscription. If dead_letter_policy is not set, dead lettering
        /// is disabled.
        /// The Cloud Pub/Sub service account associated with this subscription's
        /// parent project (i.e.,
        /// service-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com) must have
        /// permission to Acknowledge() messages on this subscription.
        /// Structure is documented below.
        #[builder(into, default)]
        pub dead_letter_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pubsub::SubscriptionDeadLetterPolicy>,
        >,
        /// If `true`, Pub/Sub provides the following guarantees for the delivery
        /// of a message with a given value of messageId on this Subscriptions':
        /// - The message sent to a subscriber is guaranteed not to be resent before the message's acknowledgement deadline expires.
        /// - An acknowledged message will not be resent to a subscriber.
        /// Note that subscribers may still receive multiple copies of a message when `enable_exactly_once_delivery`
        /// is true if the message was published multiple times by a publisher client. These copies are considered distinct by Pub/Sub and have distinct messageId values
        #[builder(into, default)]
        pub enable_exactly_once_delivery: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// If `true`, messages published with the same orderingKey in PubsubMessage will be delivered to
        /// the subscribers in the order in which they are received by the Pub/Sub system. Otherwise, they
        /// may be delivered in any order.
        #[builder(into, default)]
        pub enable_message_ordering: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A policy that specifies the conditions for this subscription's expiration.
        /// A subscription is considered active as long as any connected subscriber
        /// is successfully consuming messages from the subscription or is issuing
        /// operations on the subscription. If expirationPolicy is not set, a default
        /// policy with ttl of 31 days will be used.  If it is set but ttl is "", the
        /// resource never expires.  The minimum allowed value for expirationPolicy.ttl
        /// is 1 day.
        /// Structure is documented below.
        #[builder(into, default)]
        pub expiration_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pubsub::SubscriptionExpirationPolicy>,
        >,
        /// The subscription only delivers the messages that match the filter.
        /// Pub/Sub automatically acknowledges the messages that don't match the filter. You can filter messages
        /// by their attributes. The maximum length of a filter is 256 bytes. After creating the subscription,
        /// you can't modify the filter.
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A set of key/value label pairs to assign to this Subscription.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// How long to retain unacknowledged messages in the subscription's
        /// backlog, from the moment a message is published. If
        /// retain_acked_messages is true, then this also configures the retention
        /// of acknowledged messages, and thus configures how far back in time a
        /// subscriptions.seek can be done. Defaults to 7 days. Cannot be more
        /// than 31 days (`"2678400s"`) or less than 10 minutes (`"600s"`).
        /// A duration in seconds with up to nine fractional digits, terminated
        /// by 's'. Example: `"600.5s"`.
        #[builder(into, default)]
        pub message_retention_duration: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Name of the subscription.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If push delivery is used with this subscription, this field is used to
        /// configure it. An empty pushConfig signifies that the subscriber will
        /// pull and ack messages using API methods.
        /// Structure is documented below.
        #[builder(into, default)]
        pub push_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pubsub::SubscriptionPushConfig>,
        >,
        /// Indicates whether to retain acknowledged messages. If `true`, then
        /// messages are not expunged from the subscription's backlog, even if
        /// they are acknowledged, until they fall out of the
        /// messageRetentionDuration window.
        #[builder(into, default)]
        pub retain_acked_messages: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A policy that specifies how Pub/Sub retries message delivery for this subscription.
        /// If not set, the default retry policy is applied. This generally implies that messages will be retried as soon as possible for healthy subscribers.
        /// RetryPolicy will be triggered on NACKs or acknowledgement deadline exceeded events for a given message
        /// Structure is documented below.
        #[builder(into, default)]
        pub retry_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pubsub::SubscriptionRetryPolicy>,
        >,
        /// A reference to a Topic resource, of the form projects/{project}/topics/{{name}}
        /// (as in the id property of a google_pubsub_topic), or just a topic name if
        /// the topic is in the same project as the subscription.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub topic: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionResult {
        /// This value is the maximum time after a subscriber receives a message
        /// before the subscriber should acknowledge the message. After message
        /// delivery but before the ack deadline expires and before the message is
        /// acknowledged, it is an outstanding message and will not be delivered
        /// again during that time (on a best-effort basis).
        /// For pull subscriptions, this value is used as the initial value for
        /// the ack deadline. To override this value for a given message, call
        /// subscriptions.modifyAckDeadline with the corresponding ackId if using
        /// pull. The minimum custom deadline you can specify is 10 seconds. The
        /// maximum custom deadline you can specify is 600 seconds (10 minutes).
        /// If this parameter is 0, a default value of 10 seconds is used.
        /// For push delivery, this value is also used to set the request timeout
        /// for the call to the push endpoint.
        /// If the subscriber never acknowledges the message, the Pub/Sub system
        /// will eventually redeliver the message.
        pub ack_deadline_seconds: pulumi_gestalt_rust::Output<i32>,
        /// If delivery to BigQuery is used with this subscription, this field is used to configure it.
        /// Either pushConfig, bigQueryConfig or cloudStorageConfig can be set, but not combined.
        /// If all three are empty, then the subscriber will pull and ack messages using API methods.
        /// Structure is documented below.
        pub bigquery_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::pubsub::SubscriptionBigqueryConfig>,
        >,
        /// If delivery to Cloud Storage is used with this subscription, this field is used to configure it.
        /// Either pushConfig, bigQueryConfig or cloudStorageConfig can be set, but not combined.
        /// If all three are empty, then the subscriber will pull and ack messages using API methods.
        /// Structure is documented below.
        pub cloud_storage_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::pubsub::SubscriptionCloudStorageConfig>,
        >,
        /// A policy that specifies the conditions for dead lettering messages in
        /// this subscription. If dead_letter_policy is not set, dead lettering
        /// is disabled.
        /// The Cloud Pub/Sub service account associated with this subscription's
        /// parent project (i.e.,
        /// service-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com) must have
        /// permission to Acknowledge() messages on this subscription.
        /// Structure is documented below.
        pub dead_letter_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::pubsub::SubscriptionDeadLetterPolicy>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If `true`, Pub/Sub provides the following guarantees for the delivery
        /// of a message with a given value of messageId on this Subscriptions':
        /// - The message sent to a subscriber is guaranteed not to be resent before the message's acknowledgement deadline expires.
        /// - An acknowledged message will not be resent to a subscriber.
        /// Note that subscribers may still receive multiple copies of a message when `enable_exactly_once_delivery`
        /// is true if the message was published multiple times by a publisher client. These copies are considered distinct by Pub/Sub and have distinct messageId values
        pub enable_exactly_once_delivery: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If `true`, messages published with the same orderingKey in PubsubMessage will be delivered to
        /// the subscribers in the order in which they are received by the Pub/Sub system. Otherwise, they
        /// may be delivered in any order.
        pub enable_message_ordering: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A policy that specifies the conditions for this subscription's expiration.
        /// A subscription is considered active as long as any connected subscriber
        /// is successfully consuming messages from the subscription or is issuing
        /// operations on the subscription. If expirationPolicy is not set, a default
        /// policy with ttl of 31 days will be used.  If it is set but ttl is "", the
        /// resource never expires.  The minimum allowed value for expirationPolicy.ttl
        /// is 1 day.
        /// Structure is documented below.
        pub expiration_policy: pulumi_gestalt_rust::Output<
            super::super::types::pubsub::SubscriptionExpirationPolicy,
        >,
        /// The subscription only delivers the messages that match the filter.
        /// Pub/Sub automatically acknowledges the messages that don't match the filter. You can filter messages
        /// by their attributes. The maximum length of a filter is 256 bytes. After creating the subscription,
        /// you can't modify the filter.
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// A set of key/value label pairs to assign to this Subscription.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// How long to retain unacknowledged messages in the subscription's
        /// backlog, from the moment a message is published. If
        /// retain_acked_messages is true, then this also configures the retention
        /// of acknowledged messages, and thus configures how far back in time a
        /// subscriptions.seek can be done. Defaults to 7 days. Cannot be more
        /// than 31 days (`"2678400s"`) or less than 10 minutes (`"600s"`).
        /// A duration in seconds with up to nine fractional digits, terminated
        /// by 's'. Example: `"600.5s"`.
        pub message_retention_duration: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the subscription.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If push delivery is used with this subscription, this field is used to
        /// configure it. An empty pushConfig signifies that the subscriber will
        /// pull and ack messages using API methods.
        /// Structure is documented below.
        pub push_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::pubsub::SubscriptionPushConfig>,
        >,
        /// Indicates whether to retain acknowledged messages. If `true`, then
        /// messages are not expunged from the subscription's backlog, even if
        /// they are acknowledged, until they fall out of the
        /// messageRetentionDuration window.
        pub retain_acked_messages: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A policy that specifies how Pub/Sub retries message delivery for this subscription.
        /// If not set, the default retry policy is applied. This generally implies that messages will be retried as soon as possible for healthy subscribers.
        /// RetryPolicy will be triggered on NACKs or acknowledgement deadline exceeded events for a given message
        /// Structure is documented below.
        pub retry_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::pubsub::SubscriptionRetryPolicy>,
        >,
        /// A reference to a Topic resource, of the form projects/{project}/topics/{{name}}
        /// (as in the id property of a google_pubsub_topic), or just a topic name if
        /// the topic is in the same project as the subscription.
        ///
        ///
        /// - - -
        pub topic: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SubscriptionArgs,
    ) -> SubscriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let ack_deadline_seconds_binding = args.ack_deadline_seconds.get_output(context);
        let bigquery_config_binding = args.bigquery_config.get_output(context);
        let cloud_storage_config_binding = args.cloud_storage_config.get_output(context);
        let dead_letter_policy_binding = args.dead_letter_policy.get_output(context);
        let enable_exactly_once_delivery_binding = args
            .enable_exactly_once_delivery
            .get_output(context);
        let enable_message_ordering_binding = args
            .enable_message_ordering
            .get_output(context);
        let expiration_policy_binding = args.expiration_policy.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let message_retention_duration_binding = args
            .message_retention_duration
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let push_config_binding = args.push_config.get_output(context);
        let retain_acked_messages_binding = args
            .retain_acked_messages
            .get_output(context);
        let retry_policy_binding = args.retry_policy.get_output(context);
        let topic_binding = args.topic.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:pubsub/subscription:Subscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ackDeadlineSeconds".into(),
                    value: ack_deadline_seconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bigqueryConfig".into(),
                    value: bigquery_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudStorageConfig".into(),
                    value: cloud_storage_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deadLetterPolicy".into(),
                    value: dead_letter_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableExactlyOnceDelivery".into(),
                    value: enable_exactly_once_delivery_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableMessageOrdering".into(),
                    value: enable_message_ordering_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expirationPolicy".into(),
                    value: expiration_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "messageRetentionDuration".into(),
                    value: message_retention_duration_binding.get_id(),
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
                    name: "pushConfig".into(),
                    value: push_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retainAckedMessages".into(),
                    value: retain_acked_messages_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retryPolicy".into(),
                    value: retry_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "topic".into(),
                    value: topic_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SubscriptionResult {
            ack_deadline_seconds: o.get_field("ackDeadlineSeconds"),
            bigquery_config: o.get_field("bigqueryConfig"),
            cloud_storage_config: o.get_field("cloudStorageConfig"),
            dead_letter_policy: o.get_field("deadLetterPolicy"),
            effective_labels: o.get_field("effectiveLabels"),
            enable_exactly_once_delivery: o.get_field("enableExactlyOnceDelivery"),
            enable_message_ordering: o.get_field("enableMessageOrdering"),
            expiration_policy: o.get_field("expirationPolicy"),
            filter: o.get_field("filter"),
            labels: o.get_field("labels"),
            message_retention_duration: o.get_field("messageRetentionDuration"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            push_config: o.get_field("pushConfig"),
            retain_acked_messages: o.get_field("retainAckedMessages"),
            retry_policy: o.get_field("retryPolicy"),
            topic: o.get_field("topic"),
        }
    }
}
