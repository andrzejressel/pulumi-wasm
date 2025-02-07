#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceUserMetadata {
    /// Number of additional Developer Users to allocate to the Looker Instance.
    #[builder(into, default)]
    #[serde(rename = "additionalDeveloperUserCount")]
    pub r#additional_developer_user_count: Box<Option<i32>>,
    /// Number of additional Standard Users to allocate to the Looker Instance.
    #[builder(into, default)]
    #[serde(rename = "additionalStandardUserCount")]
    pub r#additional_standard_user_count: Box<Option<i32>>,
    /// Number of additional Viewer Users to allocate to the Looker Instance.
    #[builder(into, default)]
    #[serde(rename = "additionalViewerUserCount")]
    pub r#additional_viewer_user_count: Box<Option<i32>>,
}
