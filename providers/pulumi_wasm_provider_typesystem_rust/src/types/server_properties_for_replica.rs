#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ServerPropertiesForReplica {
    #[builder(into)]
    #[serde(rename = "createMode")]
    pub r#create_mode: Box<String>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
