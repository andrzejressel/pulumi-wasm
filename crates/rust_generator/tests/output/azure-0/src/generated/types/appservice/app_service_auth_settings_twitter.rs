#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AppServiceAuthSettingsTwitter {
    /// The consumer key of the Twitter app used for login
    #[builder(into)]
    #[serde(rename = "consumerKey")]
    pub r#consumer_key: Box<String>,
    /// The consumer secret of the Twitter app used for login.
    #[builder(into)]
    #[serde(rename = "consumerSecret")]
    pub r#consumer_secret: Box<String>,
}
