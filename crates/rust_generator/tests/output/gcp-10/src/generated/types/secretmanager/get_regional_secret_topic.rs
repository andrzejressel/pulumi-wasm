#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRegionalSecretTopic {
    /// The resource name of the Pub/Sub topic that will be published to, in the following format:
    /// projects/*/topics/*. For publication to succeed, the Secret Manager Service
    /// Agent service account must have pubsub.publisher permissions on the topic.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
