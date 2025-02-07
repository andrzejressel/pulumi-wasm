/// A named resource to which messages are sent by publishers.
///
///
/// To get more information about Topic, see:
///
/// * [API documentation](https://cloud.google.com/pubsub/docs/reference/rest/v1/projects.topics)
/// * How-to Guides
///     * [Managing Topics](https://cloud.google.com/pubsub/docs/admin#managing_topics)
///
/// > **Note:** You can retrieve the email of the Google Managed Pub/Sub Service Account used for forwarding
/// by using the `gcp.projects.ServiceIdentity` resource.
///
/// ## Example Usage
///
/// ### Pubsub Topic Basic
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:pubsub:Topic
///     properties:
///       name: example-topic
///       labels:
///         foo: bar
///       messageRetentionDuration: 86600s
/// ```
/// ### Pubsub Topic Cmek
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cryptoKey = crypto_key::create(
///         "cryptoKey",
///         CryptoKeyArgs::builder()
///             .key_ring("${keyRing.id}")
///             .name("example-key")
///             .build_struct(),
///     );
///     let example = topic::create(
///         "example",
///         TopicArgs::builder()
///             .kms_key_name("${cryptoKey.id}")
///             .name("example-topic")
///             .build_struct(),
///     );
///     let keyRing = key_ring::create(
///         "keyRing",
///         KeyRingArgs::builder().location("global").name("example-keyring").build_struct(),
///     );
/// }
/// ```
/// ### Pubsub Topic Geo Restricted
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = topic::create(
///         "example",
///         TopicArgs::builder()
///             .message_storage_policy(
///                 TopicMessageStoragePolicy::builder()
///                     .allowedPersistenceRegions(vec!["europe-west3",])
///                     .build_struct(),
///             )
///             .name("example-topic")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Pubsub Topic Schema Settings
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = schema::create(
///         "example",
///         SchemaArgs::builder()
///             .definition(
///                 "{\n  \"type\" : \"record\",\n  \"name\" : \"Avro\",\n  \"fields\" : [\n    {\n      \"name\" : \"StringField\",\n      \"type\" : \"string\"\n    },\n    {\n      \"name\" : \"IntField\",\n      \"type\" : \"int\"\n    }\n  ]\n}\n",
///             )
///             .name("example")
///             .type_("AVRO")
///             .build_struct(),
///     );
///     let exampleTopic = topic::create(
///         "exampleTopic",
///         TopicArgs::builder()
///             .name("example-topic")
///             .schema_settings(
///                 TopicSchemaSettings::builder()
///                     .encoding("JSON")
///                     .schema("projects/my-project-name/schemas/example")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Pubsub Topic Ingestion Kinesis
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = topic::create(
///         "example",
///         TopicArgs::builder()
///             .ingestion_data_source_settings(
///                 TopicIngestionDataSourceSettings::builder()
///                     .awsKinesis(
///                         TopicIngestionDataSourceSettingsAwsKinesis::builder()
///                             .awsRoleArn("arn:aws:iam::111111111111:role/fake-role-name")
///                             .consumerArn(
///                                 "arn:aws:kinesis:us-west-2:111111111111:stream/fake-stream-name/consumer/consumer-1:1111111111",
///                             )
///                             .gcpServiceAccount(
///                                 "fake-service-account@fake-gcp-project.iam.gserviceaccount.com",
///                             )
///                             .streamArn(
///                                 "arn:aws:kinesis:us-west-2:111111111111:stream/fake-stream-name",
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("example-topic")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Pubsub Topic Ingestion Cloud Storage
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = topic::create(
///         "example",
///         TopicArgs::builder()
///             .ingestion_data_source_settings(
///                 TopicIngestionDataSourceSettings::builder()
///                     .cloudStorage(
///                         TopicIngestionDataSourceSettingsCloudStorage::builder()
///                             .bucket("test-bucket")
///                             .matchGlob("foo/**")
///                             .minimumObjectCreateTime("2024-01-01T00:00:00Z")
///                             .textFormat(
///                                 TopicIngestionDataSourceSettingsCloudStorageTextFormat::builder()
///                                     .delimiter(" ")
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .platformLogsSettings(
///                         TopicIngestionDataSourceSettingsPlatformLogsSettings::builder()
///                             .severity("WARNING")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("example-topic")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Topic can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/topics/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Topic can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:pubsub/topic:Topic default projects/{{project}}/topics/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/topic:Topic default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/topic:Topic default {{name}}
/// ```
///
pub mod topic {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicArgs {
        /// Settings for ingestion from a data source into this topic.
        /// Structure is documented below.
        #[builder(into, default)]
        pub ingestion_data_source_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pubsub::TopicIngestionDataSourceSettings>,
        >,
        /// The resource name of the Cloud KMS CryptoKey to be used to protect access
        /// to messages published on this topic. Your project's PubSub service account
        /// (`service-{{PROJECT_NUMBER}}@gcp-sa-pubsub.iam.gserviceaccount.com`) must have
        /// `roles/cloudkms.cryptoKeyEncrypterDecrypter` to use this feature.
        /// The expected format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`
        #[builder(into, default)]
        pub kms_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A set of key/value label pairs to assign to this Topic.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Indicates the minimum duration to retain a message after it is published
        /// to the topic. If this field is set, messages published to the topic in
        /// the last messageRetentionDuration are always available to subscribers.
        /// For instance, it allows any attached subscription to seek to a timestamp
        /// that is up to messageRetentionDuration in the past. If this field is not
        /// set, message retention is controlled by settings on individual subscriptions.
        /// The rotation period has the format of a decimal number, followed by the
        /// letter `s` (seconds). Cannot be more than 31 days or less than 10 minutes.
        #[builder(into, default)]
        pub message_retention_duration: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Policy constraining the set of Google Cloud Platform regions where
        /// messages published to the topic may be stored. If not present, then no
        /// constraints are in effect.
        /// Structure is documented below.
        #[builder(into, default)]
        pub message_storage_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pubsub::TopicMessageStoragePolicy>,
        >,
        /// Name of the topic.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Settings for validating messages published against a schema.
        /// Structure is documented below.
        #[builder(into, default)]
        pub schema_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pubsub::TopicSchemaSettings>,
        >,
    }
    #[allow(dead_code)]
    pub struct TopicResult {
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Settings for ingestion from a data source into this topic.
        /// Structure is documented below.
        pub ingestion_data_source_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::pubsub::TopicIngestionDataSourceSettings>,
        >,
        /// The resource name of the Cloud KMS CryptoKey to be used to protect access
        /// to messages published on this topic. Your project's PubSub service account
        /// (`service-{{PROJECT_NUMBER}}@gcp-sa-pubsub.iam.gserviceaccount.com`) must have
        /// `roles/cloudkms.cryptoKeyEncrypterDecrypter` to use this feature.
        /// The expected format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`
        pub kms_key_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A set of key/value label pairs to assign to this Topic.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Indicates the minimum duration to retain a message after it is published
        /// to the topic. If this field is set, messages published to the topic in
        /// the last messageRetentionDuration are always available to subscribers.
        /// For instance, it allows any attached subscription to seek to a timestamp
        /// that is up to messageRetentionDuration in the past. If this field is not
        /// set, message retention is controlled by settings on individual subscriptions.
        /// The rotation period has the format of a decimal number, followed by the
        /// letter `s` (seconds). Cannot be more than 31 days or less than 10 minutes.
        pub message_retention_duration: pulumi_gestalt_rust::Output<Option<String>>,
        /// Policy constraining the set of Google Cloud Platform regions where
        /// messages published to the topic may be stored. If not present, then no
        /// constraints are in effect.
        /// Structure is documented below.
        pub message_storage_policy: pulumi_gestalt_rust::Output<
            super::super::types::pubsub::TopicMessageStoragePolicy,
        >,
        /// Name of the topic.
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
        /// Settings for validating messages published against a schema.
        /// Structure is documented below.
        pub schema_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::pubsub::TopicSchemaSettings>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TopicArgs,
    ) -> TopicResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let ingestion_data_source_settings_binding = args
            .ingestion_data_source_settings
            .get_output(context)
            .get_inner();
        let kms_key_name_binding = args.kms_key_name.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let message_retention_duration_binding = args
            .message_retention_duration
            .get_output(context)
            .get_inner();
        let message_storage_policy_binding = args
            .message_storage_policy
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let schema_settings_binding = args
            .schema_settings
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:pubsub/topic:Topic".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ingestionDataSourceSettings".into(),
                    value: &ingestion_data_source_settings_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyName".into(),
                    value: &kms_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "messageRetentionDuration".into(),
                    value: &message_retention_duration_binding,
                },
                register_interface::ObjectField {
                    name: "messageStoragePolicy".into(),
                    value: &message_storage_policy_binding,
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
                    name: "schemaSettings".into(),
                    value: &schema_settings_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TopicResult {
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            ingestion_data_source_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ingestionDataSourceSettings"),
            ),
            kms_key_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyName"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            message_retention_duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("messageRetentionDuration"),
            ),
            message_storage_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("messageStoragePolicy"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            schema_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schemaSettings"),
            ),
        }
    }
}
