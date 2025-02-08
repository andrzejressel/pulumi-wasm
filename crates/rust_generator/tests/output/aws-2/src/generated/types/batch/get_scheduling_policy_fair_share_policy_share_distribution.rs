#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetSchedulingPolicyFairSharePolicyShareDistribution {
    /// Fair share identifier or fair share identifier prefix. For more information, see [ShareAttributes](https://docs.aws.amazon.com/batch/latest/APIReference/API_ShareAttributes.html).
    #[builder(into)]
    #[serde(rename = "shareIdentifier")]
    pub r#share_identifier: Box<String>,
    /// Weight factor for the fair share identifier. For more information, see [ShareAttributes](https://docs.aws.amazon.com/batch/latest/APIReference/API_ShareAttributes.html).
    #[builder(into)]
    #[serde(rename = "weightFactor")]
    pub r#weight_factor: Box<f64>,
}
