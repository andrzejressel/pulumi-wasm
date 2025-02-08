#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobDefinitionEksPropertiesPodPropertiesContainersSecurityContext {
    #[builder(into, default)]
    #[serde(rename = "privileged")]
    pub r#privileged: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "readOnlyRootFileSystem")]
    pub r#read_only_root_file_system: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "runAsGroup")]
    pub r#run_as_group: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "runAsNonRoot")]
    pub r#run_as_non_root: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "runAsUser")]
    pub r#run_as_user: Box<Option<i32>>,
}
