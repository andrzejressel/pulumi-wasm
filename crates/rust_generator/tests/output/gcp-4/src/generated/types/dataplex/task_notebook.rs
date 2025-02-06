#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TaskNotebook {
    /// Cloud Storage URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip.
    #[builder(into, default)]
    #[serde(rename = "archiveUris")]
    pub r#archive_uris: Box<Option<Vec<String>>>,
    /// Cloud Storage URIs of files to be placed in the working directory of each executor.
    #[builder(into, default)]
    #[serde(rename = "fileUris")]
    pub r#file_uris: Box<Option<Vec<String>>>,
    /// Infrastructure specification for the execution.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "infrastructureSpec")]
    pub r#infrastructure_spec: Box<Option<super::super::types::dataplex::TaskNotebookInfrastructureSpec>>,
    /// Path to input notebook. This can be the Cloud Storage URI of the notebook file or the path to a Notebook Content. The execution args are accessible as environment variables (TASK_key=value).
    #[builder(into)]
    #[serde(rename = "notebook")]
    pub r#notebook: Box<String>,
}
