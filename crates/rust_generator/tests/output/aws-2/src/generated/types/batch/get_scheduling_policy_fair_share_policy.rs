#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSchedulingPolicyFairSharePolicy {
    /// Value used to reserve some of the available maximum vCPU for fair share identifiers that have not yet been used. For more information, see [FairsharePolicy](https://docs.aws.amazon.com/batch/latest/APIReference/API_FairsharePolicy.html).
    #[builder(into)]
    #[serde(rename = "computeReservation")]
    pub r#compute_reservation: Box<i32>,
    #[builder(into)]
    #[serde(rename = "shareDecaySeconds")]
    pub r#share_decay_seconds: Box<i32>,
    /// One or more share distribution blocks which define the weights for the fair share identifiers for the fair share policy. For more information, see [FairsharePolicy](https://docs.aws.amazon.com/batch/latest/APIReference/API_FairsharePolicy.html). The `share_distribution` block is documented below.
    #[builder(into)]
    #[serde(rename = "shareDistributions")]
    pub r#share_distributions: Box<Vec<super::super::types::batch::GetSchedulingPolicyFairSharePolicyShareDistribution>>,
}
