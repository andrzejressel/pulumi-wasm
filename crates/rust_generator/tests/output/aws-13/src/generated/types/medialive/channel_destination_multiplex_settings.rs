#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelDestinationMultiplexSettings {
    /// The ID of the Multiplex that the encoder is providing output to.
    #[builder(into)]
    #[serde(rename = "multiplexId")]
    pub r#multiplex_id: Box<String>,
    /// The program name of the Multiplex program that the encoder is providing output to.
    #[builder(into)]
    #[serde(rename = "programName")]
    pub r#program_name: Box<String>,
}
