#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FunctionAppSlotSiteConfigCors {
    /// A list of origins which should be able to make cross-origin calls. `*` can be used to allow all calls.
    #[builder(into)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Box<Vec<String>>,
    /// Are credentials supported?
    #[builder(into, default)]
    #[serde(rename = "supportCredentials")]
    pub r#support_credentials: Box<Option<bool>>,
}
