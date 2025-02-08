#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointMongodbSettings {
    /// Authentication mechanism to access the MongoDB source endpoint. Default is `default`.
    #[builder(into, default)]
    #[serde(rename = "authMechanism")]
    pub r#auth_mechanism: Box<Option<String>>,
    /// Authentication database name. Not used when `auth_type` is `no`. Default is `admin`.
    #[builder(into, default)]
    #[serde(rename = "authSource")]
    pub r#auth_source: Box<Option<String>>,
    /// Authentication type to access the MongoDB source endpoint. Default is `password`.
    #[builder(into, default)]
    #[serde(rename = "authType")]
    pub r#auth_type: Box<Option<String>>,
    /// Number of documents to preview to determine the document organization. Use this setting when `nesting_level` is set to `one`. Default is `1000`.
    #[builder(into, default)]
    #[serde(rename = "docsToInvestigate")]
    pub r#docs_to_investigate: Box<Option<String>>,
    /// Document ID. Use this setting when `nesting_level` is set to `none`. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "extractDocId")]
    pub r#extract_doc_id: Box<Option<String>>,
    /// Specifies either document or table mode. Default is `none`. Valid values are `one` (table mode) and `none` (document mode).
    #[builder(into, default)]
    #[serde(rename = "nestingLevel")]
    pub r#nesting_level: Box<Option<String>>,
}
