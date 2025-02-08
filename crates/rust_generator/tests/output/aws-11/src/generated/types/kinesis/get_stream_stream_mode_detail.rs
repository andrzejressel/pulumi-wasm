#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetStreamStreamModeDetail {
    /// Capacity mode of the stream. Either `ON_DEMAND` or `PROVISIONED`.
    #[builder(into)]
    #[serde(rename = "streamMode")]
    pub r#stream_mode: Box<String>,
}
