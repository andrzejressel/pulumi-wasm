#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTaskSpecContainerSpecPrivilegesCredentialSpec {
    /// Load credential spec from this file
    #[builder(into, default)]
    #[serde(rename = "file")]
    pub r#file: Box<Option<String>>,
    /// Load credential spec from this value in the Windows registry
    #[builder(into, default)]
    #[serde(rename = "registry")]
    pub r#registry: Box<Option<String>>,
}
