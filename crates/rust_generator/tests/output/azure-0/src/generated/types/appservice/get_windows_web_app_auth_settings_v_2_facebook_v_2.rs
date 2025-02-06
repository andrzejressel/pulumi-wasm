#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWindowsWebAppAuthSettingsV2FacebookV2 {
    /// The App ID of the Facebook app used for login.
    #[builder(into)]
    #[serde(rename = "appId")]
    pub r#app_id: Box<String>,
    /// The app setting name that contains the `app_secret` value used for Facebook login.
    #[builder(into)]
    #[serde(rename = "appSecretSettingName")]
    pub r#app_secret_setting_name: Box<String>,
    /// The version of the Facebook API to be used while logging in.
    #[builder(into)]
    #[serde(rename = "graphApiVersion")]
    pub r#graph_api_version: Box<String>,
    /// The list of Login scopes that are requested as part of Microsoft Account authentication.
    #[builder(into)]
    #[serde(rename = "loginScopes")]
    pub r#login_scopes: Box<Vec<String>>,
}
