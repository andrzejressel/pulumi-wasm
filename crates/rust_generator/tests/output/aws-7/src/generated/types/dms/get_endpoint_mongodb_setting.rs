#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetEndpointMongodbSetting {
    #[builder(into)]
    #[serde(rename = "authMechanism")]
    pub r#auth_mechanism: Box<String>,
    #[builder(into)]
    #[serde(rename = "authSource")]
    pub r#auth_source: Box<String>,
    #[builder(into)]
    #[serde(rename = "authType")]
    pub r#auth_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "docsToInvestigate")]
    pub r#docs_to_investigate: Box<String>,
    #[builder(into)]
    #[serde(rename = "extractDocId")]
    pub r#extract_doc_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "nestingLevel")]
    pub r#nesting_level: Box<String>,
}
