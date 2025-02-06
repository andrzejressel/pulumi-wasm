#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScraperDestination {
    /// Configuration block for an Amazon Managed Prometheus workspace destination. See `amp`.
    #[builder(into, default)]
    #[serde(rename = "amp")]
    pub r#amp: Box<Option<super::super::types::amp::ScraperDestinationAmp>>,
}
