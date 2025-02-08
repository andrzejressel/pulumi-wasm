#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AccountRouting {
    /// Specifies the kind of network routing opted by the user. Possible values are `InternetRouting` and `MicrosoftRouting`. Defaults to `MicrosoftRouting`.
    #[builder(into, default)]
    #[serde(rename = "choice")]
    pub r#choice: Box<Option<String>>,
    /// Should internet routing storage endpoints be published? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "publishInternetEndpoints")]
    pub r#publish_internet_endpoints: Box<Option<bool>>,
    /// Should Microsoft routing storage endpoints be published? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "publishMicrosoftEndpoints")]
    pub r#publish_microsoft_endpoints: Box<Option<bool>>,
}
