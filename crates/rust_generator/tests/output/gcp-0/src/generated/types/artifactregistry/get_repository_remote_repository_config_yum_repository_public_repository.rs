#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRepositoryRemoteRepositoryConfigYumRepositoryPublicRepository {
    /// A common public repository base for Yum. Possible values: ["CENTOS", "CENTOS_DEBUG", "CENTOS_VAULT", "CENTOS_STREAM", "ROCKY", "EPEL"]
    #[builder(into)]
    #[serde(rename = "repositoryBase")]
    pub r#repository_base: Box<String>,
    /// Specific repository from the base, e.g. '"pub/rocky/9/BaseOS/x86_64/os"'
    #[builder(into)]
    #[serde(rename = "repositoryPath")]
    pub r#repository_path: Box<String>,
}
