#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetFoldersFolder {
    /// The timestamp of when the folder was created
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<String>,
    /// The timestamp of when the folder was requested to be deleted (if applicable)
    #[builder(into)]
    #[serde(rename = "deleteTime")]
    pub r#delete_time: Box<String>,
    /// The display name of the folder
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    /// Entity tag identifier of the folder
    #[builder(into)]
    #[serde(rename = "etag")]
    pub r#etag: Box<String>,
    /// The id of the folder
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The parent id of the folder
    #[builder(into)]
    #[serde(rename = "parent")]
    pub r#parent: Box<String>,
    /// The lifecycle state of the folder
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
    /// The timestamp of when the folder was last modified
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<String>,
}
