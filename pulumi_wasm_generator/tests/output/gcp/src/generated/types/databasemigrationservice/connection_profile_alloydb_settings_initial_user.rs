#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionProfileAlloydbSettingsInitialUser {
    /// The initial password for the user.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// (Output)
    /// Output only. Indicates if the initialUser.password field has been set.
    #[builder(into, default)]
    #[serde(rename = "passwordSet")]
    pub r#password_set: Box<Option<bool>>,
    /// The database username.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: Box<String>,
}
