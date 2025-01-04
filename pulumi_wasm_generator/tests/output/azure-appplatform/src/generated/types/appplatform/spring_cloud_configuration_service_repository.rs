#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudConfigurationServiceRepository {
    /// Specifies the ID of the Certificate Authority used when retrieving the Git Repository via HTTPS.
    #[builder(into, default)]
    #[serde(rename = "caCertificateId")]
    pub r#ca_certificate_id: Box<Option<String>>,
    /// Specifies the SSH public key of git repository.
    #[builder(into, default)]
    #[serde(rename = "hostKey")]
    pub r#host_key: Box<Option<String>>,
    /// Specifies the SSH key algorithm of git repository.
    #[builder(into, default)]
    #[serde(rename = "hostKeyAlgorithm")]
    pub r#host_key_algorithm: Box<Option<String>>,
    /// Specifies the label of the repository.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// Specifies the name which should be used for this repository.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the password of git repository basic auth.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// Specifies the collection of patterns of the repository.
    #[builder(into)]
    #[serde(rename = "patterns")]
    pub r#patterns: Box<Vec<String>>,
    /// Specifies the SSH private key of git repository.
    #[builder(into, default)]
    #[serde(rename = "privateKey")]
    pub r#private_key: Box<Option<String>>,
    /// Specifies a list of searching path of the repository
    #[builder(into, default)]
    #[serde(rename = "searchPaths")]
    pub r#search_paths: Box<Option<Vec<String>>>,
    /// Specifies whether enable the strict host key checking.
    #[builder(into, default)]
    #[serde(rename = "strictHostKeyChecking")]
    pub r#strict_host_key_checking: Box<Option<bool>>,
    /// Specifies the URI of the repository.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
    /// Specifies the username of git repository basic auth.
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
