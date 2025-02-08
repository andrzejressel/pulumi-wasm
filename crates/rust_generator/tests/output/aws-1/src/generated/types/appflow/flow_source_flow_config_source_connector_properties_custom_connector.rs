#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowSourceFlowConfigSourceConnectorPropertiesCustomConnector {
    #[builder(into, default)]
    #[serde(rename = "customProperties")]
    pub r#custom_properties: Box<Option<std::collections::HashMap<String, String>>>,
    #[builder(into)]
    #[serde(rename = "entityName")]
    pub r#entity_name: Box<String>,
}
