#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceOpensearchserviceConfig {
    /// HTTP endpoint of the OpenSearch domain.
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: Box<String>,
    /// AWS region of the OpenSearch domain. Defaults to current region.
    #[builder(into, default)]
    #[serde(rename = "region")]
    pub r#region: Box<Option<String>>,
}
