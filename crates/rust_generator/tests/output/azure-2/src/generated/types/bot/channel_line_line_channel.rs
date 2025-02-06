#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelLineLineChannel {
    /// The access token which is used to call the Line Channel API.
    #[builder(into)]
    #[serde(rename = "accessToken")]
    pub r#access_token: Box<String>,
    /// The secret which is used to access the Line Channel.
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Box<String>,
}
