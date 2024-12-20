#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServerPropertiesForRestore {
    #[builder(skip)]
    #[serde(rename = "createMode")]
    r#create_mode: Box<crate::ConstStringPointInTimeRestore>,
    #[builder(into)]
    #[serde(rename = "restorePointInTime")]
    pub r#restore_point_in_time: Box<String>,
}
