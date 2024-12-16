#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub enum Farm {
    #[serde(rename = "Pulumi Planters Inc.")]
    PulumiPlantersInc,
    #[serde(rename = "Plants'R'Us")]
    PlantsRUs,
}
