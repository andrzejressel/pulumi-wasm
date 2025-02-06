#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiFeatureOnlineStoreBigtable {
    /// Autoscaling config applied to Bigtable Instance.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "autoScaling")]
    pub r#auto_scaling: Box<super::super::types::vertex::AiFeatureOnlineStoreBigtableAutoScaling>,
}
