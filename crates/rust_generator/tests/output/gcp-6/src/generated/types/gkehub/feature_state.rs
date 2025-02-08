#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FeatureState {
    /// (Output)
    /// Output only. The "running state" of the Feature in this Hub.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "states")]
    pub r#states: Box<Option<Vec<super::super::types::gkehub::FeatureStateState>>>,
}
