#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApiKeyRestrictions {
    /// The Android apps that are allowed to use the key.
    #[builder(into, default)]
    #[serde(rename = "androidKeyRestrictions")]
    pub r#android_key_restrictions: Box<Option<super::super::types::projects::ApiKeyRestrictionsAndroidKeyRestrictions>>,
    /// A restriction for a specific service and optionally one or more specific methods. Requests are allowed if they match any of these restrictions. If no restrictions are specified, all targets are allowed.
    #[builder(into, default)]
    #[serde(rename = "apiTargets")]
    pub r#api_targets: Box<Option<Vec<super::super::types::projects::ApiKeyRestrictionsApiTarget>>>,
    /// The HTTP referrers (websites) that are allowed to use the key.
    #[builder(into, default)]
    #[serde(rename = "browserKeyRestrictions")]
    pub r#browser_key_restrictions: Box<Option<super::super::types::projects::ApiKeyRestrictionsBrowserKeyRestrictions>>,
    /// The iOS apps that are allowed to use the key.
    #[builder(into, default)]
    #[serde(rename = "iosKeyRestrictions")]
    pub r#ios_key_restrictions: Box<Option<super::super::types::projects::ApiKeyRestrictionsIosKeyRestrictions>>,
    /// The IP addresses of callers that are allowed to use the key.
    #[builder(into, default)]
    #[serde(rename = "serverKeyRestrictions")]
    pub r#server_key_restrictions: Box<Option<super::super::types::projects::ApiKeyRestrictionsServerKeyRestrictions>>,
}
