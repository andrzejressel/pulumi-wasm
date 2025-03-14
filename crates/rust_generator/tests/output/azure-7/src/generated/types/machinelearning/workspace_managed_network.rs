#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkspaceManagedNetwork {
    /// The isolation mode of the Machine Learning Workspace. Possible values are `Disabled`, `AllowOnlyApprovedOutbound`, and `AllowInternetOutbound`
    #[builder(into, default)]
    #[serde(rename = "isolationMode")]
    pub r#isolation_mode: Box<Option<String>>,
}
