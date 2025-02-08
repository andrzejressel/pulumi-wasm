#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SubscriberSourceCustomLogSourceResource {
    /// The attributes of the third-party custom source. See `attributes` Block below.
    #[builder(into, default)]
    #[serde(rename = "attributes")]
    pub r#attributes: Box<Option<Vec<super::super::types::securitylake::SubscriberSourceCustomLogSourceResourceAttribute>>>,
    /// The details of the log provider for the third-party custom source. See `provider` Block below.
    #[builder(into, default)]
    #[serde(rename = "providers")]
    pub r#providers: Box<Option<Vec<super::super::types::securitylake::SubscriberSourceCustomLogSourceResourceProvider>>>,
    /// The name for a third-party custom source. This must be a Regionally unique value.
    #[builder(into)]
    #[serde(rename = "sourceName")]
    pub r#source_name: Box<String>,
    /// The version for a third-party custom source. This must be a Regionally unique value.
    #[builder(into, default)]
    #[serde(rename = "sourceVersion")]
    pub r#source_version: Box<Option<String>>,
}
