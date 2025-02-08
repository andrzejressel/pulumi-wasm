#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelWebChatSite {
    /// Is the endpoint parameters enabled for this site?
    #[builder(into, default)]
    #[serde(rename = "endpointParametersEnabled")]
    pub r#endpoint_parameters_enabled: Box<Option<bool>>,
    /// The name of the site.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Is the storage site enabled for detailed logging? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "storageEnabled")]
    pub r#storage_enabled: Box<Option<bool>>,
    /// Is the user upload enabled for this site? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "userUploadEnabled")]
    pub r#user_upload_enabled: Box<Option<bool>>,
}
