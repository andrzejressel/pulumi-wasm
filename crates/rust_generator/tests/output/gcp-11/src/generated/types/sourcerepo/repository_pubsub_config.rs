#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryPubsubConfig {
    /// The format of the Cloud Pub/Sub messages.
    /// - PROTOBUF: The message payload is a serialized protocol buffer of SourceRepoEvent.
    /// - JSON: The message payload is a JSON string of SourceRepoEvent.
    /// Possible values are: `PROTOBUF`, `JSON`.
    #[builder(into)]
    #[serde(rename = "messageFormat")]
    pub r#message_format: Box<String>,
    /// Email address of the service account used for publishing Cloud Pub/Sub messages.
    /// This service account needs to be in the same project as the PubsubConfig. When added,
    /// the caller needs to have iam.serviceAccounts.actAs permission on this service account.
    /// If unspecified, it defaults to the compute engine default service account.
    #[builder(into, default)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Box<Option<String>>,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: Box<String>,
}
