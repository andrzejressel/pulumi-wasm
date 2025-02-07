#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TopicInputMappingDefaultValues {
    /// Specifies the default data version of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "dataVersion")]
    pub r#data_version: Box<Option<String>>,
    /// Specifies the default event type of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "eventType")]
    pub r#event_type: Box<Option<String>>,
    /// Specifies the default subject of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "subject")]
    pub r#subject: Box<Option<String>>,
}
