#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RegistryTaskBaseImageTrigger {
    /// Should the trigger be enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The name which should be used for this trigger.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The type of the trigger. Possible values are `All` and `Runtime`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The endpoint URL for receiving the trigger.
    #[builder(into, default)]
    #[serde(rename = "updateTriggerEndpoint")]
    pub r#update_trigger_endpoint: Box<Option<String>>,
    /// Type of payload body for the trigger. Possible values are `Default` and `Token`.
    #[builder(into, default)]
    #[serde(rename = "updateTriggerPayloadType")]
    pub r#update_trigger_payload_type: Box<Option<String>>,
}
