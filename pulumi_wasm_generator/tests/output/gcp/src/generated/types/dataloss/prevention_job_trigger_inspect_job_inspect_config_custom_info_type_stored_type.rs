#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobInspectConfigCustomInfoTypeStoredType {
    /// (Output)
    /// The creation timestamp of an inspectTemplate. Set by the server.
    #[builder(into, default)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<Option<String>>,
    /// Resource name of the requested StoredInfoType, for example `organizations/433245324/storedInfoTypes/432452342`
    /// or `projects/project-id/storedInfoTypes/432452342`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
