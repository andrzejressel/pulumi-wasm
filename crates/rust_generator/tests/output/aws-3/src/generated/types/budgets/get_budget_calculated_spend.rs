#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBudgetCalculatedSpend {
    #[builder(into)]
    #[serde(rename = "actualSpends")]
    pub r#actual_spends: Box<Vec<super::super::types::budgets::GetBudgetCalculatedSpendActualSpend>>,
}
