#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTriggerPubsubConfig {
    /// Service account that will make the push request.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Box<String>,
    /// Potential issues with the underlying Pub/Sub subscription configuration.
    /// Only populated on get requests.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
    /// Output only. Name of the subscription.
    #[builder(into)]
    #[serde(rename = "subscription")]
    pub r#subscription: Box<String>,
    /// The name of the topic from which this subscription is receiving messages.
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: Box<String>,
}
