#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccessPosixProfile {
    /// The POSIX group ID used for all EFS operations by this user.
    #[builder(into)]
    #[serde(rename = "gid")]
    pub r#gid: Box<i32>,
    /// The secondary POSIX group IDs used for all EFS operations by this user.
    #[builder(into, default)]
    #[serde(rename = "secondaryGids")]
    pub r#secondary_gids: Box<Option<Vec<i32>>>,
    /// The POSIX user ID used for all EFS operations by this user.
    #[builder(into)]
    #[serde(rename = "uid")]
    pub r#uid: Box<i32>,
}