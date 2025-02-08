#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDomainOffPeakWindowOptionsOffPeakWindow {
    /// 10h window for updates
    #[builder(into)]
    #[serde(rename = "windowStartTimes")]
    pub r#window_start_times: Box<Vec<super::super::types::opensearch::GetDomainOffPeakWindowOptionsOffPeakWindowWindowStartTime>>,
}
