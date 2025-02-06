#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CatalogCatalogGithub {
    #[builder(into)]
    #[serde(rename = "branch")]
    pub r#branch: Box<String>,
    #[builder(into)]
    #[serde(rename = "keyVaultKeyUrl")]
    pub r#key_vault_key_url: Box<String>,
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
