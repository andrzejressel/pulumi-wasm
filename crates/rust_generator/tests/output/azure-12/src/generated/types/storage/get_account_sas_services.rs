#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetAccountSasServices {
    /// Should permission be granted to `blob` services within this storage account?
    #[builder(into)]
    #[serde(rename = "blob")]
    pub r#blob: Box<bool>,
    /// Should permission be granted to `file` services within this storage account?
    #[builder(into)]
    #[serde(rename = "file")]
    pub r#file: Box<bool>,
    /// Should permission be granted to `queue` services within this storage account?
    #[builder(into)]
    #[serde(rename = "queue")]
    pub r#queue: Box<bool>,
    /// Should permission be granted to `table` services within this storage account?
    #[builder(into)]
    #[serde(rename = "table")]
    pub r#table: Box<bool>,
}
