#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TopicInputMappingFields {
    /// Specifies the data version of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "dataVersion")]
    pub r#data_version: Box<Option<String>>,
    /// Specifies the event time of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "eventTime")]
    pub r#event_time: Box<Option<String>>,
    /// Specifies the event type of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "eventType")]
    pub r#event_type: Box<Option<String>>,
    /// Specifies the id of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Specifies the subject of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "subject")]
    pub r#subject: Box<Option<String>>,
    /// Specifies the topic of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "topic")]
    pub r#topic: Box<Option<String>>,
}
