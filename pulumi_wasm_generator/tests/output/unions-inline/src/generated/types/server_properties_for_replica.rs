#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServerPropertiesForReplica {
    #[builder(skip)]
    #[serde(rename = "createMode")]
    r#create_mode: Box<super::constants::ConstStringReplica>,
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
