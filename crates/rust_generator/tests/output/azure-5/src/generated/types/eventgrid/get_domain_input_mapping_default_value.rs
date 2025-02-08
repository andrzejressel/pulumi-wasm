#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDomainInputMappingDefaultValue {
    /// Specifies the default data version of the EventGrid Event associated with the domain.
    #[builder(into)]
    #[serde(rename = "dataVersion")]
    pub r#data_version: Box<String>,
    /// Specifies the default event type of the EventGrid Event associated with the domain.
    #[builder(into)]
    #[serde(rename = "eventType")]
    pub r#event_type: Box<String>,
    /// Specifies the default subject of the EventGrid Event associated with the domain.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Box<String>,
}
