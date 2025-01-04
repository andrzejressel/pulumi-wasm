#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StandardAppVersionBasicScaling {
    /// Duration of time after the last request that an instance must wait before the instance is shut down.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s". Defaults to 900s.
    #[builder(into, default)]
    #[serde(rename = "idleTimeout")]
    pub r#idle_timeout: Box<Option<String>>,
    /// Maximum number of instances to create for this version. Must be in the range [1.0, 200.0].
    #[builder(into)]
    #[serde(rename = "maxInstances")]
    pub r#max_instances: Box<i32>,
}
