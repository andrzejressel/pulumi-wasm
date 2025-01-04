#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UptimeCheckConfigSyntheticMonitor {
    /// Target a Synthetic Monitor GCFv2 Instance
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_cloud_function_v2"></a>The `cloud_function_v2` block supports:
    #[builder(into)]
    #[serde(rename = "cloudFunctionV2")]
    pub r#cloud_function_v_2: Box<super::super::types::monitoring::UptimeCheckConfigSyntheticMonitorCloudFunctionV2>,
}
