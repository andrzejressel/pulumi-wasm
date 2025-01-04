#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudCustomizedAcceleratorGitRepository {
    /// A `basic_auth` block as defined below. Conflicts with `git_repository[0].ssh_auth`. Changing this forces a new Spring Cloud Customized Accelerator to be created.
    #[builder(into, default)]
    #[serde(rename = "basicAuth")]
    pub r#basic_auth: Box<Option<super::super::types::appplatform::SpringCloudCustomizedAcceleratorGitRepositoryBasicAuth>>,
    /// Specifies the Git repository branch to be used.
    #[builder(into, default)]
    #[serde(rename = "branch")]
    pub r#branch: Box<Option<String>>,
    /// Specifies the ID of the CA Spring Cloud Certificate for https URL of Git repository.
    #[builder(into, default)]
    #[serde(rename = "caCertificateId")]
    pub r#ca_certificate_id: Box<Option<String>>,
    /// Specifies the Git repository commit to be used.
    #[builder(into, default)]
    #[serde(rename = "commit")]
    pub r#commit: Box<Option<String>>,
    /// Specifies the Git repository tag to be used.
    #[builder(into, default)]
    #[serde(rename = "gitTag")]
    pub r#git_tag: Box<Option<String>>,
    /// Specifies the interval for checking for updates to Git or image repository. It should be greater than 10.
    #[builder(into, default)]
    #[serde(rename = "intervalInSeconds")]
    pub r#interval_in_seconds: Box<Option<i32>>,
    /// Specifies the path under the git repository to be treated as the root directory of the accelerator or the fragment (depending on `accelerator_type`).
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// A `ssh_auth` block as defined below. Conflicts with `git_repository[0].basic_auth`. Changing this forces a new Spring Cloud Customized Accelerator to be created.
    #[builder(into, default)]
    #[serde(rename = "sshAuth")]
    pub r#ssh_auth: Box<Option<super::super::types::appplatform::SpringCloudCustomizedAcceleratorGitRepositorySshAuth>>,
    /// Specifies Git repository URL for the accelerator.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
