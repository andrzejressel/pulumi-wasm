#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DomainOffPeakWindowOptionsOffPeakWindow {
    /// 10h window for updates
    #[builder(into, default)]
    #[serde(rename = "windowStartTime")]
    pub r#window_start_time: Box<Option<super::super::types::opensearch::DomainOffPeakWindowOptionsOffPeakWindowWindowStartTime>>,
}
