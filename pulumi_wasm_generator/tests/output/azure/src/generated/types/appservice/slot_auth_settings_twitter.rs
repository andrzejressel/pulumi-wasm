#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SlotAuthSettingsTwitter {
    /// The consumer key of the Twitter app used for login
    #[builder(into)]
    #[serde(rename = "consumerKey")]
    pub r#consumer_key: Box<String>,
    /// The consumer secret of the Twitter app used for login.
    #[builder(into)]
    #[serde(rename = "consumerSecret")]
    pub r#consumer_secret: Box<String>,
}