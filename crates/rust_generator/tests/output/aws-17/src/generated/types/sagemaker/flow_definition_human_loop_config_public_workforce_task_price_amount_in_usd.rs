#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowDefinitionHumanLoopConfigPublicWorkforceTaskPriceAmountInUsd {
    /// The fractional portion, in cents, of the amount. Valid value range between `0` and `99`.
    #[builder(into, default)]
    #[serde(rename = "cents")]
    pub r#cents: Box<Option<i32>>,
    /// The whole number of dollars in the amount. Valid value range between `0` and `2`.
    #[builder(into, default)]
    #[serde(rename = "dollars")]
    pub r#dollars: Box<Option<i32>>,
    /// Fractions of a cent, in tenths. Valid value range between `0` and `9`.
    #[builder(into, default)]
    #[serde(rename = "tenthFractionsOfACent")]
    pub r#tenth_fractions_of_a_cent: Box<Option<i32>>,
}
