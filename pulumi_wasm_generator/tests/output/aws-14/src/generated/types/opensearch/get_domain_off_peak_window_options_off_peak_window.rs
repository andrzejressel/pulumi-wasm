#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDomainOffPeakWindowOptionsOffPeakWindow {
    /// 10h window for updates
    #[builder(into)]
    #[serde(rename = "windowStartTimes")]
    pub r#window_start_times: Box<Vec<super::super::types::opensearch::GetDomainOffPeakWindowOptionsOffPeakWindowWindowStartTime>>,
}
