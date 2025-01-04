#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRepositoryRemoteRepositoryConfigMavenRepositoryCustomRepository {
    /// Specific uri to the registry, e.g. '"https://repo.maven.apache.org/maven2"'
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
