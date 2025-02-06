#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAccessPointPosixUser {
    /// Group ID
    #[builder(into)]
    #[serde(rename = "gid")]
    pub r#gid: Box<i32>,
    /// Secondary group IDs
    #[builder(into)]
    #[serde(rename = "secondaryGids")]
    pub r#secondary_gids: Box<Vec<i32>>,
    /// User Id
    #[builder(into)]
    #[serde(rename = "uid")]
    pub r#uid: Box<i32>,
}
