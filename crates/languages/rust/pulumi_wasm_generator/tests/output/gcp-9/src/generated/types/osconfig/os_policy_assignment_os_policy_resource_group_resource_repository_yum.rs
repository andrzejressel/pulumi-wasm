#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourceRepositoryYum {
    /// The location of the repository directory.
    #[builder(into)]
    #[serde(rename = "baseUrl")]
    pub r#base_url: Box<String>,
    /// The display name of the repository.
    #[builder(into, default)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<Option<String>>,
    /// URIs of GPG keys.
    #[builder(into, default)]
    #[serde(rename = "gpgKeys")]
    pub r#gpg_keys: Box<Option<Vec<String>>>,
    /// A one word, unique name for this repository. This is the
    /// `repo id` in the yum config file and also the `display_name` if
    /// `display_name` is omitted. This id is also used as the unique identifier
    /// when checking for resource conflicts.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
