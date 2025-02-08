#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WindowsFunctionAppSlotAuthSettingsV2FacebookV2 {
    /// The App ID of the Facebook app used for login.
    #[builder(into)]
    #[serde(rename = "appId")]
    pub r#app_id: Box<String>,
    /// The app setting name that contains the `app_secret` value used for Facebook Login.
    /// 
    /// !> **NOTE:** A setting with this name must exist in `app_settings` to function correctly.
    #[builder(into)]
    #[serde(rename = "appSecretSettingName")]
    pub r#app_secret_setting_name: Box<String>,
    /// The version of the Facebook API to be used while logging in.
    #[builder(into, default)]
    #[serde(rename = "graphApiVersion")]
    pub r#graph_api_version: Box<Option<String>>,
    /// The list of scopes that should be requested as part of Facebook Login authentication.
    #[builder(into, default)]
    #[serde(rename = "loginScopes")]
    pub r#login_scopes: Box<Option<Vec<String>>>,
}
