#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CrawlerLakeFormationConfiguration {
    /// Required for cross account crawls. For same account crawls as the target data, this can omitted.
    #[builder(into, default)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<Option<String>>,
    /// Specifies whether to use Lake Formation credentials for the crawler instead of the IAM role credentials.
    #[builder(into, default)]
    #[serde(rename = "useLakeFormationCredentials")]
    pub r#use_lake_formation_credentials: Box<Option<bool>>,
}