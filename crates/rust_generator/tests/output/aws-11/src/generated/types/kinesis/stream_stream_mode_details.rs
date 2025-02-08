#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamStreamModeDetails {
    /// Specifies the capacity mode of the stream. Must be either `PROVISIONED` or `ON_DEMAND`.
    #[builder(into)]
    #[serde(rename = "streamMode")]
    pub r#stream_mode: Box<String>,
}
