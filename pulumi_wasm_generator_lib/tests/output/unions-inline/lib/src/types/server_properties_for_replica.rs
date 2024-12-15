#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServerPropertiesForReplica {
    #[builder(into, default)]
    #[serde(rename = "createMode")]
    pub r#create_mode: Box<crate::ConstString_Replica>,
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
