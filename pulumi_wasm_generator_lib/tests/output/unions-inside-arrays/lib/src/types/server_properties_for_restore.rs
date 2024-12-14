#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServerPropertiesForRestore {
    #[builder(into)]
    #[serde(rename = "createMode")]
    pub r#create_mode: Box<String>,
    #[builder(into)]
    #[serde(rename = "restorePointInTime")]
    pub r#restore_point_in_time: Box<String>,
}
