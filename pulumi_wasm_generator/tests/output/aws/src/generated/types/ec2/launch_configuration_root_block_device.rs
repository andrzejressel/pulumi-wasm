#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LaunchConfigurationRootBlockDevice {
    #[builder(into, default)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "iops")]
    pub r#iops: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "throughput")]
    pub r#throughput: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Box<Option<String>>,
}