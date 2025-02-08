#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetJobDefinitionEksPropertyPodPropertyContainerSecurityContext {
    /// When this parameter is true, the container is given elevated permissions on the host container instance (similar to the root user).
    #[builder(into)]
    #[serde(rename = "privileged")]
    pub r#privileged: Box<bool>,
    #[builder(into)]
    #[serde(rename = "readOnlyRootFileSystem")]
    pub r#read_only_root_file_system: Box<bool>,
    /// When this parameter is specified, the container is run as the specified group ID (gid). If this parameter isn't specified, the default is the group that's specified in the image metadata.
    #[builder(into)]
    #[serde(rename = "runAsGroup")]
    pub r#run_as_group: Box<i32>,
    /// When this parameter is specified, the container is run as a user with a uid other than 0. If this parameter isn't specified, so such rule is enforced.
    #[builder(into)]
    #[serde(rename = "runAsNonRoot")]
    pub r#run_as_non_root: Box<bool>,
    /// When this parameter is specified, the container is run as the specified user ID (uid). If this parameter isn't specified, the default is the user that's specified in the image metadata.
    #[builder(into)]
    #[serde(rename = "runAsUser")]
    pub r#run_as_user: Box<i32>,
}
