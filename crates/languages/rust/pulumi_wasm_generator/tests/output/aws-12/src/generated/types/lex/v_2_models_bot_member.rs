#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsBotMember {
    /// (Required) - Alias ID of a bot that is a member of this network of bots.
    #[builder(into)]
    #[serde(rename = "aliasId")]
    pub r#alias_id: Box<String>,
    /// (Required) - Alias name of a bot that is a member of this network of bots.
    #[builder(into)]
    #[serde(rename = "aliasName")]
    pub r#alias_name: Box<String>,
    /// (Required) - Unique ID of a bot that is a member of this network of bots.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Name of the bot. The bot name must be unique in the account that creates the bot. Type String. Length Constraints: Minimum length of 1. Maximum length of 100.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// (Required) - Version of a bot that is a member of this network of bots.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
