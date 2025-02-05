#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceGroupManagerVersionTargetSize {
    /// , The number of instances which are managed for this version. Conflicts with `percent`.
    #[builder(into, default)]
    #[serde(rename = "fixed")]
    pub r#fixed: Box<Option<i32>>,
    /// , The number of instances (calculated as percentage) which are managed for this version. Conflicts with `fixed`.
    /// Note that when using `percent`, rounding will be in favor of explicitly set `target_size` values; a managed instance group with 2 instances and 2 `version`s,
    /// one of which has a `target_size.percent` of `60` will create 2 instances of that `version`.
    #[builder(into, default)]
    #[serde(rename = "percent")]
    pub r#percent: Box<Option<i32>>,
}
