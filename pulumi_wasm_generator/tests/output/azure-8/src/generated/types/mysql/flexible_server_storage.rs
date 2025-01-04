#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlexibleServerStorage {
    /// Should Storage Auto Grow be enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "autoGrowEnabled")]
    pub r#auto_grow_enabled: Box<Option<bool>>,
    /// Should IOPS be scaled automatically? If `true`, `iops` can not be set. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "ioScalingEnabled")]
    pub r#io_scaling_enabled: Box<Option<bool>>,
    /// The storage IOPS for the MySQL Flexible Server. Possible values are between `360` and `20000`.
    #[builder(into, default)]
    #[serde(rename = "iops")]
    pub r#iops: Box<Option<i32>>,
    /// The max storage allowed for the MySQL Flexible Server. Possible values are between `20` and `16384`.
    /// 
    /// > **Note:** Decreasing `size_gb` forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: Box<Option<i32>>,
}
