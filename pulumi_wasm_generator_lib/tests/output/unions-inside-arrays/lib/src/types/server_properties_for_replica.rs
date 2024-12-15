#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServerPropertiesForReplica {
    #[builder(into)]
    #[serde(rename = "createMode")]
    pub r#create_mode: Box<crate::__ConstString_Replica>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
