#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserProfileUserSettingsCustomPosixUserConfig {
    /// The POSIX group ID.
    #[builder(into)]
    #[serde(rename = "gid")]
    pub r#gid: Box<i32>,
    /// The POSIX user ID.
    #[builder(into)]
    #[serde(rename = "uid")]
    pub r#uid: Box<i32>,
}