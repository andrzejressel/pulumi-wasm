#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetRepositoryRemoteRepositoryConfigNpmRepositoryCustomRepository {
    /// Specific uri to the registry, e.g. '"https://registry.npmjs.org"'
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
