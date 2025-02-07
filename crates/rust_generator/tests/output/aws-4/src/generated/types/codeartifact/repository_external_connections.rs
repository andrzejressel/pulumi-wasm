#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryExternalConnections {
    /// The name of the external connection associated with a repository.
    #[builder(into)]
    #[serde(rename = "externalConnectionName")]
    pub r#external_connection_name: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "packageFormat")]
    pub r#package_format: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
