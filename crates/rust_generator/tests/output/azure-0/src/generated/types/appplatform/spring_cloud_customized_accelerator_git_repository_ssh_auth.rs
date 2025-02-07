#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudCustomizedAcceleratorGitRepositorySshAuth {
    /// Specifies the Public SSH Key of git repository basic auth.
    #[builder(into, default)]
    #[serde(rename = "hostKey")]
    pub r#host_key: Box<Option<String>>,
    /// Specifies the SSH Key algorithm of git repository basic auth.
    #[builder(into, default)]
    #[serde(rename = "hostKeyAlgorithm")]
    pub r#host_key_algorithm: Box<Option<String>>,
    /// Specifies the Private SSH Key of git repository basic auth.
    #[builder(into)]
    #[serde(rename = "privateKey")]
    pub r#private_key: Box<String>,
}
